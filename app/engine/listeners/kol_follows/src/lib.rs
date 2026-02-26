use publisher::types::{ErrorEvent, PulsgramEvent};
use publisher::{EventBus, handle_recv_error};
use shared::{postprocess_html, remove_emojis};
use std::sync::Arc;
use telegram::media::extract_photo_url_from_raw;
use telegram_types::PeerRef;
use telegram_types::{Client, Message};

pub async fn run(
    bus: Arc<EventBus>,
    target_dialog_id: i64,
    destination: PeerRef,
    dispatcher: Arc<Client>,
) {
    println!("KOL Follows running...");
    let mut rx = bus.subscribe();

    loop {
        match rx.recv().await {
            Ok(event) => match event {
                publisher::types::PulsgramEvent::Telegram(event) => {
                    let message = event.message;

                    let peer_id = message.peer_id().bare_id();

                    if peer_id != target_dialog_id {
                        continue;
                    }

                    handle_follow(message, &dispatcher, destination, &bus).await;
                }
                _ => continue,
            },
            Err(error) => {
                if handle_recv_error("KOL Follows RecvError", error, &bus) {
                    break;
                }
            }
        }
    }
}

pub async fn handle_follow(
    message: Box<Message>,
    dispatcher: &Client,
    destination: PeerRef,
    bus: &EventBus,
) {
    if !simple_is_followed_check(message.text()) {
        return;
    }

    let html_content = postprocess_html(&remove_emojis(&message.html_text()));

    let final_html = if let Some(photo_url) = extract_photo_url_from_raw(&message.raw) {
        format!("<a href=\"{}\">&#8205;</a>{}", photo_url, html_content)
    } else {
        html_content
    };

    // Special test keyword handling
    if message.text().contains("diloytte") {
        if cfg!(feature = "production") {
            return;
        }

        let input_message = telegram_types::InputMessage::new()
            .html(final_html)
            .link_preview(true)
            .invert_media(true);

        if let Err(error) = dispatcher.send_message(destination, input_message).await {
            let msg = format!(
                "KOL Follows failed (test mode).\nDestination: {}\nError: {}",
                destination.id, error
            );

            bus.publish(PulsgramEvent::Error(ErrorEvent {
                message_text: msg,
                source: "KOL Follows::SendMessage(Test)",
            }));
        }

        return;
    }

    if cfg!(feature = "production") {
        let input_message = telegram_types::InputMessage::new()
            .html(final_html)
            .link_preview(true)
            .invert_media(true);

        if let Err(error) = dispatcher.send_message(destination, input_message).await {
            let msg = format!(
                "KOL Follows failed.\nDestination: {}\nError: {}",
                destination.id, error
            );

            bus.publish(PulsgramEvent::Error(ErrorEvent {
                message_text: msg,
                source: "KOL Follows::SendMessage",
            }));
        }
    }
}

fn simple_is_followed_check(message_text: &str) -> bool {
    let first_line = message_text.lines().next().unwrap_or("");

    first_line.contains("followed")
}
