use std::sync::Arc;

use blockchains_address_extractor::extract_token_address_from_message_text;
use grammers_client::Client;
use publisher::EventBus;
use grammers_client::types::Peer;


pub async fn run(bus: Arc<EventBus>, client: Arc<Client>, forwarding_peer: Peer) {
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        let message = event.message;
        let sender = message.sender();

        println!("Does sender exist??? {:?}",sender.is_some());

        if sender.is_none() {
            continue;
        }

        let sender = sender.unwrap();

        let sender_name = sender.name().unwrap_or("NO SENDER NAME");
        let sender_username = sender.username().unwrap_or("NO SENDER UNSERNAME");

        let (address, blockchain) = extract_token_address_from_message_text(&message.text());
        if address.is_some() {
            let _ = client.send_message(&forwarding_peer, format!("Got CA: \n\n{}\n\n from name: {}\n username: {} \n Blockchain: {:?}", address.unwrap(),sender_name,sender_username,blockchain)).await;
        }
    }
}
