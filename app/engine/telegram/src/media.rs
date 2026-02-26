use grammers_client::{Client, media::Photo};
use grammers_tl_types as tl;

pub async fn save_photo(
    client: &Client,
    photo: &Photo,
    message_id: i32,
) -> Result<String, std::io::Error> {
    let path = format!("temp_photo_{}.jpg", message_id);
    client.download_media(photo, &path).await?;
    Ok(path)
}

pub fn extract_photo_url_from_raw(update: &tl::enums::Update) -> Option<String> {
    // Only handle new incoming messages.
    // Ignore edits, deletes, and other update types.
    let tl::enums::Update::NewMessage(u) = update else {
        return None;
    };
    // Extract the actual message content.
    let tl::enums::Message::Message(msg) = &u.message else {
        return None;
    };

    // Check if the message contains a WebPage media attachment.
    // Telegram automatically attaches this when a message contains a URL
    // that generates a link preview.
    let Some(tl::enums::MessageMedia::WebPage(wp)) = &msg.media else {
        return None;
    };

    //  Extract the original URL used to generate the preview.
    // WebPage::Pending appears when Telegram is still fetching metadata.
    // WebPage::Page appears when the preview is fully resolved.
    match &wp.webpage {
        tl::enums::WebPage::Pending(pending) => pending.url.clone(),
        tl::enums::WebPage::Page(page) => Some(page.url.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use grammers_tl_types as tl;

    #[test]
    fn test_extract_photo_url() {
        let update = tl::enums::Update::NewMessage(tl::types::UpdateNewMessage {
            message: tl::enums::Message::Message(tl::types::Message {
                out: false,
                mentioned: false,
                media_unread: false,
                silent: false,
                post: false,
                from_scheduled: false,
                                schedule_repeat_period: None,
                summary_from_language: None,
                legacy: false,
                edit_hide: false,
                pinned: false,
                noforwards: false,
                invert_media: true,
                offline: false,
                video_processing_pending: false,
                paid_suggested_post_stars: false,
                paid_suggested_post_ton: false,
                id: 13012,
                from_id: None,
                from_boosts_applied: None,
                peer_id: tl::enums::Peer::User(tl::types::PeerUser {
                    user_id: 7910357312,
                }),
                saved_peer_id: None,
                fwd_from: None,
                via_bot_id: None,
                via_business_bot_id: None,
                reply_to: None,
                date: 1771958199,
                message: "test".to_string(),
                media: Some(tl::enums::MessageMedia::WebPage(
                    tl::types::MessageMediaWebPage {
                        force_large_media: false,
                        force_small_media: false,
                        manual: true,
                        safe: false,
                        webpage: tl::enums::WebPage::Pending(tl::types::WebPagePending {
                            id: 6445893524869168661,
                            url: Some(
                                "https://pbs.twimg.com/profile_images/2024150251234320384/DUACK2O3.jpg".to_string(),
                            ),
                            date: 1771958319,
                        }),
                    },
                )),
                reply_markup: None,
                entities: None,
                views: None,
                forwards: None,
                replies: None,
                edit_date: None,
                post_author: None,
                grouped_id: None,
                reactions: None,
                restriction_reason: None,
                ttl_period: None,
                quick_reply_shortcut_id: None,
                effect: None,
                factcheck: None,
                report_delivery_until_date: None,
                paid_message_stars: None,
                suggested_post: None,
            }),
            pts: 22428,
            pts_count: 1,
        });

        let url = extract_photo_url_from_raw(&update);
        assert_eq!(
            url,
            Some(
                "https://pbs.twimg.com/profile_images/2024150251234320384/DUACK2O3.jpg".to_string()
            )
        );
    }

    #[test]
    fn test_no_media_returns_none() {
        let update = tl::enums::Update::NewMessage(tl::types::UpdateNewMessage {
            message: tl::enums::Message::Message(tl::types::Message {
                out: false,
                mentioned: false,
                media_unread: false,
                silent: false,
                schedule_repeat_period: None,
                summary_from_language: None,
                post: false,
                from_scheduled: false,
                legacy: false,
                edit_hide: false,
                pinned: false,
                noforwards: false,
                invert_media: false,
                offline: false,
                video_processing_pending: false,
                paid_suggested_post_stars: false,
                paid_suggested_post_ton: false,
                id: 1,
                from_id: None,
                from_boosts_applied: None,
                peer_id: tl::enums::Peer::User(tl::types::PeerUser { user_id: 1 }),
                saved_peer_id: None,
                fwd_from: None,
                via_bot_id: None,
                via_business_bot_id: None,
                reply_to: None,
                date: 0,
                message: "no media".to_string(),
                media: None,
                reply_markup: None,
                entities: None,
                views: None,
                forwards: None,
                replies: None,
                edit_date: None,
                post_author: None,
                grouped_id: None,
                reactions: None,
                restriction_reason: None,
                ttl_period: None,
                quick_reply_shortcut_id: None,
                effect: None,
                factcheck: None,
                report_delivery_until_date: None,
                paid_message_stars: None,
                suggested_post: None,
            }),
            pts: 1,
            pts_count: 1,
        });

        assert_eq!(extract_photo_url_from_raw(&update), None);
    }
}
