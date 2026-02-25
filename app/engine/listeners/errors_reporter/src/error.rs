use telegram_types::{Client, PeerRef};

pub async fn report_error(
    client: &Client,
    error_peer: PeerRef,
    source: &'static str,
    error: &str,
) {
    let formatted = format!("⚠ Pulsgram Error\n\nSource: {}\n\nError: {}", source, error);

    if let Err(send_err) = client.send_message(error_peer, formatted).await {
        println!(
            "[ERROR_REPORTER_FAILURE]\nSource: {}\nOriginal: {}\nTelegram error: {:?}",
            source, error, send_err
        );
    }
}
