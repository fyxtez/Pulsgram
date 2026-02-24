use std::sync::Arc;

use telegram_types::Client;
use telegram_types::PeerRef;

pub async fn run(client: Arc<Client>, to_peer: PeerRef, bus: Arc<publisher::EventBus>) {
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        //TODO: Needs proper error event, not TgEvent
        return;
        
        let error_message = event.message;

        let message_text = error_message.text().to_string();

        let _ = client.send_message(to_peer, message_text).await; //TODO: Handle error
    }
}
