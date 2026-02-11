mod utils;
use publisher::EventBus;
use std::sync::Arc;
use telegram_types::{Client, Message, Peer};

pub async fn run(
    bus: Arc<EventBus>,
    target_dialog_id: i64,
    destination: Peer,
    dispatcher: Arc<Client>,
    destination_test: Peer,
) {
    println!("KOL Follows running...");
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        let message = event.message;

        let peer_id = message.peer_id().bare_id();

        if !target_dialog_id.eq(&peer_id) {
            continue;
        }

        handle_follow(message, &dispatcher, &destination, &destination_test).await;
    }
}

pub async fn handle_follow(
    message: Message,
    dispatcher: &Client,
    destination: &Peer,
    _destination_test: &Peer,
) {
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

// TODO: Expensive method, needs fix or find other solution for the inconsistent blockqotes in telegram rendering.
fn postprocess_html(html: &str) -> String {
    let mut result = html.to_string();

    // 1. Remove "null" appearing as location text (from JSON nulls)
    let re_null = regex::Regex::new(r"(?i)\bnull\b").unwrap();
    result = re_null.replace_all(&result, "").to_string();

    // 2. Remove lines that contain only whitespace (inside or outside blockquotes)
    let re_blank_lines = regex::Regex::new(r"\n[ \t]*\n").unwrap();
    // Collapse to single newline repeatedly until stable
    loop {
        let next = re_blank_lines.replace_all(&result, "\n").to_string();
        if next == result {
            break;
        }
        result = next;
    }

    // 3. Clean up whitespace right before </blockquote>
    let re_bq = regex::Regex::new(r"[\s]+</blockquote>").unwrap();
    result = re_bq.replace_all(&result, "</blockquote>").to_string();

    // 4. Clean up whitespace right after <blockquote>
    let re_bq_open = regex::Regex::new(r"<blockquote>\s+").unwrap();
    result = re_bq_open.replace_all(&result, "<blockquote>").to_string();

    result.replace("null", "")
}