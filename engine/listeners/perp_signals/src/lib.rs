mod regex;

use publisher::EventBus;
use std::sync::Arc;
use telegram_types::{Client, Peer};

use crate::regex::{format_signal, parse_trading_signal, remove_emojis};

pub async fn run(
    bus: Arc<EventBus>,
    client_dispatcher: Arc<Client>,
    target_id: i64,
    signals: Peer,
) {
    println!("Perp Signals running...");
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        let message = event.message;

        let message_peer_id = message.peer_id();

        if !message_peer_id.bare_id().eq(&target_id) {
            continue;
        }

        let message_text = message.text();
        
        let message_cleaned_up = remove_emojis(message_text);

        let result = parse_trading_signal(&message_cleaned_up);

        if result.is_none() {
            continue;
        }
        let signal = result.unwrap();
        let formatted_signal = format_signal(&signal);

        match client_dispatcher
            .send_message(&signals, &formatted_signal)
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
