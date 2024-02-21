use std::net::TcpListener;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind the port");
    let addr = listener.local_addr().unwrap();
    let server = pc_lib::run(listener).expect("Can't get the server");

    let _ = tokio::spawn(server);
    format!("http://{}", addr)
}
