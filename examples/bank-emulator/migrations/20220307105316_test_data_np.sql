INSERT INTO public.notification_preferences
    (device_token, notification_toggles, contacts_id, asset_id)
VALUES
    ('android_1', b'000110'::int, 1, 1),
    ('ios_1', b'100110'::int, 1, 1),
    ('android_1', b'000110'::int, 1, 2),
    ('ios_1', b'100110'::int, 1, 2),
    ('android_3', b'000110'::int, 2, 3),
    ('ios_3', b'100110'::int, 3, 4),
    ('android_4', b'000110'::int, 4, 5),
    ('ios_4', b'100110'::int, 5, 6),
    ('android_5', b'000110'::int, 6, 7),
    ('ios_5', b'100110'::int, 7, 8),
    ('android_6', b'000110'::int, 8, 9),
    ('ios_6', b'000110'::int, 9, 10),
    ('ios_7', b'100110'::int, 9, 11),
    ('android_7', b'000110'::int, 10, 12),
    ('ios_8', b'100110'::int, 11, 13),
    ('ios_9', b'100110'::int, 11, 14),
    ('android_8', b'000110'::int, 13, 15),
    ('ios_10', b'000110'::int, 14, 16),
    ('android_9', b'000110'::int, 15, 17),
    ('ios_11', b'000110'::int, 15, 18),
    ('ios_12', b'000110'::int, 16, 19),
    ('android_10', b'000110'::int, 17, 20),
    ('ios_13', b'000110'::int, 17, 21),
    ('android_11', b'000110'::int, 18, 22);

-- Entries for default user
INSERT INTO public.accounts
    (status, bank_reference, tenant)
VALUES
    ('open', '{"name": "default-test-account", "number": "20223030019"}', 'm10-test');

INSERT INTO public.contacts
    (user_id, account_id, contact_data, rbac_role, account_set, tenant)
VALUES
    ('auth0|6227946b65653d0068e10403', 19, '{"name": "prepop", "email": "omega-prepopulated-user@m10test.io"}', '7e1c8a82-3aed-44a3-83bc-5a7a487299aa', '864683c2-a3d1-4384-907e-f59802e451cf', 'm10-test');

INSERT INTO public.assets
    (ledger_account_id, instrument, linked_account, tenant)
VALUES
    ('\x02800001000018000000000000000003', 'usd', 19, 'm10-test'),
    ('\x02800001000019000000000000000003', 'btc', 19, 'm10-test');

INSERT INTO public.notification_preferences
    (device_token, notification_toggles, contacts_id, asset_id)
VALUES
    ('android_12', b'000110'::int, 19, 23),
    ('ios_14', b'100110'::int, 19, 23),
    ('android_12', b'000110'::int, 19, 24),
    ('ios_14', b'100110'::int, 19, 24);