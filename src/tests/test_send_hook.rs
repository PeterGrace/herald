extern crate httpmock;

use httpmock::Method::{POST};
use httpmock::{
    Mock,
    MockServer
};
use crate::watching::send_hook::send_hook;

#[tokio::test]
async fn test_send_hook() {
    let mock_server = MockServer::start();
    let search_mock = Mock::new()
        .expect_method(POST)
        .expect_path("/mockpost")
        .return_status(200)
        .create_on(&mock_server);

    let response = send_hook(mock_server.url("/mockpost").to_string(), String::from("POST"), String::from("{}")).await.unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(search_mock.times_called(), 1);
}