INSERT INTO public.url_store_data(id, user_id, url, slug, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING $table_fields;
