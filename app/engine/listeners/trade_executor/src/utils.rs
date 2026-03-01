use binance::{error::BinanceError, response_types::FuturesOrderResponse};
use domain::types::trade::TradeApproved;

pub enum OrderExecutionStatus {
    Filled {
        order_id: i64,
        qty: String,
        avg_price: String,
    },
    New,
    PartiallyFilled,
    Canceled,
    Rejected,
    Expired,
    Unknown(String),
}

impl From<&FuturesOrderResponse> for OrderExecutionStatus {
    fn from(response: &FuturesOrderResponse) -> Self {
        match response.status.as_str() {
            "NEW" => OrderExecutionStatus::New,
            "PARTIALLY_FILLED" => OrderExecutionStatus::PartiallyFilled,
            "FILLED" => OrderExecutionStatus::Filled {
                order_id: response.order_id,
                qty: response.executed_qty.clone(),
                avg_price: response.avg_price.clone(),
            },
            "CANCELED" => OrderExecutionStatus::Canceled,
            "REJECTED" => OrderExecutionStatus::Rejected,
            "EXPIRED" | "EXPIRED_IN_MATCH" => OrderExecutionStatus::Expired,
            other => OrderExecutionStatus::Unknown(other.to_string()),
        }
    }
}

pub fn handle_order_status(trade: &TradeApproved, status: OrderExecutionStatus) {
    match status {
        OrderExecutionStatus::New => {
            println!("[STATUS] Order accepted but not filled yet.");
        }

        OrderExecutionStatus::PartiallyFilled => {
            println!("[STATUS] Order partially filled.");
        }

        OrderExecutionStatus::Filled {
            order_id,
            qty,
            avg_price,
        } => {
            println!(
                "[FILLED] id={} order_id={} qty={} avg_price={}",
                trade.intent_id, order_id, qty, avg_price
            );
        }

        OrderExecutionStatus::Canceled => {
            println!("[STATUS] Order was canceled.");
        }

        OrderExecutionStatus::Rejected => {
            println!("[STATUS] Order was rejected.");
        }

        OrderExecutionStatus::Expired => {
            println!("[STATUS] Order expired.");
        }

        OrderExecutionStatus::Unknown(raw) => {
            println!("[STATUS] Unknown status received: {}", raw);
        }
    }
}

pub fn format_trade_error(trade: &TradeApproved, error: &BinanceError) -> String {
    match error {
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
                trade.intent_id, trade.symbol, trade.side, trade.entry, error
            )
        }
    }
}
