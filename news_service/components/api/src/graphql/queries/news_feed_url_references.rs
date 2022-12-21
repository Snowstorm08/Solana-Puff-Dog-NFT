use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use db::{
    models::news_feed_url_reference::NewsFeedUrlReference,
    queries::news_feed_url_references_query::get_news_feed_url_references_with_metadata,
    sql::news_feed_url_references_query::get_news_feed_url_references,
};
use sqlx::postgres::PgPool;

pub async fn news_feed_url_references_query<'a>(
    pool: &PgPool,
    url_slug: String,
) -> FieldResult<Vec<NewsFeedUrlReference>> {
    let news_feed_url_references_result = get_news_feed_url_references(pool, url_slug).await;

    if let Some(news_feed_url_references_query_list) = news_feed_url_references_result {
        Ok(get_news_feed_url_references_with_metadata(
            news_feed_url_references_query_list,
        ))
    } else {
        Err(GqlError::NotFound.extend())
    }
}

#[cfg(test)]
mod tests {

    use crate::graphql::test_util::create_fake_schema;
    use async_graphql::value;
    use db::{
        init_env, init_test_db_pool,
        util::{
            convert::now_utc_timestamp,
            test::test_util::{
                create_fake_news_referenced_tweet_url, create_fake_news_referenced_tweets,
                create_fake_news_tweet, create_fake_news_tweet_url,
                create_fake_news_twitter_referenced_user, create_fake_news_twitter_user,
            },
        },
    };

    #[tokio::test]
    async fn get_news_feed_url_references_test() {
        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        let created_at_timestamp = now_utc_timestamp();

        create_fake_news_twitter_user(&db_pool).await;
        create_fake_news_tweet(&db_pool, created_at_timestamp).await;
        create_fake_news_tweet_url(&db_pool, created_at_timestamp).await;
        create_fake_news_referenced_tweet_url(&db_pool).await;
        create_fake_news_referenced_tweets(&db_pool).await;
        create_fake_news_twitter_referenced_user(&db_pool).await;

        let schema = create_fake_schema(db_pool);

        let resp = schema
            .execute(
                r#"
                query {
                    newsFeedUrlReferences(urlSlug: "example-title") {
                        tweetId
                        tweetText
                        tweetCreatedAtStr
                        authorUsername
                        retweetedByUsernames 
                      }
                }
                "#,
            )
            .await;
        assert_eq!(
            resp.data,
            value!({
                "newsFeedUrlReferences": [
                    {
                        "tweetId": String::from("3"),
                        "tweetText": String::from("quoted_tweet_text"),
                        "tweetCreatedAtStr": String::from("created_at_str"),
                        "authorUsername": String::from("quoted_username"),
                        "retweetedByUsernames": [],
                    },
                    {
                        "tweetId": String::from("1"),
                        "tweetText": String::from("tweet_text"),
                        "tweetCreatedAtStr": String::from("created_at_str"),
                        "authorUsername": String::from("username"),
                        "retweetedByUsernames": [String::from("@retweeted_username")],
                    }
                ],
            })
        );
    }
}
