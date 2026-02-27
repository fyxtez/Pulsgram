mod regex;

use domain::{TradeApproved, TradeIntent};
use publisher::types::{ErrorEvent, PulsgramEvent};
use publisher::{EventBus, handle_recv_error};
use std::sync::Arc;
use telegram_types::{Client, InputMessage, PeerRef};

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
            Ok(event) => {
                if let PulsgramEvent::Telegram(event) = event {
                    let message = event.message;

                    if message.peer_id().bare_id() != target_id {
                        continue;
                    }

                    let message_cleaned_up = remove_emojis(message.text());

                    let Some(signal) = parse_trading_signal(&message_cleaned_up) else {
                        continue;
                    };

                    let symbol = signal.symbol.clone();

                    let intent = TradeIntent::new(symbol.clone(), signal.is_long.into());

                    let approved = TradeApproved {
                        intent_id: intent.intent_id,
                        symbol,
                        side: intent.side,
                    };

                    let formatted_signal = format_signal(&signal);

                    bus.publish(PulsgramEvent::TradeApproved(approved));

                    let input_message = InputMessage::new().html(formatted_signal);

                    if let Err(error) = client_dispatcher.send_message(signals, input_message).await
                    {
                        let msg = format!(
                            "Perp Signals failed.\nTarget: {}\nSignals Peer: {}\nError: {}",
                            target_id, signals.id, error
                        );

                        bus.publish(PulsgramEvent::Error(ErrorEvent {
                            message_text: msg,
                            source: "PerpSignals::SendMessage",
                        }));
                    }
                }
            }

            Err(error) => {
                if handle_recv_error("PerpSignals RecvError", error, &bus) {
                    break;
                }
            }
        }
    }
}
