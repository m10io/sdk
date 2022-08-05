CREATE TYPE asset_type as ENUM('regulated', 'cbdc', 'indirect_cbdc');

ALTER TABLE public.assets
    ADD asset_type asset_type NOT NULL DEFAULT 'regulated';

UPDATE public.assets set asset_type = 'regulated';