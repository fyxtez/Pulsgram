mod utils;
use lazy_static::lazy_static;
use publisher::EventBus;
use regex::Regex;
use std::sync::Arc;
use telegram_types::{Client, Message, Peer};

pub async fn run(
    bus: Arc<EventBus>,
    target_dialog_id: i64,
    destination: Peer,
    dispatcher: Arc<Client>,
) {
    println!("KOL Follows running...");
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        let message = event.message;

        let peer_id = message.peer_id().bare_id();

        if !target_dialog_id.eq(&peer_id) {
            continue;
        }

        handle_follow(message, &dispatcher, &destination).await;
    }
}

pub async fn handle_follow(message: Message, dispatcher: &Client, destination: &Peer) {
    if !simple_is_followed_check(message.text()) {
        return;
    }

    let mut html_content = remove_emojis(&message.html_text());
    html_content = postprocess_html(&html_content);

    if message.text().contains("diloytte") {
        // println!("Ignoring diloytte...");
        // let input_message = telegram_types::InputMessage::new().html(&html_content);

        // let result = dispatcher
        //     .send_message(destination_test, input_message)
        //     .await;
        // if result.is_err() {
        //     dbg!(result.err());
        // }
        return;
    }

    if cfg!(feature = "production") {
        let input_message = telegram_types::InputMessage::new().html(&html_content);
        if let Err(err) = dispatcher.send_message(destination, input_message).await {
            eprintln!("Failed to send message: {:?}", err);
        }
    }
}

fn remove_emojis(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii() || (*c as u32) < 0x1F000)
        .collect()
}

fn simple_is_followed_check(message_text: &str) -> bool {
    let first_line = message_text.lines().next().unwrap_or("");

    first_line.contains("followed")
}

lazy_static! {
    // Compiled once at startup.
    // If these fail, it's a programmer error (invalid regex literal).
    static ref RE_NULL: Regex =
        Regex::new(r"(?i)\bnull\b").expect("Invalid RE_NULL regex");

    static ref RE_BLANK_LINES: Regex =
        Regex::new(r"\n[ \t]*\n").expect("Invalid RE_BLANK_LINES regex");

    static ref RE_BQ_CLOSE: Regex =
        Regex::new(r"[\s]+</blockquote>").expect("Invalid RE_BQ_CLOSE regex");

    static ref RE_BQ_OPEN: Regex =
        Regex::new(r"<blockquote>\s+").expect("Invalid RE_BQ_OPEN regex");
}

// TODO: Expensive method, needs redesign if Telegram rendering inconsistencies persist.
fn postprocess_html(html: &str) -> String {
    let mut result = html.to_string();

    // 1. Remove "null" appearing as location text (case-insensitive)
    result = RE_NULL.replace_all(&result, "").to_string();

    // 2. Collapse consecutive blank lines
    loop {
        let next = RE_BLANK_LINES.replace_all(&result, "\n").to_string();
        if next == result {
            break;
        }
        result = next;
    }

    // 3. Clean whitespace before </blockquote>
    result = RE_BQ_CLOSE
        .replace_all(&result, "</blockquote>")
        .to_string();

    // 4. Clean whitespace after <blockquote>
    result = RE_BQ_OPEN.replace_all(&result, "<blockquote>").to_string();

    result
}
