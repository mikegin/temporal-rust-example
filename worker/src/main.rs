use std::time::Duration;
use std::{str::FromStr, sync::Arc, collections::HashMap};
use temporal_sdk::{sdk_client_options, ActContext, Worker, WfContext, ActivityOptions};
use temporal_sdk_core::{init_worker, Url, CoreRuntime};
use temporal_sdk_core_api::{worker::WorkerConfigBuilder, telemetry::TelemetryOptionsBuilder};
use temporal_sdk_core_protos::temporal::api::common::v1::Payload;
use temporal_sdk_core_protos::coresdk::workflow_commands::ActivityCancellationType;
use temporal_sdk_core_protos::coresdk::activity_result::ActivityResolution;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;

    let client = server_options.connect("default", None, None).await?;

    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;

    let task_queue = "task_queue1".to_string();

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue(&task_queue)
        .worker_build_id("123")
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), &task_queue);

    worker.register_wf("workflow1", |_ctx: WfContext| async move { 
        println!("Hello from workflow1");

        let task_queue = "task_queue1".to_string();

        let mut metadata = HashMap::new();
    
        metadata.insert("encoding".to_string(), b"json/plain".to_vec());
        let activity_payload = Payload {
            metadata: metadata.clone(),
            data: serde_json::to_vec("Hello World! 123").unwrap(),
        };

        let result: ActivityResolution = _ctx.activity(ActivityOptions {
            activity_id: None,
            activity_type: "echo_activity".to_string(),
            input: activity_payload,
            task_queue,
            schedule_to_start_timeout: None,
            start_to_close_timeout: Some(Duration::from_secs(10)),
            schedule_to_close_timeout: None,
            heartbeat_timeout: None,
            cancellation_type: ActivityCancellationType::TryCancel,
            retry_policy: None,
        }).await.into();

        if result.completed_ok() {
            let result: Payload = result.unwrap_ok_payload();
            println!("Activity result: {:?}", result);
            let body = serde_json::from_str::<String>(&String::from_utf8(result.data).unwrap()[..])?;
            println!("Activity result body: {:?}", body);
            return Ok(temporal_sdk::WfExitValue::Normal(()));
        }

        Ok(temporal_sdk::WfExitValue::Cancelled)
     });

    worker.register_activity(
        "echo_activity",
        |_ctx: ActContext, echo_me: String| async move { 
            println!("Hello from echo_activity");
            return Ok(echo_me);
         },
    );

    worker.run().await?;

    Ok(())
}