SELECT
    COUNT(DISTINCT url_store_data.user_id) AS total_users,
    COUNT(*) AS total_urls
FROM
    public.url_store_data;