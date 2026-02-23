mod utils;
use lazy_static::lazy_static;
use publisher::EventBus;
use regex::Regex;
use std::sync::{Arc, OnceLock};
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

        if peer_id != target_dialog_id {
            continue;
        }

        handle_follow(message, &dispatcher, &destination).await;
    }
}

pub async fn handle_follow(message: Message, dispatcher: &Client, destination: &Peer) {
    if !simple_is_followed_check(message.text()) {
        return;
    }

    let html_content = postprocess_html(&remove_emojis(&message.html_text()));

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

fn postprocess_html(html: &str) -> String {
    RE_BQ_OPEN
        .replace_all(
            &RE_BQ_CLOSE.replace_all(
                &RE_BLANK_LINES.replace_all(&RE_NULL.replace_all(html, ""), "\n"),
                "</blockquote>",
            ),
            "<blockquote>",
        )
        .to_string()
}
