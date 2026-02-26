use regex::Regex;
use std::sync::LazyLock;

pub fn postprocess_html(html: &str) -> String {
    RE_BQ_OPEN
        .replace_all(
            &RE_BQ_CLOSE.replace_all(
                &RE_BLANK_LINES.replace_all(&RE_NULL.replace_all(html, ""), "\n"),
                "</blockquote>",
            ),
            "<blockquote>",
        )
        .to_string()
}

static RE_NULL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bnull\b").expect("Invalid RE_NULL regex"));

static RE_BLANK_LINES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\n[ \t]*\n").expect("Invalid RE_BLANK_LINES regex"));

static RE_BQ_CLOSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\s]+</blockquote>").expect("Invalid RE_BQ_CLOSE regex"));

static RE_BQ_OPEN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<blockquote>\s+").expect("Invalid RE_BQ_OPEN regex"));

static EMOJI_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[\p{Emoji_Presentation}\p{Extended_Pictographic}]").expect("Invalid emoji regex")
});

pub fn remove_emojis(input: &str) -> std::borrow::Cow<'_, str> {
    EMOJI_REGEX.replace_all(input, "")
}
