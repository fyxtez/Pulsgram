use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, PartialEq)]
pub struct TradingSignal {
    pub symbol: String,
    pub is_long: bool, // true = LONG, false = SHORT
    pub entry: f64,
    pub targets: Vec<f64>,
    pub stop_loss: f64,
}

struct SignalRegexes {
    symbol: Regex,
    direction: Regex,
    entry: Regex,
    targets: Regex,
    stop_loss: Regex,
    disclaimer: Regex,
}

fn regexes() -> &'static SignalRegexes {
    static REGEXES: OnceLock<SignalRegexes> = OnceLock::new();
    REGEXES.get_or_init(|| SignalRegexes {
        symbol: Regex::new(r"([A-Z]+)USDT").unwrap(),
        direction: Regex::new(r"(LONG|SHORT)").unwrap(),
        entry: Regex::new(r"Entry:\s*([0-9]+\.?[0-9]*)").unwrap(),
        targets: Regex::new(r"TP[0-9]+:\s*([0-9]+\.?[0-9]*)").unwrap(),
        stop_loss: Regex::new(r"SL:\s*([0-9]+\.?[0-9]*)").unwrap(),
        disclaimer: Regex::new(r"(?i)disclaimer:.*").unwrap(),
    })
}
static EMOJI_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn remove_emojis(s: &str) -> String {
    let re = EMOJI_REGEX.get_or_init(|| {
        Regex::new(r"[\p{Emoji_Presentation}\p{Emoji}\u{200d}\u{fe0f}]").unwrap()
    });

    re.replace_all(s, "").to_string()
}

fn is_valid_trading_signal(text: &str) -> bool {
    let re = regexes();

    let has_tp_targets = re.targets.is_match(text);
    let has_stop_loss = re.stop_loss.is_match(text);
    let is_status_message = text.contains("Target:") || text.contains("Now:");

    has_tp_targets && has_stop_loss && !is_status_message
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

    let stop_loss = re
        .stop_loss
        .captures(&cleaned_text)?
        .get(1)?
        .as_str()
        .parse::<f64>()
        .ok()?;

    Some(TradingSignal {
        symbol,
        is_long,
        entry,
        targets,
        stop_loss,
    })
}


