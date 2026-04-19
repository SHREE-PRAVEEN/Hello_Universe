



-- =======================================================
-- USERS
-- =======================================================
CREATE INDEX idx_users_email          ON users(email) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_username       ON users(username) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_status         ON users(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_search_vector  ON users USING GIN(search_vector);
CREATE INDEX idx_users_created_at     ON users(created_at);

-- =======================================================
-- SESSIONS
-- =======================================================
CREATE INDEX idx_sessions_user_id     ON sessions(user_id) WHERE is_revoked = FALSE;
CREATE INDEX idx_sessions_expires     ON sessions(expires_at) WHERE is_revoked = FALSE;

-- =======================================================
-- ORGANIZATIONS
-- =======================================================
CREATE INDEX idx_orgs_slug            ON organizations(slug) WHERE deleted_at IS NULL;
CREATE INDEX idx_orgs_owner           ON organizations(owner_id);
CREATE INDEX idx_orgs_search_vector   ON organizations USING GIN(search_vector);

CREATE INDEX idx_org_members_user     ON organization_members(user_id) WHERE is_active = TRUE;
CREATE INDEX idx_org_members_org      ON organization_members(organization_id) WHERE is_active = TRUE;

-- =======================================================
-- PROJECTS
-- =======================================================
CREATE INDEX idx_projects_owner       ON projects(owner_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_org         ON projects(organization_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_status      ON projects(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_visibility  ON projects(visibility) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_category    ON projects(category_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_slug        ON projects(slug) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_featured    ON projects(featured, created_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_access      ON projects(access_type) WHERE deleted_at IS NULL;
CREATE INDEX idx_projects_downloads   ON projects(download_count DESC) WHERE status = 'published';
CREATE INDEX idx_projects_search_vec  ON projects USING GIN(search_vector);
CREATE INDEX idx_projects_created_at  ON projects(created_at DESC);

CREATE INDEX idx_project_versions_proj  ON project_versions(project_id, created_at DESC);
CREATE INDEX idx_project_collab_user    ON project_collaborators(user_id);
CREATE INDEX idx_project_collab_proj    ON project_collaborators(project_id);
CREATE INDEX idx_project_tags_tag       ON project_tags(tag_id);
CREATE INDEX idx_project_tags_proj      ON project_tags(project_id);

-- =======================================================
-- MEDIA FILES
-- =======================================================
CREATE INDEX idx_media_project        ON media_files(project_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_media_uploader       ON media_files(uploaded_by) WHERE deleted_at IS NULL;
CREATE INDEX idx_media_type           ON media_files(media_type) WHERE deleted_at IS NULL;
CREATE INDEX idx_media_checksum       ON media_files(checksum_sha256);
CREATE INDEX idx_media_created        ON media_files(created_at DESC);
CREATE INDEX idx_media_search_vec     ON media_files USING GIN(search_vector);

CREATE INDEX idx_storage_media        ON media_storage_locations(media_file_id);
CREATE INDEX idx_storage_backend      ON media_storage_locations(backend);

-- =======================================================
-- AI & SEARCH
-- =======================================================
CREATE INDEX idx_ai_tags_entity       ON ai_tags(entity_type, entity_id);
CREATE INDEX idx_ai_tags_tag_name     ON ai_tags(tag_name);
CREATE INDEX idx_ai_tags_confidence   ON ai_tags(confidence DESC);

CREATE INDEX idx_search_index_vector  ON search_index USING GIN(search_vector);
CREATE INDEX idx_search_index_type    ON search_index(entity_type);
CREATE INDEX idx_search_index_rank    ON search_index(rank_score DESC);

CREATE INDEX idx_search_queries_user  ON search_queries(user_id, created_at DESC);
CREATE INDEX idx_search_queries_text  ON search_queries USING GIN(to_tsvector('english', query_text));
CREATE INDEX idx_search_queries_date  ON search_queries(created_at DESC);

-- =======================================================
-- OWNERSHIP / BLOCKCHAIN
-- =======================================================
CREATE INDEX idx_content_hashes_entity ON content_hashes(entity_type, entity_id);
CREATE INDEX idx_ownership_entity      ON ownership_records(entity_type, entity_id);
CREATE INDEX idx_ownership_user        ON ownership_records(owner_user_id);
CREATE INDEX idx_ownership_org         ON ownership_records(owner_org_id);
CREATE INDEX idx_blockchain_entity     ON blockchain_transactions(entity_type, entity_id);
CREATE INDEX idx_blockchain_txhash     ON blockchain_transactions(tx_hash);
CREATE INDEX idx_ipfs_cid              ON ipfs_records(cid);

-- =======================================================
-- ENTITLEMENTS
-- =======================================================
CREATE INDEX idx_entitlements_user    ON content_entitlements(user_id, entity_type, entity_id)
    WHERE revoked_at IS NULL;
CREATE INDEX idx_entitlements_entity  ON content_entitlements(entity_type, entity_id)
    WHERE revoked_at IS NULL;

-- =======================================================
-- COMMERCE
-- =======================================================
CREATE INDEX idx_purchases_user       ON purchases(user_id, created_at DESC);
CREATE INDEX idx_purchases_entity     ON purchases(entity_type, entity_id);
CREATE INDEX idx_purchases_status     ON purchases(status);
CREATE INDEX idx_subscriptions_user   ON subscriptions(user_id) WHERE status = 'active';
CREATE INDEX idx_subscriptions_org    ON subscriptions(organization_id) WHERE status = 'active';
CREATE INDEX idx_subscriptions_period ON subscriptions(current_period_end) WHERE status = 'active';
CREATE INDEX idx_revenue_splits_recip ON revenue_splits(recipient_id, created_at DESC);

-- =======================================================
-- ANALYTICS (partitioned tables — indexes on each partition)
-- =======================================================
CREATE INDEX idx_pv_project_date      ON project_views(project_id, viewed_at DESC);
CREATE INDEX idx_pv_user              ON project_views(user_id, viewed_at DESC);

CREATE INDEX idx_mv_media_date        ON media_views(media_file_id, viewed_at DESC);
CREATE INDEX idx_mv_user              ON media_views(user_id, viewed_at DESC);

CREATE INDEX idx_dl_entity_date       ON downloads(entity_type, entity_id, downloaded_at DESC);
CREATE INDEX idx_dl_user_date         ON downloads(user_id, downloaded_at DESC);

CREATE INDEX idx_eng_entity           ON engagement_events(entity_type, entity_id, created_at DESC);
CREATE INDEX idx_eng_user             ON engagement_events(user_id, event_type, created_at DESC);

CREATE INDEX idx_activity_user        ON activity_logs(user_id, created_at DESC);
CREATE INDEX idx_activity_entity      ON activity_logs(entity_type, entity_id, created_at DESC);
CREATE INDEX idx_activity_action      ON activity_logs(action, created_at DESC);

-- =======================================================
-- ENGAGEMENT
-- =======================================================
CREATE INDEX idx_reviews_entity       ON reviews(entity_type, entity_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_reviews_user         ON reviews(user_id) WHERE deleted_at IS NULL;

CREATE INDEX idx_comments_entity      ON comments(entity_type, entity_id, created_at DESC)
    WHERE deleted_at IS NULL;
CREATE INDEX idx_comments_parent      ON comments(parent_id) WHERE parent_id IS NOT NULL;

CREATE INDEX idx_favorites_user       ON favorites(user_id, entity_type);
CREATE INDEX idx_favorites_entity     ON favorites(entity_type, entity_id);

CREATE INDEX idx_follows_follower     ON follows(follower_id, entity_type);
CREATE INDEX idx_follows_entity       ON follows(entity_type, entity_id);

CREATE INDEX idx_reports_entity       ON reports(entity_type, entity_id);
CREATE INDEX idx_reports_status       ON reports(status) WHERE status = 'pending';

-- =======================================================
-- MODERATION & AUDIT
-- =======================================================
CREATE INDEX idx_modq_status          ON moderation_queue(status, priority DESC)
    WHERE status = 'pending';
CREATE INDEX idx_modq_entity          ON moderation_queue(entity_type, entity_id);
CREATE INDEX idx_modq_assigned        ON moderation_queue(assigned_to) WHERE status = 'pending';

CREATE INDEX idx_audit_actor          ON audit_logs(actor_id, created_at DESC);
CREATE INDEX idx_audit_entity         ON audit_logs(entity_type, entity_id, created_at DESC);
CREATE INDEX idx_audit_action         ON audit_logs(action, created_at DESC);
CREATE INDEX idx_audit_org            ON audit_logs(organization_id, created_at DESC);

CREATE INDEX idx_notifications_user   ON notifications(user_id, is_read, created_at DESC);

-- =======================================================
-- TAGS
-- =======================================================
CREATE INDEX idx_tags_slug            ON tags(slug);
CREATE INDEX idx_tags_use_count       ON tags(use_count DESC);
CREATE INDEX idx_tags_name_trgm       ON tags USING GIN(name gin_trgm_ops);  -- fuzzy tag search




