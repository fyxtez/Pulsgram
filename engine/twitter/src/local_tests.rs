#[cfg(test)]
mod tests {
    use crate::regex::*;

    #[test]
    fn test_message_type_retweet_elchartox() {
        let message = r#"🖼️🔄 elchartox Retweeted brommmyy
$GHOST reclaimed 8M 
everything we touch turns to gold 
projects of this caliber don't come often… this is most likely the last entry you'll see before the next leg 
don't say I didn't warn you"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Retweet {
                user,
                user_link,
                text,
                mentioned,
                mentioned_link,
            } => {
                assert_eq!(user, "elchartox");
                assert_eq!(user_link, None);
                assert_eq!(mentioned, "brommmyy");
                assert_eq!(mentioned_link, None);
                assert!(text.contains("$GHOST reclaimed 8M"));
            }
            _ => panic!("Expected Retweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_tweet_shadow36() {
        let message = r#"🖼️📝 _Shadow36 Tweeted
50 mothafuckin million.
Now 100x it.
$buttcoin"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Tweet { user, user_link, text } => {
                assert_eq!(user, "_Shadow36");
                assert_eq!(user_link, None);
                assert!(text.contains("50 mothafuckin million"));
            }
            _ => panic!("Expected Tweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_retweet_shawmakesmagic() {
        let message = r#"🔄 shawmakesmagic Retweeted elizaOS
ATTENTION
We are looking for testers for the next-generation version of Eliza.
Requirements:
- Comfortable with setting up AI agents
- Able to structure feedback on bugs
- Minimum IQ of 50, but we are open-minded
Interested? Join the Eliza Discord and visit the #coders channel.
Meaningful input will be rewarded."#;

        let result = parse_message_type(message);
        match result {
            MessageType::Retweet {
                user,
                user_link,
                text,
                mentioned,
                mentioned_link,
            } => {
                assert_eq!(user, "shawmakesmagic");
                assert_eq!(user_link, None);
                assert_eq!(mentioned, "elizaOS");
                assert_eq!(mentioned_link, None);
                assert!(text.contains("ATTENTION"));
            }
            _ => panic!("Expected Retweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_tweet_hungrypawnsx() {
        let message = r#"📝 HungryPawnsX Tweeted
lol fine.  BarkingPuppy is starting to grow on me."#;

        let result = parse_message_type(message);
        match result {
            MessageType::Tweet { user, user_link, text } => {
                assert_eq!(user, "HungryPawnsX");
                assert_eq!(user_link, None);
                assert!(text.contains("BarkingPuppy is starting to grow on me"));
            }
            _ => panic!("Expected Tweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_reply_pumpfun() {
        let message = r#"🖼️🖇️ (https://fxtwitter.com/cladzsol/status/2020218888353349740) Pumpfun (https://x.com/Pumpfun) Replied To cladzsol (https://x.com/cladzsol)
download the app and see for yourself"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Reply {
                user,
                user_link,
                text,
                replied_to,
                replied_to_link,
            } => {
                assert_eq!(user, "Pumpfun");
                assert_eq!(user_link, Some("https://x.com/Pumpfun".to_string()));
                assert_eq!(replied_to, "cladzsol");
                assert_eq!(replied_to_link, Some("https://x.com/cladzsol".to_string()));
                assert!(text.contains("download the app and see for yourself"));
            }
            _ => panic!("Expected Reply, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_retweet_toly() {
        let message = r#"🔄 toly Retweeted zkmort
This is why we do it"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Retweet {
                user,
                user_link,
                text,
                mentioned,
                mentioned_link,
            } => {
                assert_eq!(user, "toly");
                assert_eq!(user_link, None);
                assert_eq!(mentioned, "zkmort");
                assert_eq!(mentioned_link, None);
                assert_eq!(text, "This is why we do it");
            }
            _ => panic!("Expected Retweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_tweet_himgajria() {
        let message = r#"📝 himgajria Tweeted
Underestimating Inunomics will feed your anti-portfolio."#;

        let result = parse_message_type(message);
        match result {
            MessageType::Tweet { user, user_link, text } => {
                assert_eq!(user, "himgajria");
                assert_eq!(user_link, None);
                assert!(text.contains("Inunomics"));
            }
            _ => panic!("Expected Tweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_tweet_pumpfun() {
        let message = r#"🖼️📝 Pumpfun Tweeted
trenches bought the bottom yet nobody holds bitcoin
lesson in there"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Tweet { user, user_link, text } => {
                assert_eq!(user, "Pumpfun");
                assert_eq!(user_link, None);
                assert!(text.contains("trenches bought the bottom"));
            }
            _ => panic!("Expected Tweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_tweet_onlyljc() {
        let message = r#"📝 OnlyLJC Tweeted
Why everytime a fat bitch serves my coffee she is rude and soulless, but the baddie smiles and makes your day better 
Like piggy if you want a tip for more cookies atleast be nice"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Tweet { user, user_link, text } => {
                assert_eq!(user, "OnlyLJC");
                assert_eq!(user_link, None);
                assert!(text.contains("Why everytime"));
            }
            _ => panic!("Expected Tweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_recession_warning() {
        let message = r#"*RECESSION WARNING*
The latest job openings data just dropped.
It's an absolute disaster.
U.S. job openings are now at recession levels.
Not only that, but we're below the levels recorded during the 2001 recession.
The market will likely react tomorrow.
Brace for impact."#;

        let result = parse_message_type(message);
        assert_eq!(result, MessageType::Unknown);
    }

    #[test]
    fn test_message_type_tweet_cryptolyxe_advice() {
        let message = r#"📝 cryptolyxe Tweeted
my honest advice if you've been struggling with the markets recently:
go back to enjoying some dumb shit.
some of the best memoriesin my whole life have been with basically $0 travelling different countries, or doing spontaenous activites with your closest friends
laughing with the boys is peak living. whether you have millions of dollars or none.
your life isn't over because some numbers on a screen went down,
there's always going to be good and bad times in markets, there will always be another cook, another opportunity if you stick around long enough."#;

        let result = parse_message_type(message);
        match result {
            MessageType::Tweet { user, user_link, text } => {
                assert_eq!(user, "cryptolyxe");
                assert_eq!(user_link, None);
                assert!(text.contains("my honest advice"));
            }
            _ => panic!("Expected Tweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_reply_trading_axe() {
        let message = r#"🖇️ (https://fxtwitter.com/k8sified/status/2020389649089536255) trading_axe (https://x.com/trading_axe) Replied To k8sified (https://x.com/k8sified)
It still doesn't answer how any of it came into existence originally.
Why do "mystery languages" exist or HAVE existed, like DuQu, that took researchers years to decipher?
How do you just create a foreign language for malware that actually does what you want it to do and "the machine" [computer] understands it?
Surely the machine must have it inside already to know it, you can't add something new?
~ Dr. Axius."#;

        let result = parse_message_type(message);
        match result {
            MessageType::Reply {
                user,
                user_link,
                text,
                replied_to,
                replied_to_link,
            } => {
                assert_eq!(user, "trading_axe");
                assert_eq!(user_link, Some("https://x.com/trading_axe".to_string()));
                assert_eq!(replied_to, "k8sified");
                assert_eq!(replied_to_link, Some("https://x.com/k8sified".to_string()));
                assert!(text.contains("It still doesn't answer"));
            }
            _ => panic!("Expected Reply, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_quote_shawmakesmagic() {
        let message = r#"💬 (https://fxtwitter.com/grok/status/2020357196261859511) shawmakesmagic (https://x.com/shawmakesmagic) Quoted grok (https://x.com/grok)
Lol"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Quote {
                user,
                user_link,
                text,
                quoted,
                quoted_link,
            } => {
                assert_eq!(user, "shawmakesmagic");
                assert_eq!(user_link, Some("https://x.com/shawmakesmagic".to_string()));
                assert_eq!(quoted, "grok");
                assert_eq!(quoted_link, Some("https://x.com/grok".to_string()));
                assert_eq!(text, "Lol");
            }
            _ => panic!("Expected Quote, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_tweet_cryptolyxe_morning() {
        let message = r#"📝 cryptolyxe Tweeted
good morning boob and butt enjoyers"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Tweet { user, user_link, text } => {
                assert_eq!(user, "cryptolyxe");
                assert_eq!(user_link, None);
                assert_eq!(text, "good morning boob and butt enjoyers");
            }
            _ => panic!("Expected Tweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_follow_blknoiz06() {
        let message = r#"🦶 blknoiz06 (https://x.com/blknoiz06) followed rohunvora (https://x.com/rohunvora)
 rohun (rohunvora)
2,406 Following | 10,927 Followers
automating things
📍 nyc for now
🔗 http://github.com/rohunvora"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Follow {
                follower,
                follower_link,
                followee,
                followee_link,
                profile_info,
            } => {
                assert_eq!(follower, "blknoiz06");
                assert_eq!(follower_link, Some("https://x.com/blknoiz06".to_string()));
                assert_eq!(followee, "rohunvora");
                assert_eq!(followee_link, Some("https://x.com/rohunvora".to_string()));
                assert!(profile_info.len() > 0);
            }
            _ => panic!("Expected Follow, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_follow_shawmakesmagic() {
        let message = r#"🦶 shawmakesmagic (https://x.com/shawmakesmagic) followed seanwbren (https://x.com/seanwbren)
 Sean Brennan (seanwbren)
6,445 Following | 5,374 Followers
lead @regent_cx 🤖 regent.eth | Prev: @bioprotocol @infura_io @makerdao
📍 NYC
🔗 http://warpcast.com/seanwbren"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Follow {
                follower,
                follower_link,
                followee,
                followee_link,
                profile_info,
            } => {
                assert_eq!(follower, "shawmakesmagic");
                assert_eq!(follower_link, Some("https://x.com/shawmakesmagic".to_string()));
                assert_eq!(followee, "seanwbren");
                assert_eq!(followee_link, Some("https://x.com/seanwbren".to_string()));
                assert!(profile_info.contains("Sean Brennan"));
                assert!(profile_info.contains("6,445 Following"));
            }
            _ => panic!("Expected Follow, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_follow_toly() {
        let message = r#"🦶 toly (https://x.com/toly) followed yashaxbt (https://x.com/yashaxbt)
 Inuyasha (yashaxbt)
753 Following | 719 Followers
📍 null
🔗 https://partner.blofin.com/d/Inuyasha"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Follow {
                follower,
                follower_link,
                followee,
                followee_link,
                profile_info,
            } => {
                assert_eq!(follower, "toly");
                assert_eq!(follower_link, Some("https://x.com/toly".to_string()));
                assert_eq!(followee, "yashaxbt");
                assert_eq!(followee_link, Some("https://x.com/yashaxbt".to_string()));
                assert!(profile_info.contains("Inuyasha"));
                assert!(profile_info.contains("753 Following"));
            }
            _ => panic!("Expected Follow, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_follow_pmarca() {
        let message = r#"🦶 pmarca (https://x.com/pmarca) followed chiweethedog (https://x.com/chiweethedog)
 Roy Drones Jr (chiweethedog)
998 Following | 47,700 Followers
macdonal im dont love it
📍 Ring of Fire
🔗 https://x.com/cia/status/486255845588475905"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Follow {
                follower,
                follower_link,
                followee,
                followee_link,
                profile_info,
            } => {
                assert_eq!(follower, "pmarca");
                assert_eq!(follower_link, Some("https://x.com/pmarca".to_string()));
                assert_eq!(followee, "chiweethedog");
                assert_eq!(followee_link, Some("https://x.com/chiweethedog".to_string()));
                assert!(profile_info.len() > 0);
            }
            _ => panic!("Expected Follow, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_retweet_double_image() {
        let message = r#"🖼️🖼️🔄 elchartox Retweeted WhaleInsider
JUST IN: $GHOST shows strength despite market volatility as solana follows their X page."#;

        let result = parse_message_type(message);
        match result {
            MessageType::Retweet {
                user,
                user_link,
                text,
                mentioned,
                mentioned_link,
            } => {
                assert_eq!(user, "elchartox");
                assert_eq!(user_link, None);
                assert_eq!(mentioned, "WhaleInsider");
                assert_eq!(mentioned_link, None);
                assert!(text.contains("JUST IN: $GHOST shows strength"));
            }
            _ => panic!("Expected Retweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_tweet_triple_image() {
        let message = r#"🖼️🖼️🖼️📝 testuser Tweeted
Testing with three images"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Tweet { user, user_link, text } => {
                assert_eq!(user, "testuser");
                assert_eq!(user_link, None);
                assert_eq!(text, "Testing with three images");
            }
            _ => panic!("Expected Tweet, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_reply_double_image() {
        let message = r#"🖼️🖼️🖇️ (https://fxtwitter.com/user1/status/123) user2 (https://x.com/user2) Replied To user1 (https://x.com/user1)
Reply with two images"#;

        let result = parse_message_type(message);
        match result {
            MessageType::Reply {
                user,
                user_link,
                text,
                replied_to,
                replied_to_link,
            } => {
                assert_eq!(user, "user2");
                assert_eq!(user_link, Some("https://x.com/user2".to_string()));
                assert_eq!(replied_to, "user1");
                assert_eq!(replied_to_link, Some("https://x.com/user1".to_string()));
                assert_eq!(text, "Reply with two images");
            }
            _ => panic!("Expected Reply, got {:?}", result),
        }
    }

    #[test]
    fn test_message_type_profile_update() {
        let message = r#"🆔 Profile Update - rektober 

- url
❌ null
-
✅ [object Object]"#;

        let result = parse_message_type(message);
        match result {
            MessageType::ProfileUpdate { user, update_info } => {
                assert_eq!(user, "rektober");
                assert!(update_info.contains("- url"));
                assert!(update_info.contains("[object Object]"));
            }
            _ => panic!("Expected ProfileUpdate, got {:?}", result),
        }
    }
}