ALTER TABLE public.ledger_transfers
    DROP CONSTRAINT ledger_transfers_pkey;

ALTER TABLE public.ledger_transfers
    ADD UNIQUE (handler, tx_id);
