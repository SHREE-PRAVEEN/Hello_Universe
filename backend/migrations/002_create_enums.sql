-- Enums
CREATE TYPE user_status AS ENUM ('active', 'suspended', 'banned', 'pending_verification', 'deleted');
CREATE TYPE org_type AS ENUM ('company', 'institute', 'lab', 'team', 'individual', 'government');
CREATE TYPE project_status AS ENUM ('draft', 'pending_review', 'approved', 'published', 'rejected', 'archived');
CREATE TYPE visibility AS ENUM ('public', 'private', 'premium', 'organization');
CREATE TYPE media_type AS ENUM ('image', 'video', 'document', 'pdf', 'cad', 'design', 'code', 'dataset', 'model', 'archive', 'other');
CREATE TYPE storage_backend AS ENUM ('local', 's3', 'gcs', 'azure_blob', 'ipfs', 'r2');
CREATE TYPE content_access AS ENUM ('free', 'premium', 'paid', 'subscription', 'organization_only');
CREATE TYPE license_type AS ENUM ('open_source', 'creative_commons', 'commercial', 'educational', 'proprietary', 'custom');
CREATE TYPE moderation_status AS ENUM ('pending', 'approved', 'rejected', 'flagged', 'escalated');
CREATE TYPE transaction_status AS ENUM ('pending', 'completed', 'failed', 'refunded', 'disputed');
CREATE TYPE subscription_status AS ENUM ('active', 'cancelled', 'expired', 'past_due', 'trialing');
CREATE TYPE notification_type AS ENUM ('system', 'comment', 'review', 'purchase', 'moderation', 'follow', 'collaboration', 'alert');
CREATE TYPE permission_scope AS ENUM ('platform', 'organization', 'project');