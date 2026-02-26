use publisher::types::{ErrorEvent, PulsgramEvent};
use publisher::{EventBus, handle_recv_error};
use std::sync::Arc;
use telegram_types::{Client, PeerRef};

pub async fn run(
    bus: Arc<EventBus>,
    client: Arc<Client>,
    fyxtez: PeerRef,
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

                    let message_text_lower = message.text().to_lowercase();

                    if !target_kols_lowercase
                        .iter()
                        .any(|kol| message_text_lower.contains(kol))
                    {
                        continue;
                    }

                    if let Err(error) = client
                        .forward_messages(perp_kols_peer, &[message.id()], fyxtez)
                        .await
                    {
                        let msg = format!(
                            "Perp KOL forward failed.\nFrom Target: {}\nTo Peer: {}\nError: {}",
                            from_target_id, perp_kols_peer.id, error
                        );

                        bus.publish(PulsgramEvent::Error(ErrorEvent {
                            message_text: msg,
                            source: "PerpKols::Forward",
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
