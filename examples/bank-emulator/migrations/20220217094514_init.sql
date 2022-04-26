CREATE TYPE account_status as ENUM('pending', 'open', 'pending_closure', 'closed');
CREATE TYPE contact_type as ENUM('individual', 'legal_entity', 'employee');

CREATE TABLE public.accounts (
    id BIGSERIAL PRIMARY KEY,
    status account_status DEFAULT 'pending' NOT NULL,
    bank_reference JSONB,
    tenant text NOT NULL DEFAULT 'm10',
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);

CREATE TABLE public.assets (
    id BIGSERIAL PRIMARY KEY,
    ledger_account_id bytea NOT NULL,
    instrument text NOT NULL,
    linked_account bigint NOT NULL REFERENCES public.accounts (id),
    tenant text NOT NULL DEFAULT 'm10',
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);

CREATE INDEX assets_linked_account ON public.assets USING btree (linked_account);
CREATE INDEX assets_ledger_account_id ON public.assets USING btree (ledger_account_id);

CREATE TABLE public.contacts (
    id BIGSERIAL PRIMARY KEY,
    user_id text NOT NULL UNIQUE,
    co_owner text,
    account_id bigint REFERENCES public.accounts(id) ON DELETE SET NULL,
    contact_data JSONB NOT NULL,
    bank_reference JSONB,
    contact_type contact_type NOT NULL DEFAULT 'individual',
    relationship bigint REFERENCES public.contacts(id) ON DELETE SET NULL,
    rbac_role UUID NOT NULL UNIQUE,
    account_set UUID NOT NULL UNIQUE,
    tenant text NOT NULL DEFAULT 'm10',
    retired_since timestamp with time zone,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);

CREATE INDEX contacts_user_id ON public.contacts USING btree (user_id);

CREATE TABLE public.notification_preferences (
    id SERIAL PRIMARY KEY,
    device_token text NOT NULL,
    notification_toggles integer NOT NULL,
    contacts_id bigint NOT NULL REFERENCES public.contacts (id) ON DELETE CASCADE,
    asset_id bigint UNIQUE NOT NULL REFERENCES public.assets (id),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);

-- adapted from https://brandur.org/idempotency-keys#calling-stripe
CREATE TABLE idempotency_keys (
    idempotency_key UUID PRIMARY KEY,
    created_at      timestamp with time zone NOT NULL DEFAULT now(),
    last_run_at     timestamp with time zone NOT NULL DEFAULT now(),
    locked_at       timestamp with time zone DEFAULT now(),

    request_hash    BYTEA NOT NULL,

    -- for finished requests, stored status code and body
    response_code   INT NULL,
    response_type   TEXT NULL,
    response_body   BYTEA NULL,

    recovery_point  VARCHAR(50) NOT NULL
);
