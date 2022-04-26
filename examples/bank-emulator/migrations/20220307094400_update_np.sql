ALTER TABLE public.notification_preferences
    DROP CONSTRAINT notification_preferences_asset_id_key;

ALTER TABLE public.notification_preferences
    ADD UNIQUE (device_token, asset_id);    