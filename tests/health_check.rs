fn spawn_app() {
    let server = pc_lib::run().expect("Can't get the server");

    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client.get("http://127.0.0.1:8080/health").send().await.expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}
