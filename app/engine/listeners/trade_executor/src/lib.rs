use binance::client::BinanceClient;
use binance::error::BinanceError;
use publisher::types::{ErrorEvent, PulsgramEvent};
use publisher::{EventBus, handle_recv_error};
use std::sync::Arc;

pub async fn run(bus: Arc<EventBus>, client: BinanceClient) {
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
    
                    match client
                        .place_minimum_market_order(trade.symbol, &trade.side)
                        .await
                    {
                        //TODO: Replace this work somewhere else.
                        //TODO: Create OrderExecutionStatusEnum
                        Ok(response) => match response.status.as_str() {
                            "NEW" => println!("[STATUS] Order accepted but not filled yet."),
                            "PARTIALLY_FILLED" => println!("[STATUS] Order partially filled."),
                            "FILLED" => println!(
                                "[FILLED] id={} order_id={} qty={} avg_price={}",
                                trade.intent_id,
                                response.order_id,
                                response.executed_qty,
                                response.avg_price
                            ),
                            "CANCELED" => println!("[STATUS] Order was canceled."),
                            "REJECTED" => println!("[STATUS] Order was rejected."),
                            "EXPIRED" => println!("[STATUS] Order expired."),
                            "EXPIRED_IN_MATCH" => {
                                println!("[STATUS] Order expired during matching.")
                            }
                            other => println!("[STATUS] Unknown status received: {}", other),
                        },
                        Err(error) => {
                            let message = match &error {
                                BinanceError::Api(api_err) => {
                                    format!(
                                        "TRADE EXECUTION FAILED\n\n\
                                        Trade ID: {}\n\
                                        Symbol: {}\n\
                                        Side: {}\n\
                                        Entry: {}\n\
                                        Stop Loss: {}\n\
                                        Timeframe: {}\n\n\
                                        Exchange Error:\n\
                                        Code: {}\n\
                                        Reason: {}",
                                        trade.intent_id,
                                        trade.symbol,
                                        trade.side,
                                        trade.entry,
                                        trade.stop_loss,
                                        trade.timeframe,
                                        api_err.code,
                                        api_err.msg
                                    )
                                }
                                _ => {
                                    format!(
                                        "⚠️ TRADE EXECUTION FAILED\n\n\
                                        Trade ID: {}\n\
                                        Symbol: {}\n\
                                        Side: {}\n\
                                        Entry: {}\n\n\
                                        System Error:\n\
                                        {}",
                                        trade.intent_id,
                                        trade.symbol,
                                        trade.side,
                                        trade.entry,
                                        error
                                    )
                                }
                            };

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
