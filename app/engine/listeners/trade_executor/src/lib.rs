use publisher::types::PulsgramEvent;
use publisher::{EventBus, handle_recv_error};
use std::sync::Arc;

pub async fn run(bus: Arc<EventBus>) {
    println!("Trade Executor running...");
    let mut rx = bus.subscribe();

    loop {
        match rx.recv().await {
            Ok(event) => match event {
                PulsgramEvent::TradeApproved(trade) => {
                    println!(
                        "[APPROVED] id={} symbol={} side={}",
                        trade.intent_id, trade.symbol, trade.side,
                    );
                }

                PulsgramEvent::TradeRejected(trade) => {
                    println!(
                        "[REJECTED] id={} symbol={} reason={:?}",
                        trade.intent_id, trade.symbol, trade.reason,
                    );
                }

                _ => {}
            },

            Err(error) => {
                if handle_recv_error("PerpSignals RecvError", error, &bus) {
                    break;
                }
            }
        }
    }
}
