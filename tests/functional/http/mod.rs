use {
    crate::{context::ServerContext, utils::send_jsonrpc_request, JSONRPC_VERSION},
    hyper::{Body, Client, Method, Request, StatusCode},
    hyper_tls::HttpsConnector,
    rpc_proxy::handlers::history::HistoryResponseBody,
    test_context::test_context,
};

pub(crate) mod base;
pub(crate) mod binance;
pub(crate) mod infura;
pub(crate) mod pokt;
pub(crate) mod zksync;
pub(crate) mod zora;

async fn check_if_rpc_is_responding_correctly_for_supported_chain(
    ctx: &ServerContext,
    chaind_id: &str,
    expected_id: &str,
) {
    let addr = format!(
        "{}/v1/?projectId={}&chainId=",
        ctx.server.public_addr, ctx.server.project_id
    );

    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
    let request = jsonrpc::Request {
        method: "eth_chainId",
        params: &[],
        id: serde_json::Value::Number(1.into()),
        jsonrpc: JSONRPC_VERSION,
    };

    let (status, rpc_response) = send_jsonrpc_request(client, addr, chaind_id, request).await;

    match status {
        StatusCode::BAD_GATEWAY => {}
        StatusCode::OK => {
            // Verify there was no error in rpc
            assert!(rpc_response.error.is_none());

            // Check chainId
            assert_eq!(rpc_response.result::<String>().unwrap(), expected_id)
        }
        _ => panic!("Unexpected status code: {}", status),
    };
}

#[test_context(ServerContext)]
#[tokio::test]
async fn health_check(ctx: &mut ServerContext) {
    let addr = format!("{}/health", ctx.server.public_addr);

    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    let request = Request::builder()
        .method(Method::GET)
        .uri(addr)
        .body(Body::default())
        .unwrap();

    let response = client.request(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK)
}

#[test_context(ServerContext)]
#[tokio::test]
async fn account_history_check(ctx: &mut ServerContext) {
    let account = "0xf3ea39310011333095CFCcCc7c4Ad74034CABA63";
    let project_id = ctx.server.project_id.clone();
    let addr = format!(
        "{}/v1/account/{}/history?projectId={}",
        ctx.server.public_addr, account, project_id
    );

    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    let request = Request::builder()
        .method(Method::GET)
        .uri(addr)
        .body(Body::default())
        .unwrap();

    let response = client.request(request).await.unwrap();
    let status = response.status();
    assert_eq!(status, StatusCode::OK);

    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = String::from_utf8_lossy(&bytes);

    let json_response: HistoryResponseBody =
        serde_json::from_str(&body_str).expect("Failed to parse response body");
    assert!(!json_response.data.is_empty());
}
