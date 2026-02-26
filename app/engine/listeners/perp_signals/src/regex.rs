use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, PartialEq)]
pub struct TradingSignal {
    pub symbol: String,
    pub is_long: bool, // true = LONG, false = SHORT
    pub entry: f64,
    pub targets: Vec<f64>,
    pub timeframe: String,
    pub stop_loss: f64,
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
        symbol: Regex::new(r"\b([A-Z]+)USDT\b").expect("Invalid regex: symbol"),

        timeframe: Regex::new(r"·\s*(\d+[hmdw])").expect("Invalid regex: timeframe"),

        direction: Regex::new(r"(LONG|SHORT)").expect("Invalid regex: direction"),

        entry: Regex::new(r"Entry:\s*([0-9]+\.?[0-9]*)").expect("Invalid regex: entry"),

        targets: Regex::new(r"TP[0-9]+:\s*([0-9]+\.?[0-9]*)").expect("Invalid regex: targets"),

        stop_loss: Regex::new(r"SL:\s*([0-9]+\.?[0-9]*)").expect("Invalid regex: stop_loss"),

        disclaimer: Regex::new(r"(?i)disclaimer:.*").expect("Invalid regex: disclaimer"),
    })
}

fn emoji_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| {
        Regex::new(r"[\p{Emoji_Presentation}\p{Extended_Pictographic}]")
            .expect("Invalid emoji regex")
    })
}

pub fn remove_emojis(input: &str) -> std::borrow::Cow<'_, str> {
    emoji_regex().replace_all(input, "")
}

pub fn format_signal(signal: &TradingSignal) -> String {
    let direction = if signal.is_long { "LONG" } else { "SHORT" };

    let entry_low = signal.entry;
    let entry_high = signal.entry;

    let Some(tp3) = signal.targets.last() else {
        return String::from("Invalid signal: missing targets");
    };

    let target_low = tp3;
    let target_high = tp3;

    let stop_loss = signal.stop_loss;

    format!(
        "<b>{} {}</b>\n<b>Timeframe:</b> {}\n<b>Entry:</b> {:.3}-{:.3}\n<b>Target:</b> {:.3}-{:.3}\n<b>Stop:</b> {}\n",
        signal.symbol,
        direction,
        signal.timeframe,
        entry_low,
        entry_high,
        target_low,
        target_high,
        stop_loss,
    )
}

pub fn parse_trading_signal(text: &str) -> Option<TradingSignal> {
    let re = regexes();

    let cleaned_text = re.disclaimer.replace_all(text, "");

    let symbol = re
        .symbol
        .captures(&cleaned_text)?
        .get(1)?
        .as_str()
        .to_string();

    let direction_str = re.direction.captures(&cleaned_text)?.get(1)?.as_str();

    let is_long = direction_str == "LONG";

    let timeframe = re
        .timeframe
        .captures(&cleaned_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or("1h".into());

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

    let stop_loss = re
        .stop_loss
        .captures(&cleaned_text)?
        .get(1)?
        .as_str()
        .parse::<f64>()
        .ok()?;

    if targets.is_empty() {
        return None;
    }

    Some(TradingSignal {
        symbol,
        is_long,
        entry,
        targets,
        stop_loss,
        timeframe,
    })
}
