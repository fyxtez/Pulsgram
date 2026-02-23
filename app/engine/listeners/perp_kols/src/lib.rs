use publisher::EventBus;
use std::sync::Arc;
use telegram_types::{Client, Peer};

pub async fn run(
    bus: Arc<EventBus>,
    client: Arc<Client>,
    fyxtez: Peer,
    from_target_id: i64,
    perp_kols_peer: Peer,
    target_kols: Vec<String>,
) {
    println!("KOL Perp Signals running...");

    let mut rx = bus.subscribe();

    let target_kols_lowercase: Vec<String> =
        target_kols.into_iter().map(|kol| kol.to_lowercase()).collect();

    while let Ok(event) = rx.recv().await {
        let message = event.message;

        let message_peer_id = message.peer_id();

        if message_peer_id.bare_id() != from_target_id {
            continue;
        }

        let message_text = message.text();
        let message_text_lower = message_text.to_lowercase(); // ✅ once per message
        
        // Check if any target KOL is mentioned
        if !target_kols_lowercase
            .iter()
            .any(|kol| message_text_lower.contains(kol))
        {
            continue;
        }

        match client
            .forward_messages(&perp_kols_peer, &[message.id()], &fyxtez)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", message.text());
                println!("{:?}", err);
            }
        }
    }
}
