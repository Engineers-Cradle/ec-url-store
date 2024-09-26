INSERT INTO public.url_store_analytics_data(id, url_id, ip_address, country, user_agent, referer, created_at)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING $table_fields;
