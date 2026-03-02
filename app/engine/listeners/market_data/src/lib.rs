use futures_util::StreamExt;
use serde::Deserialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Debug, Deserialize)]
struct StreamWrapper {
    data: TickerData,
}

#[derive(Debug, Deserialize)]
struct TickerData {
    #[serde(rename = "s")]
    symbol: String,

    #[serde(rename = "c")]
    last_price: String,
}

pub async fn run(symbols: Vec<domain::types::symbol::Symbol>) {
    let streams = symbols
        .iter()
        .map(|s| format!("{}@ticker", s.to_string().to_lowercase()))
        .collect::<Vec<_>>()
        .join("/");

    let url = format!("wss://fstream.binance.com/stream?streams={}", streams);

    println!("Connecting to {}", url);

    let (ws_stream, _) = match connect_async(&url).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("WS connect error: {}", e);
            return; // just end the task
        }
    };

    let (_write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                eprintln!("WS read error: {}", e);
                break;
            }
        };

        match msg {
            Message::Text(txt) => match serde_json::from_str::<StreamWrapper>(&txt) {
                Ok(parsed) => {
                    if let Ok(symbol) = parsed.data.symbol.parse::<domain::types::symbol::Symbol>()
                    {
                        println!("{:?} => {}", symbol, parsed.data.last_price);
                    }
                }
                Err(e) => eprintln!("JSON parse error: {}", e),
            },
            Message::Ping(_) | Message::Pong(_) => {}
            Message::Close(frame) => {
                println!("WS closed: {:?}", frame);
                break;
            }
            _ => {}
        }
    }
}
