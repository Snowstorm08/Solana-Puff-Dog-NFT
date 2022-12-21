use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::news_feed_url_reference::NewsFeedUrlReference;

// News feed URL references
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
#[graphql(name = "NewsFeedUrlReference")]
pub struct NewsFeedUrlReferencesQuery {
    pub url_id: i32,
    pub text: String,
    pub tweet_id: i64,
    pub author_id: i64,
    pub created_at: i64,
    pub created_at_str: String,
    pub username: Option<String>,
    pub referenced_username: Option<String>,
    pub referenced_tweet_id: Option<i64>,
    pub referenced_tweet_kind: Option<String>,
}

pub fn get_news_feed_url_references_with_metadata(
    news_feed_url_references_query_list: Vec<NewsFeedUrlReferencesQuery>,
) -> Vec<NewsFeedUrlReference> {
    let mut news_feed_url_references: Vec<NewsFeedUrlReference> = vec![];
    for news_feed_url_reference_query in news_feed_url_references_query_list.clone() {
        if news_feed_url_reference_query.referenced_tweet_kind != Some(String::from("retweeted")) {
            let retweeted_by_usernames: Vec<String> = get_retweeted_by_usernames(
                &news_feed_url_references_query_list,
                &news_feed_url_reference_query,
            );

            let author_username = get_author_username(&news_feed_url_reference_query.clone())
                .unwrap_or_else(|| String::from(""));

            let news_feed_url_reference = NewsFeedUrlReference {
                url_id: news_feed_url_reference_query.url_id,
                tweet_id: news_feed_url_reference_query.tweet_id.to_string(),
                tweet_text: news_feed_url_reference_query.text.clone(),
                tweet_created_at_str: news_feed_url_reference_query.created_at_str.clone(),
                author_username,
                retweeted_by_usernames,
            };
            news_feed_url_references.push(news_feed_url_reference);
        }
    }
    news_feed_url_references
}

pub fn get_retweeted_by_usernames(
    news_feed_url_references_query_list: &Vec<NewsFeedUrlReferencesQuery>,
    news_feed_url_reference_query: &NewsFeedUrlReferencesQuery,
) -> Vec<String> {
    // TODO make const for "retweeted"
    // Get the list of users that retweeted a tweet with a url
    news_feed_url_references_query_list
        .clone()
        .into_iter()
        .filter(|nfu| {
            nfu.referenced_tweet_id == Some(news_feed_url_reference_query.tweet_id)
                && nfu.referenced_tweet_kind == Some(String::from("retweeted"))
                && get_author_username(nfu).is_some()
        })
        .map(|nfu| get_author_username(&nfu))
        .map(|username| format!("@{}", username.unwrap()))
        .collect()
}

pub fn get_author_username(
    news_feed_url_reference_query: &NewsFeedUrlReferencesQuery,
) -> Option<String> {
    match news_feed_url_reference_query.username.clone() {
        Some(author_username) => Some(author_username),
        None => news_feed_url_reference_query.referenced_username.clone(),
    }
}
