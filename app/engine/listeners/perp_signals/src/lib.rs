mod regex;

use publisher::EventBus;
use std::sync::Arc;
use telegram_types::PeerRef;
use telegram_types::{Client, InputMessage};

use crate::regex::{format_signal, parse_trading_signal, remove_emojis};

pub async fn run(
    bus: Arc<EventBus>,
    client_dispatcher: Arc<Client>,
    target_id: i64,
    signals: PeerRef,
) {
    println!("Perp Signals running...");
    let mut rx = bus.subscribe();

    loop {
        match rx.recv().await {
            Ok(event) => match event {
                publisher::types::PulsgramEvent::Telegram(event) => {
                    let message = event.message;

                    let message_peer_id = message.peer_id();

                    if message_peer_id.bare_id() != target_id {
                        continue;
                    }

                    let message_cleaned_up = remove_emojis(message.text());

                    let Some(signal) = parse_trading_signal(&message_cleaned_up) else {
                        continue;
                    };

                    let formatted_signal = format_signal(&signal);

                    let input_message = InputMessage::new().html(formatted_signal);

                    // TODO: Ovde publishaj novi event proveri broadcast i proemni ga da ima drugicje event
                    // novi listener ce da slusa na taj novi event i radi sta trijeba.
                    match client_dispatcher.send_message(signals, input_message).await {
                        Ok(_) => {}
                        Err(err) => {
                            println!("{:?}", message.text());
                            println!("{:?}", err);
                        }
                    }
                }
                _ => continue,
            },
            Err(error) => {
                println!("Error receiving event: {:?}", error);
                continue;
            }
        }
    }
}
