use std::collections::HashSet;
use std::sync::Arc;

use blockchains_address_extractor::extract_token_address_from_message_text;
use publisher::EventBus;
use telegram_types::Client;
use telegram_types::Peer;

// TODO: Ignored senders implementation
pub async fn run(
    bus: Arc<EventBus>,
    client: Arc<Client>,
    forwarding_peer: Peer,
    _ignored_senders: HashSet<&'static str>,
    _ignored_peers: HashSet<&Peer>,
) {
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        let message = event.message;
        let sender = message.sender();

        println!("Does sender exist??? {:?}", sender.is_some());

        if sender.is_none() {
            continue;
        }

        let sender = sender.unwrap();

        let sender_name = sender.name().unwrap_or("NO SENDER NAME");
        let sender_username = sender.username().unwrap_or("NO SENDER UNSERNAME");

        let (address, blockchain) = extract_token_address_from_message_text(message.text());
        if let Some(address) = address {
            let _ = client
                .send_message(
                    &forwarding_peer,
                    format!(
                        "Got CA: \n\n{}\n\n from name: {}\n username: {} \n Blockchain: {:?}",
                        address,
                        sender_name,
                        sender_username,
                        blockchain
                    ),
                )
                .await;
        }
    }
}
