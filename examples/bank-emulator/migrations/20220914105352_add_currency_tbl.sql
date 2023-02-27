CREATE TABLE public.currencies (
    code text NOT NULL UNIQUE,
    bank_id bytea,
    regulated_account bytea,
    cbdc_account bytea,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);