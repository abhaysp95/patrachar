mod test_utils;

use test_utils::spawn_app;

#[tokio::test]
async fn subscribe_200_for_vaild_form_data() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=John%20Doe&email=johndoe%40xyz.com";
    let response = client
        .put(format!("{}/subscription", addr))
        .body(body)
        .header("Content-Type", "x-www-form-urlencoded")
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscribe_400_when_data_is_missing() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let bodies = vec![
        ("name=&email=johndoe%40xyz.com", "name not found"),
        ("name=John%20Doe&email=", "email not found"),
        ("name=&email=", "name and email not found"),
    ];

    for (body, err_msg) in bodies {
        let response = client
            .put(format!("{}/subscription", addr))
            .body(body)
            .header("Content-Type", "x-www-form-urlencoded")
            .send()
            .await
            .expect("Failed to send request");
        assert_eq!(
            response.status().as_u16(),
            400,
            // additional error message on test failure
            "API failed with 400 when the payload was {}",
            err_msg
        );
    }
}
