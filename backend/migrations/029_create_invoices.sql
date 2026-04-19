-- -------------------------------------------------------
-- INVOICES
-- -------------------------------------------------------
CREATE TABLE invoices (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL REFERENCES users(id),
    invoice_number  VARCHAR(50) NOT NULL UNIQUE,   -- human-readable INV-2024-00001
    subtotal        NUMERIC(12,2) NOT NULL,
    tax_amount      NUMERIC(12,2) NOT NULL DEFAULT 0,
    discount_amount NUMERIC(12,2) NOT NULL DEFAULT 0,
    total           NUMERIC(12,2) NOT NULL,
    currency        CHAR(3) NOT NULL DEFAULT 'USD',
    status          VARCHAR(20) NOT NULL DEFAULT 'draft',   -- 'draft', 'issued', 'paid', 'void'
    issued_at       TIMESTAMPTZ,
    due_at          TIMESTAMPTZ,
    paid_at         TIMESTAMPTZ,
    billing_address JSONB NOT NULL DEFAULT '{}',
    line_items      JSONB NOT NULL DEFAULT '[]',   -- snapshot of purchased items
    pdf_url         TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
