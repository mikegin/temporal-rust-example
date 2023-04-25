use serde_json::json;
use temporal_sdk_core::{ClientOptionsBuilder, ClientOptions, Url};
use temporal_client::{Client, WorkflowClientTrait, WorkflowOptions};
use uuid::Uuid;
use std::{
  convert::TryFrom, env, future::Future, net::SocketAddr, path::PathBuf, sync::Arc,
  time::Duration, collections::HashMap,
};
use temporal_sdk_core_protos::{
  coresdk::{workflow_commands::QueryResult, IntoPayloadsExt, AsJsonPayloadExt, FromJsonPayloadExt},
  grpc::health::v1::health_client::HealthClient,
  temporal::api::{
      common::v1::{Header, Payload, Payloads, WorkflowExecution, WorkflowType},
      enums::v1::{TaskQueueKind, WorkflowIdReusePolicy, WorkflowTaskFailedCause},
      failure::v1::Failure,
      operatorservice::v1::operator_service_client::OperatorServiceClient,
      query::v1::WorkflowQuery,
      replication::v1::ClusterReplicationConfig,
      taskqueue::v1::TaskQueue,
      testservice::v1::test_service_client::TestServiceClient,
      workflowservice::v1::{workflow_service_client::WorkflowServiceClient, *},
  },
  TaskToken,
};

pub const INTEG_SERVER_TARGET_ENV_VAR: &str = "TEMPORAL_SERVICE_ADDRESS";

fn get_integ_server_options() -> ClientOptions {
  let temporal_server_address = match env::var(INTEG_SERVER_TARGET_ENV_VAR) {
      Ok(addr) => addr,
      Err(_) => "http://localhost:7233".to_owned(),
  };
  let url = Url::try_from(&*temporal_server_address).unwrap();
  ClientOptionsBuilder::default()
      .identity("integ_tester".to_string())
      .target_url(url)
      .client_name("temporal-core".to_string())
      .client_version("0.1.0".to_string())
      .build()
      .unwrap()
}


pub async fn execute_workflow() {
  let client = Arc::new(
    get_integ_server_options()
        .connect("default", None, None)
        .await
        .expect("Must connect"),
  );

  let wf_id = format!("{}{}", "wf", Uuid::new_v4().to_string().as_str().to_owned());

  let mut metadata = HashMap::new();
  metadata.insert("encoding".to_string(), b"json/plain".to_vec());

  let json_arg1 = json!({
    "some": {
      "nested": {
        "json": "value"
      }
  }});


  let arg1 = Payload {
    metadata: metadata.clone(),
    data: serde_json::to_vec(&json_arg1).ok().unwrap(),
  };

  let arg2 = Payload {
    metadata: metadata.clone(),
    data: serde_json::to_vec(&json!("some string")).ok().unwrap(),
  };

  let arg3 = Payload {
    metadata: metadata.clone(),
    data: serde_json::to_vec(&json!("some string")).ok().unwrap(),
  };

  let arg4 = Payload {
    metadata: metadata.clone(),
    data: serde_json::to_vec(&json!({
      "accept": "application/json, text/plain, */*",
      "content-type": "application/json",
      "user-agent": "axios/0.26.1",
      "content-length": "54",
      "host": "localhost:3000",
      "connection": "close"
    })).ok().unwrap(),
  };

  let arg5 = Payload {
    metadata: metadata.clone(),
    data: serde_json::to_vec(&json!({
      "name": "John Doe"
    })).ok().unwrap(),
  };

  let arg6 = Payload {
    metadata: metadata.clone(),
    data: serde_json::to_vec(&json!("some string")).ok().unwrap(),
  };

//   let tags = json!([
//     "value1",
//     "other value",
//     "third",
//     "4"
//   ]);

  client.get_client().start_workflow(vec![arg1, arg2, arg3, arg4, arg5, arg6], (&"task_queue1").to_string(), wf_id, (&"workflow1").to_string(), None,  WorkflowOptions {
//     search_attributes: Some(HashMap::from([
//       ("tags".to_string(), Payload {
//         metadata,
//         data: serde_json::to_vec(&tags).ok().unwrap(),
//       }),
//   ])),
    ..Default::default()
},).await.unwrap();
}