use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, PartialEq)]
pub struct TradingSignal {
    pub symbol: String,
    pub is_long: bool, // true = LONG, false = SHORT
    pub entry: f64,
    pub targets: Vec<f64>,
    pub timeframe: String,
    pub stop_loss: String,
}

struct SignalRegexes {
    symbol: Regex,
    direction: Regex,
    entry: Regex,
    targets: Regex,
    stop_loss: Regex,
    disclaimer: Regex,
    timeframe: Regex,
}

fn regexes() -> &'static SignalRegexes {
    static REGEXES: OnceLock<SignalRegexes> = OnceLock::new();
    REGEXES.get_or_init(|| SignalRegexes {
        symbol: Regex::new(r"([A-Z]+)USDT").unwrap(),
        timeframe: Regex::new(r"·\s*(\d+[hmdw])").unwrap(),
        direction: Regex::new(r"(LONG|SHORT)").unwrap(),
        entry: Regex::new(r"Entry:\s*([0-9]+\.?[0-9]*)").unwrap(),
        targets: Regex::new(r"TP[0-9]+:\s*([0-9]+\.?[0-9]*)").unwrap(),
        stop_loss: Regex::new(r"SL:\s*([0-9]+\.?[0-9]*)").unwrap(),
        disclaimer: Regex::new(r"(?i)disclaimer:.*").unwrap(),
    })
}

pub fn remove_emojis(input: &str) -> String {
    let re = Regex::new(r"[\p{Emoji_Presentation}\p{Extended_Pictographic}]").unwrap();
    re.replace_all(input, "").to_string()
}

fn is_valid_trading_signal(text: &str) -> bool {
    let re = regexes();

    let has_tp_targets = re.targets.is_match(text);
    let has_stop_loss = re.stop_loss.is_match(text);
    let is_status_message = text.contains("Target:") || text.contains("Now:");

    has_tp_targets && has_stop_loss && !is_status_message
}

pub fn format_signal(signal: &TradingSignal) -> String {
    let direction = if signal.is_long { "LONG" } else { "SHORT" };

    let entry_low = signal.entry * 0.99;
    let entry_high = signal.entry * 1.01;

    let tp3 = signal.targets.last().unwrap();
    let target_low = tp3 * 0.98;
    let target_high = tp3 * 1.02;

    format!(
        "<b>{} {}</b>\n<b>Timeframe:</b> {}\n<b>Entry:</b> {:.3}-{:.3}\n<b>Target:</b> {:.3}-{:.3}\n<b>Stop:</b> {}\n{}",
        signal.symbol,
        direction,
        signal.timeframe,
        entry_low,
        entry_high,
        target_low,
        target_high,
        format!("Optional/Personal"),
        format!(
            "----This scalp indicator is based on market trend structure and momentum.\n Always check charts before enterig a trade."
        )
    )
}

pub fn parse_trading_signal(text: &str) -> Option<TradingSignal> {
    if !is_valid_trading_signal(text) {
        return None;
    }

    let re = regexes();

    let cleaned_text = re.disclaimer.replace_all(text, "");

    let symbol = re
        .symbol
        .captures(&cleaned_text)?
        .get(1)?
        .as_str()
        .to_string();

    let direction_str = re.direction.captures(&cleaned_text)?.get(1)?.as_str();

    let is_long = match direction_str {
        "LONG" => true,
        "SHORT" => false,
        _ => return None,
    };

    let timeframe = re
        .timeframe
        .captures(&cleaned_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| "1h".to_string());

    let entry = re
        .entry
        .captures(&cleaned_text)?
        .get(1)?
        .as_str()
        .parse::<f64>()
        .ok()?;

    let targets: Vec<f64> = re
        .targets
        .captures_iter(&cleaned_text)
        .filter_map(|cap| cap.get(1)?.as_str().parse::<f64>().ok())
        .collect();

    if targets.is_empty() {
        return None;
    }

    let stop_loss = String::from("Optional");

    Some(TradingSignal {
        symbol,
        is_long,
        entry,
        targets,
        stop_loss,
        timeframe,
    })
}
