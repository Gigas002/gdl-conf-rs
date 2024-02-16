use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Twitter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ads: Option<bool>,
    cards: Option<BoolOrString>,
    cards_blacklist: Option<Vec<String>>,
    conversations: Option<BoolOrString>,
    csrf: Option<String>,
    expand: Option<bool>,
    include: Option<StringOrList>,
    transform: Option<bool>,
    tweet_endpoint: Option<String>,
    size: Option<Vec<String>>,
    logout: Option<bool>,
    pinned: Option<bool>,
    quoted: Option<bool>,
    ratelimit: Option<String>,
    replies: Option<bool>,
    retweets: Option<bool>,
    timeline: Option<TwitterTimeline>,
    text_tweets: Option<bool>,
    twitpic: Option<bool>,
    unique: Option<bool>,
    users: Option<String>,
    videos: Option<BoolOrString>,
}

impl Twitter {
    pub fn new() -> Self {
        return Twitter {
            base: None,
            ads: Some(true),
            cards: Some(BoolOrString::Bool(false)),
            cards_blacklist: None,
            conversations: Some(BoolOrString::Bool(false)),
            csrf: Some("cookies".to_string()),
            expand: Some(false),
            include: Some(StringOrList::String("timeline".to_string())),
            transform: Some(true),
            tweet_endpoint: Some("auto".to_string()),
            size: Some(vec!["orig".to_string(), "4096x4096".to_string(), "large".to_string(), "medium".to_string(), "small".to_string()]),
            logout: Some(false),
            pinned: Some(false),
            quoted: Some(false),
            ratelimit: Some("wait".to_string()),
            replies: Some(true),
            retweets: Some(false),
            timeline: Some(TwitterTimeline::new()),
            text_tweets: Some(false),
            twitpic: Some(false),
            unique: Some(true),
            users: Some("user".to_string()),
            videos: Some(BoolOrString::Bool(true)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TwitterTimeline {
    strategy: Option<String>,
}

impl TwitterTimeline {
    pub fn new() -> Self {
        return TwitterTimeline {
            strategy: Some("auto".to_string()),
        };
    }
}
