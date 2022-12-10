use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(strum_macros::Display)]
pub enum CronType {
    Main,
    Tweet,
}

// Cron job information
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
#[graphql(name = "NewsFeedStatus")]
pub struct NewsCronJob {
    #[graphql(skip)]
    pub cron_type: String,
    #[graphql(skip)]
    pub started_at: i64,
    #[graphql(skip)]
    pub started_at_str: String,
    pub completed_at: Option<i64>,
    #[graphql(skip)]
    pub completed_at_str: Option<String>,
    #[graphql(skip)]
    pub error: Option<String>,
}

#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsCronJobWithId {
    pub id: i32,
    pub cron_type: String,
    pub started_at: i64,
    pub started_at_str: String,
    pub completed_at: Option<i64>,
    pub completed_at_str: Option<String>,
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cron_type() {
        assert_eq!(CronType::Tweet.to_string(), String::from("Tweet"));
        assert_eq!(CronType::Main.to_string(), String::from("Main"));
    }
}
