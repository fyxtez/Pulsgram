mod utils;

use binance::client::BinanceClient;
use publisher::types::{ErrorEvent, PulsgramEvent};
use publisher::{EventBus, handle_recv_error};
use std::sync::Arc;

use crate::utils::{OrderExecutionStatus, format_trade_error, handle_order_status};

pub async fn run(bus: Arc<EventBus>, client: BinanceClient) {
    println!("Trade Executor running...");
    let mut rx = bus.subscribe();

    loop {
        match rx.recv().await {
            Ok(event) => match event {
                PulsgramEvent::TradeApproved(trade) => {
                    match client
                        .place_minimum_market_order(trade.symbol, &trade.side)
                        .await
                    {
                        Ok(response) => {
                            let status = OrderExecutionStatus::from(&response);
                            handle_order_status(&trade, status);
                        }

                        Err(error) => {
                            let message = format_trade_error(&trade, &error);
                            bus.publish(PulsgramEvent::Error(ErrorEvent {
                                source: "TradeExecutor",
                                message_text: message,
                            }));
                        }
                    }
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
