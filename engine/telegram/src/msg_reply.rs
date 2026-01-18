pub async fn get_replied_message_text(client: &grammers_client::Client, message: &grammers_client::types::Message) -> Option<String> {
    if let Some(replied_msg_id) = message.reply_to_message_id() {
        let msgs = client.get_messages_by_id(message.peer().unwrap(), &[replied_msg_id]).await.ok()?;
        let replied_msg = msgs.get(0)?.as_ref()?;
        Some(replied_msg.text().to_string())
    } else {
        None
    }
}

// TODO: Test before using in production

// pub async fn get_all_replied_message_texts(
//     client: &grammers_client::Client,
//     message: &grammers_client::types::Message,
// ) -> Option<Vec<String>> {
//     let peer = message.peer().ok()?;

//     let mut texts = Vec::new();
//     let mut current_reply_id = message.reply_to_message_id()?;

//     loop {
//         let msgs = client
//             .get_messages_by_id(peer, &[current_reply_id])
//             .await
//             .ok()?;

//         let msg = msgs.get(0)?.as_ref()?;
//         texts.push(msg.text().to_string());

//         match msg.reply_to_message_id() {
//             Some(next_reply_id) => current_reply_id = next_reply_id,
//             None => break,
//         }
//     }

//     Some(texts)
// }
