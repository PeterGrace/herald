extern crate httpmock;

use crate::watching::send_hook::send_hook;
use httpmock::Method::{GET, POST};
use httpmock::{Mock, MockServer};

#[tokio::test]
async fn test_send_hook() {
    let mock_server = MockServer::start();
    let search_mock = Mock::new()
        .expect_method(POST)
        .expect_path("/mockpost")
        .return_status(200)
        .create_on(&mock_server);

    let response = send_hook(
        mock_server.url("/mockpost").to_string(),
        String::from("POST"),
        String::from("{}"),
    )
    .await
    .unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(search_mock.times_called(), 1);
}
#[tokio::test]
async fn test_send_get_hook() {
    let mock_server = MockServer::start();

    let search_mock = Mock::new()
        .expect_method(GET)
        .expect_path("/mockpost")
        .return_status(200)
        .create_on(&mock_server);

    let response = send_hook(
        mock_server.url("/mockpost").to_string(),
        String::from("GET"),
        String::from("{}"),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(search_mock.times_called(), 1);
}
