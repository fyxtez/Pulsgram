use telegram_types::Message;

pub async fn handle(
    message: &Message,
) {
    //TODO: Sometimes this might not work
    let sender = match message.sender() {
        Some(s) => s,
        None => {
            dbg!("NO_MESSAGE_SENDER");
            return;
        }
    };

    let name = sender
        .name()
        .unwrap_or("NO_NAME")
        .to_string();

    let username = sender
        .username()
        .unwrap_or("NO_USERNAME")
        .to_string();


        if name.contains("Rick") || name.contains("Phanes"){
            return;
        }

        println!("{}",format!("Group msg from: {} said: \n {}",name,message.text()));
}
