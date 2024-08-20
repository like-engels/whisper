pub fn handle(message: String, title: String, authorization_token: String) {
    let http_client = reqwest::blocking::Client::new();

    let request_body = serde_json::json!({
        "notification": message,
        "accessCode": authorization_token,
        "title": title
    });

    let response = http_client
        .post("https://api.notifymyecho.com/v1/NotifyMe")
        .json(&request_body)
        .send();

    match response {
        Err(error) => {
            println!(
                "Something went wrong while sending NotifyMe message: {}",
                error.to_string()
            )
        }
        Ok(resp) => {
            let status_code = resp.status();

            if status_code.is_success() {
                println!("Notification sent!");
            } else {
                let error_message = resp.text();
                println!(
                    "Something went wrong with NotifyMe side: {}",
                    error_message.unwrap()
                );
            }
        }
    }
}
