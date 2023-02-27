CREATE TYPE transfer_handler as ENUM('cbdc_limits', 'cbdc_reserves', 'drc_reserves');

ALTER TABLE public.ledger_transfers
    ADD COLUMN handler transfer_handler NOT NULL DEFAULT 'cbdc_limits';

UPDATE public.ledger_transfers set handler = 'cbdc_limits';