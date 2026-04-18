use crate::models::ai::*;
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::errors::AppResult;

pub struct SearchRepo;

impl SearchRepo {
    pub async fn full_text_search(
        pool: &PgPool,
        query: &str,
        entity_types: Option<&[String]>,
        limit: i64,
        offset: i64,
    ) -> AppResult<(Vec<SearchResult>, i64)> {
        let results = sqlx::query_as!(
            SearchResult,
            r#"SELECT entity_type, entity_id, title,
               ts_rank(search_vector, plainto_tsquery('english', $1))::FLOAT4 AS "rank_score!: f32",
               metadata
               FROM search_index
               WHERE search_vector @@ plainto_tsquery('english', $1)
                 AND ($2::TEXT[] IS NULL OR entity_type = ANY($2))
               ORDER BY rank_score DESC
               LIMIT $3 OFFSET $4"#,
            query,
            entity_types as _,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM search_index
               WHERE search_vector @@ plainto_tsquery('english', $1)
                 AND ($2::TEXT[] IS NULL OR entity_type = ANY($2))"#,
            query,
            entity_types as _
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        Ok((results, total))
    }

    pub async fn upsert_search_index(
        pool: &PgPool,
        entity_type: &str,
        entity_id: Uuid,
        title: &str,
        body: Option<&str>,
        tags: Option<&[String]>,
        metadata: &serde_json::Value,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"INSERT INTO search_index (entity_type, entity_id, title, body, tags, search_vector, metadata)
               VALUES ($1, $2, $3, $4, $5,
                 setweight(to_tsvector('english', $3), 'A') ||
                 setweight(to_tsvector('english', COALESCE($4, '')), 'C'),
                 $6)
               ON CONFLICT (entity_type, entity_id) DO UPDATE SET
                 title = EXCLUDED.title,
                 body = EXCLUDED.body,
                 tags = EXCLUDED.tags,
                 search_vector = EXCLUDED.search_vector,
                 metadata = EXCLUDED.metadata,
                 updated_at = NOW()"#,
            entity_type, entity_id, title, body, tags as _, metadata
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_ai_tags(pool: &PgPool, entity_type: &str, entity_id: Uuid) -> AppResult<Vec<AiTag>> {
        Ok(sqlx::query_as!(
            AiTag,
            r#"SELECT id, entity_type, entity_id, tag_name, confidence,
               model_name, model_version, raw_response, created_at
               FROM ai_tags WHERE entity_type=$1 AND entity_id=$2
               ORDER BY confidence DESC"#,
            entity_type, entity_id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn upsert_ai_tags(
        pool: &PgPool,
        entity_type: &str,
        entity_id: Uuid,
        tags: &[AiTagItem],
        model_name: &str,
    ) -> AppResult<()> {
        for tag in tags {
            sqlx::query!(
                r#"INSERT INTO ai_tags (entity_type, entity_id, tag_name, confidence, model_name)
                   VALUES ($1,$2,$3,$4,$5)
                   ON CONFLICT (entity_type, entity_id, tag_name) DO UPDATE
                   SET confidence = EXCLUDED.confidence, model_name = EXCLUDED.model_name"#,
                entity_type, entity_id, tag.tag, bigdecimal::BigDecimal::try_from(tag.confidence).unwrap(), model_name
            )
            .execute(pool)
            .await?;
        }
        Ok(())
    }

    pub async fn autocomplete_tags(pool: &PgPool, prefix: &str, limit: i64) -> AppResult<Vec<String>> {
        Ok(sqlx::query_scalar!(
            r#"SELECT name FROM tags
               WHERE name ILIKE $1 || '%'
               ORDER BY use_count DESC LIMIT $2"#,
            prefix, limit
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .collect())
    }

    pub async fn popular_searches(pool: &PgPool, limit: i64) -> AppResult<Vec<String>> {
        Ok(sqlx::query_scalar!(
            r#"SELECT query_text FROM search_queries
               WHERE created_at > NOW() - INTERVAL '7 days'
               GROUP BY query_text ORDER BY COUNT(*) DESC LIMIT $1"#,
            limit
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .collect())
    }
}
