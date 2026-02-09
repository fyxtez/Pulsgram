use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Tweet { 
        user: String, 
        user_link: Option<String>,
        text: String 
    },
    Retweet { 
        user: String, 
        user_link: Option<String>,
        text: String, 
        mentioned: String,
        mentioned_link: Option<String>
    },
    Reply { 
        user: String, 
        user_link: Option<String>,
        text: String, 
        replied_to: String,
        replied_to_link: Option<String>
    },
    Quote { 
        user: String, 
        user_link: Option<String>,
        text: String, 
        quoted: String,
        quoted_link: Option<String>
    },
    Follow { 
        follower: String, 
        follower_link: Option<String>,
        followee: String,
        followee_link: Option<String>,
        profile_info: String 
    },
    ProfileUpdate { 
        user: String, 
        update_info: String 
    },
    Unknown,
}

static TWEET_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn tweet_regex() -> &'static Regex {
    TWEET_REGEX.get_or_init(|| {
        Regex::new(r"(?ms)^(?:🖼️)*📝 (\S+)(?: \((https?://[^\)]+)\))? Tweeted\s*\n+(.+)").unwrap()
    })
}

static RETWEET_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn retweet_regex() -> &'static Regex {
    RETWEET_REGEX.get_or_init(|| {
        Regex::new(r"(?ms)^(?:🖼️)*🔄 (\S+)(?: \((https?://[^\)]+)\))? Retweeted (\S+)(?: \((https?://[^\)]+)\))?\s*\n+(.+)").unwrap()
    })
}

static QUOTE_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn quote_regex() -> &'static Regex {
    QUOTE_REGEX.get_or_init(|| {
        Regex::new(r"(?ms)^(?:🖼️)*💬(?:\s*\([^)]+\))? (\S+)(?: \((https?://[^\)]+)\))? Quoted (\S+)(?: \((https?://[^\)]+)\))?\s*\n+(.+)").unwrap()
    })
}

static REPLY_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn reply_regex() -> &'static Regex {
    REPLY_REGEX.get_or_init(|| {
        Regex::new(r"(?ms)^(?:[🖼️🎥])*🖇️(?:\s*\([^\)]+\))?\s*(\S+)(?:\s*\((https?://[^\)]+)\))?\s+Replied To\s+(\S+)(?:\s*\((https?://[^\)]+)\))?\s*\n+(.+)").unwrap()
    })
}

static FOLLOW_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn follow_regex() -> &'static Regex {
    FOLLOW_REGEX.get_or_init(|| {
        Regex::new(r"(?ms)^🦶 (\S+)(?: \((https?://[^\)]+)\))? followed (\S+)(?: \((https?://[^\)]+)\))?\s*\n(.+)").unwrap()
    })
}

static PROFILE_UPDATE_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn profile_update_regex() -> &'static Regex {
    PROFILE_UPDATE_REGEX.get_or_init(|| {
        Regex::new(r"(?ms)^🆔 Profile Update - (\S+)\s*\n+(.+)").unwrap()
    })
}

pub fn parse_message_type(message: &str) -> MessageType {
    // Try follow pattern first
    if let Some(caps) = follow_regex().captures(message) {
        return MessageType::Follow {
            follower: caps[1].to_string(),
            follower_link: caps.get(2).map(|m| m.as_str().to_string()),
            followee: caps[3].to_string(),
            followee_link: caps.get(4).map(|m| m.as_str().to_string()),
            profile_info: caps[5].to_string(),
        };
    }
    
    // Try profile update pattern
    if let Some(caps) = profile_update_regex().captures(message) {
        return MessageType::ProfileUpdate {
            user: caps[1].to_string(),
            update_info: caps[2].to_string(),
        };
    }
    
    // Try retweet pattern
    if let Some(caps) = retweet_regex().captures(message) {
        return MessageType::Retweet {
            user: caps[1].to_string(),
            user_link: caps.get(2).map(|m| m.as_str().to_string()),
            text: caps[5].to_string(),
            mentioned: caps[3].to_string(),
            mentioned_link: caps.get(4).map(|m| m.as_str().to_string()),
        };
    }
    
    // Try quote pattern
    if let Some(caps) = quote_regex().captures(message) {
        return MessageType::Quote {
            user: caps[1].to_string(),
            user_link: caps.get(2).map(|m| m.as_str().to_string()),
            text: caps[5].to_string(),
            quoted: caps[3].to_string(),
            quoted_link: caps.get(4).map(|m| m.as_str().to_string()),
        };
    }
    
    // Try reply pattern
    if let Some(caps) = reply_regex().captures(message) {
        return MessageType::Reply {
            user: caps[1].to_string(),
            user_link: caps.get(2).map(|m| m.as_str().to_string()),
            text: caps[5].to_string(),
            replied_to: caps[3].to_string(),
            replied_to_link: caps.get(4).map(|m| m.as_str().to_string()),
        };
    }
    
    // Try tweet pattern
    if let Some(caps) = tweet_regex().captures(message) {
        return MessageType::Tweet {
            user: caps[1].to_string(),
            user_link: caps.get(2).map(|m| m.as_str().to_string()),
            text: caps[3].to_string(),
        };
    }
    
    MessageType::Unknown
}