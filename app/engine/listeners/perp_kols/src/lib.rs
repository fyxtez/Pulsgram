use publisher::types::{ErrorEvent, PulsgramEvent};
use publisher::{EventBus, handle_recv_error};
use shared::{postprocess_html, remove_emojis};
use std::sync::Arc;
use telegram::media::extract_photo_url_from_raw;
use telegram_types::{Client, PeerRef};

pub async fn run(
    bus: Arc<EventBus>,
    dispatcher: Arc<Client>,
    from_target_id: i64,
    perp_kols_peer: PeerRef,
    target_kols: Vec<String>,
) {
    println!("KOL Perp Signals running...");

    let mut rx = bus.subscribe();

    let target_kols_lowercase: Vec<String> = target_kols
        .into_iter()
        .map(|kol| kol.to_lowercase())
        .collect();

    loop {
        match rx.recv().await {
            Ok(event) => {
                if let PulsgramEvent::Telegram(tg_event) = event {
                    let message = tg_event.message;

                    if message.peer_id().bare_id() != from_target_id {
                        continue;
                    }

                    let message_text = message.text();
                    let message_text_lower = message_text.to_lowercase();

                    if !target_kols_lowercase
                        .iter()
                        .any(|kol| message_text_lower.contains(kol))
                    {
                        continue;
                    }

                    let html_content = postprocess_html(&remove_emojis(&message.html_text()));
                    let final_html =
                        if let Some(photo_url) = extract_photo_url_from_raw(&message.raw) {
                            format!("<a href=\"{}\">&#8205;</a>{}", photo_url, html_content)
                        } else {
                            html_content
                        };
                    let input_message = telegram_types::InputMessage::new()
                        .html(final_html)
                        .link_preview(true)
                        .invert_media(true);

                    if let Err(error) = dispatcher.send_message(perp_kols_peer, input_message).await
                    {
                        let msg = format!(
                            "Perp KOL send failed.\nTo Peer: {}\nError: {}",
                            perp_kols_peer.id, error
                        );

                        bus.publish(PulsgramEvent::Error(ErrorEvent {
                            message_text: msg,
                            source: "PerpKols::SendMessage",
                        }));
                    }
                }
            }

            Err(error) => {
                if handle_recv_error("PerpKols RecvError", error, &bus) {
                    break;
                }
            }
        }
    }
}
