use telegram_types::{Client, Message, Peer};
use twitter::regex::MessageType;

pub async fn handle_follow(
    message_type: &MessageType,
    full_message: Message,
    client: &Client,
    targeted_kols: &[String],
    destination: &Peer,
    source: &Peer,
) {
    if let MessageType::Follow {
        follower,
        followee,
        profile_info,
    } = message_type
    {
        // len = 0 <- track everyone
        // if targeted_kols.len().eq(&0) || targeted_kols.contains(follower){
        let _ = client
            .forward_messages(destination, &[full_message.id()], source)
            .await;
        // }
    } else {
    }
}
