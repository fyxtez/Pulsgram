use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Tweet {
        user: String,
        text: String,
    },
    Retweet {
        user: String,
        text: String,
        mentioned: String,
    },
    Reply {
        user: String,
        text: String,
        replied_to: String,
    },
    Quote {
        user: String,
        text: String,
        quoted: String,
    },
    Follow {
        follower: String,
        followee: String,
        profile_info: String,
    },
    ProfileUpdate {
        user: String,
        update_info: String,
    },
    Unknown,
}

static TWEET_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn tweet_regex() -> &'static Regex {
    TWEET_REGEX.get_or_init(|| Regex::new(r"(?ms)^(?:🖼️)*📝 (\S+) Tweeted\s*\n+(.+)").unwrap())
}

static RETWEET_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn retweet_regex() -> &'static Regex {
    RETWEET_REGEX
        .get_or_init(|| Regex::new(r"(?ms)^(?:🖼️)*🔄 (\S+) Retweeted (\S+)\s*\n+(.+)").unwrap())
}

static QUOTE_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn quote_regex() -> &'static Regex {
    QUOTE_REGEX.get_or_init(|| {
        Regex::new(r"(?ms)^(?:🖼️)*💬 (?:\([^)]+\) )?(\S+)(?: \([^)]+\))? Quoted (\S+)(?: \([^)]+\))?\s*\n+(.+)")
            .unwrap()
    })
}

static REPLY_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn reply_regex() -> &'static Regex {
    REPLY_REGEX.get_or_init(|| {
        Regex::new(r"(?ms)^(?:[🖼️🎥])*🖇️(?:\s*\([^\)]+\))?\s*(\S+)(?:\s*\([^\)]+\))?\s+Replied To\s+(\S+)(?:\s*\([^\)]+\))?\s*\n+(.+)").unwrap()
    })
}

static FOLLOW_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn follow_regex() -> &'static Regex {
    FOLLOW_REGEX.get_or_init(|| {
        Regex::new(r"(?ms)^🦶 (\S+)(?: \([^\)]+\))? followed (\S+)(?: \([^\)]+\))?\s*\n(.+)")
            .unwrap()
    })
}

static PROFILE_UPDATE_REGEX: OnceLock<Regex> = OnceLock::new();
pub fn profile_update_regex() -> &'static Regex {
    PROFILE_UPDATE_REGEX
        .get_or_init(|| Regex::new(r"(?ms)^🆔 Profile Update - (\S+)\s*\n+(.+)").unwrap())
}

pub fn parse_message_type(message: &str) -> MessageType {
    if let Some(caps) = follow_regex().captures(message) {
        return MessageType::Follow {
            follower: caps[1].to_string(),
            followee: caps[2].to_string(),
            profile_info: caps[3].to_string(),
        };
    }

    if let Some(caps) = profile_update_regex().captures(message) {
        return MessageType::ProfileUpdate {
            user: caps[1].to_string(),
            update_info: caps[2].to_string(),
        };
    }

    if let Some(caps) = retweet_regex().captures(message) {
        return MessageType::Retweet {
            user: caps[1].to_string(),
            text: caps[3].to_string(),
            mentioned: caps[2].to_string(),
        };
    }

    if let Some(caps) = quote_regex().captures(message) {
        return MessageType::Quote {
            user: caps[1].to_string(),
            text: caps[3].to_string(),
            quoted: caps[2].to_string(),
        };
    }

    if let Some(caps) = reply_regex().captures(message) {
        return MessageType::Reply {
            user: caps[1].to_string(),
            text: caps[3].to_string(),
            replied_to: caps[2].to_string(),
        };
    }

    if let Some(caps) = reply_regex().captures(message) {
        return MessageType::Reply {
            user: caps[1].to_string(),
            text: caps[3].to_string(),
            replied_to: caps[2].to_string(),
        };
    }

    if let Some(caps) = tweet_regex().captures(message) {
        return MessageType::Tweet {
            user: caps[1].to_string(),
            text: caps[2].to_string(),
        };
    }

    MessageType::Unknown
}
