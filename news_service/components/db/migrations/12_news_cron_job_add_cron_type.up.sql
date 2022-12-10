TRUNCATE news_cron_job RESTART IDENTITY;
ALTER TABLE news_cron_job ADD cron_type TEXT NOT NULL;