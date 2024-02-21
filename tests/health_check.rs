mod test_utils;

use test_utils::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();

    dbg!(&addr);
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", &addr))
        .send()
        .await
        .expect("Failed to send request");

    dbg!(&response);
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}
