use publisher::EventBus;
use std::sync::Arc;
use telegram_types::Client;
use telegram_types::PeerRef;

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
            Ok(event) => match event {
                publisher::types::PulsgramEvent::Telegram(tg_event) => {
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

                    if let Err(err) = client
                        .forward_messages(perp_kols_peer, &[message.id()], fyxtez)
                        .await
                    {
                        println!("{:?}", message.text());
                        println!("{:?}", err);
                    }
                }
                _ => {
                    continue;
                }
            },
            Err(error) => {
                println!("Error receiving event: {:?}", error);
                continue;
            }
        }
    }
}
