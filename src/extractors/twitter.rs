use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Twitter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_blacklist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversations: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csrf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweet_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quoted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retweets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<TwitterTimeline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_tweets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitpic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<BoolOrString>,
}

impl Twitter {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = username;
        base.password = password;

        return Twitter {
            base: Some(base),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

impl TwitterTimeline {
    pub fn new() -> Self {
        return TwitterTimeline {
            strategy: Some("auto".to_string()),
        };
    }
}
