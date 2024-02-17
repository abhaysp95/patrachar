use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind the port");
    let addr = listener.local_addr().unwrap();
    let server = pc_lib::run(listener).expect("Can't get the server");

    let _ = tokio::spawn(server);
    format!("http://{}", addr)
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();

    dbg!(&addr);
    let client = reqwest::Client::new();
    let response = client.get(format!("{}/health", &addr)).send().await.expect("Failed to send request");

    dbg!(&response);
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}
