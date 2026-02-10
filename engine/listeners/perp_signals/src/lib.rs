mod regex;

use publisher::EventBus;
use std::sync::Arc;
use telegram_types::{Client, Peer};

use crate::regex::{parse_trading_signal, remove_emojis};

pub async fn run(bus: Arc<EventBus>, client_dispatcher: Arc<Client>, target_id: i64, signals: Peer) {
    println!("Perp Signals running...");
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        let message = event.message;

        let message_peer_id = message.peer_id();

        if !message_peer_id.bare_id().eq(&target_id) {
            continue;
        }

        let message_text = message.text();

        let result = parse_trading_signal(message_text);

        if result.is_none() {
            continue;
        }

        let result = result.unwrap();

        let _symbol = result.symbol;

        let result = client_dispatcher
            .send_message(&signals, remove_emojis(message.text()))
            .await;

        if result.is_err() {
            println!("{:?}",message.text());
            println!("{:?}",result.err());
        }
    }
}
