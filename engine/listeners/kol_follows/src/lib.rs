mod utils;
use publisher::EventBus;
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
    // TODO!
    if message.text().contains("diloytte"){
        println!("Ignoring diloytte");
        return
    }

    if !simple_is_followed_check(message.text()) {return}

    let msg_original_html = message.html_text();

    let mut html_content = remove_emojis(&message.html_text());

    dbg!(&msg_original_html);
    dbg!("-------------");
    dbg!(&html_content);

    html_content = html_content.replace("null", "");

    if cfg!(feature = "production") {
        let input_message = telegram_types::InputMessage::new().html(&html_content);
        if let Err(err) = dispatcher.send_message(destination, input_message).await {
            eprintln!("Failed to send message: {:?}", err);
        }
    } else {
        println!("[LOCAL] Would send message: {}", html_content);
    }


    println!("{:?}",html_content);
}

fn remove_emojis(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii() || (*c as u32) < 0x1F000)
        .collect()
}


fn simple_is_followed_check(message_text: &str)->bool{
     let first_line = message_text.lines().next().unwrap_or("");

     first_line.contains("followed")
}