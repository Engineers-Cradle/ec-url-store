SELECT
    COUNT(*) AS total_visits,
    COUNT(DISTINCT ip_address) AS unique_visitors,
    COUNT(DISTINCT country) AS countries,
    COUNT(DISTINCT user_agent) AS user_agents,
    COUNT(DISTINCT referer) AS referrers
FROM
    public.url_store_analytics_data;