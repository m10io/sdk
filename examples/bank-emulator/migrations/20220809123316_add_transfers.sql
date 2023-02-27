CREATE TABLE public.ledger_transfers (
    tx_id bytea PRIMARY KEY,
    target bytea NOT NULL,
    symbol text NOT NULL,
    handled boolean DEFAULT false,
    timestamp timestamp with time zone NOT NULL
);