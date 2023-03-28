use std::net::TcpListener;

#[tokio::test]
async fn requests_without_apikeys_are_rejected_for_login() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/login", address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(401, response.status().as_u16());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to find random port.");
    let port = listener.local_addr().unwrap().port();
    let server = rust_hub::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
