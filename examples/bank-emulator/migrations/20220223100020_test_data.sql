INSERT INTO public.accounts
    (status, bank_reference, tenant)
VALUES
    ('pending', '{"name": "test-account-0001", "number": "20223030001"}', 'm10-test'),
    ('open', '{"name": "test-account-0002", "number": "20223030002"}', 'm10-test'),
    ('pending', '{"name": "test-account-0003", "number": "20223030003"}', 'm10-test'),
    ('open', '{"name": "test-account-0004", "number": "20223030004"}', 'm10-test'),
    ('pending', '{"name": "test-account-0005", "number": "20223030005"}', 'm10-test'),
    ('open', '{"name": "test-account-0006", "number": "20223030006"}', 'm10-test'),
    ('pending', '{"name": "test-account-0007", "number": "20223030007"}', 'm10-test'),
    ('open', '{"name": "test-account-0008", "number": "20223030008"}', 'm10-test'),
    ('pending', '{"name": "test-account-0009", "number": "20223030009"}', 'm10-test'),
    ('open', '{"name": "test-account-0010", "number": "20223030010"}', 'm10-test'),
    ('pending', '{"name": "test-account-0011", "number": "20223030011"}', 'm10-test'),
    ('open', '{"name": "test-account-0012", "number": "20223030012"}', 'm10-test'),
    ('pending', '{"name": "test-account-0013", "number": "20223030013"}', 'm10-test'),
    ('open', '{"name": "test-account-0014", "number": "20223030014"}', 'm10-test'),
    ('pending', '{"name": "test-account-0015", "number": "20223030015"}', 'm10-test'),
    ('open', '{"name": "test-account-0016", "number": "20223030016"}', 'm10-test'),
    ('pending', '{"name": "test-account-0017", "number": "20223030017"}', 'm10-test'),
    ('closed', '{"name": "test-account-0018", "number": "20223030018"}', 'm10-test');

INSERT INTO public.contacts
    (user_id, account_id, contact_data, rbac_role, account_set, tenant)
VALUES
    ('auth0|7e570001', 1, '{"name": "test0001"}', '4275da17-60e4-424c-8dd0-b224fa08a43f', 'a036b1ed-4f9b-45c9-ab3e-6ebcd832ef51', 'm10-test'),
    ('auth0|7e570002', 2, '{"name": "test0002"}', '0fa846f5-3476-4e92-a5bd-e3aa8ba7a587', 'a5ff6631-8bc5-4221-aa26-30981125c158', 'm10-test'),
    ('auth0|7e570003', 3, '{"name": "test0003"}', 'c2548007-9f36-41d8-bc6a-702e968285d9', '98ef10c0-3fcf-4d22-9e71-db07196f76b8', 'm10-test'),
    ('auth0|7e570004', 4, '{"name": "test0004"}', 'b766c0f3-17a5-406c-a2f5-f81f4ab6c26d', '49ff138f-7ffe-4fe8-97da-f13574014348', 'm10-test'),
    ('auth0|7e570005', 5, '{"name": "test0005"}', 'df7a6138-5309-428a-8dd3-09e646035502', '5841f526-6721-4b5d-8115-d8bb134fb0f0', 'm10-test'),
    ('auth0|7e570006', 6, '{"name": "test0006"}', 'f640f0d7-f77b-4657-a151-4734d303959d', '97afb43f-3c2f-4dc6-8c16-2a8ad2b2ffca', 'm10-test'),
    ('auth0|7e570007', 7, '{"name": "test0007"}', 'b7ec3356-d7bc-4e59-9acd-c8983b431662', 'b8aa2ace-0894-436d-a74d-3a2edee43188', 'm10-test'),
    ('auth0|7e570008', 8, '{"name": "test0008"}', 'ffa709be-d240-4ae7-a86e-64352eec551b', '810eb3df-b0af-496d-a2da-141c36727f07', 'm10-test'),
    ('auth0|7e570009', 9, '{"name": "test0009"}', '31aa90c5-6c8c-475e-822c-792fd4e9679a', 'babac6ec-f720-4328-bbcf-975b2fd5e112', 'm10-test'),
    ('auth0|7e570010', 10, '{"name": "test0010"}', '97f8c8a6-4495-4699-93c0-fb48cd78f861', '97a26c09-0de8-4d6d-92c3-d987d3c624d5', 'm10-test'),
    ('auth0|7e570011', 11, '{"name": "test0011"}', 'b9893d3d-b3b0-4efa-8377-7ca6784e3eac', 'c0a87c77-a739-4d05-8271-b90c839305a3', 'm10-test'),
    ('auth0|7e570012', 12, '{"name": "test0012"}', '028a3b1b-1dcf-47b5-b788-e960625c864e', '00bffb6b-f7a6-44f2-b798-531c0c91e918', 'm10-test'),
    ('auth0|7e570013', 13, '{"name": "test0013"}', 'bd3506f4-b98a-4125-8c02-977645c2d029', '06a8c078-7ba9-45bf-ad90-e6515182ae46', 'm10-test'),
    ('auth0|7e570014', 14, '{"name": "test0014"}', 'fe87d2e4-9f10-4a9c-9471-f700812f24b2', '3f56f216-42b1-4e29-8903-8e4142f5b2b3', 'm10-test'),
    ('auth0|7e570015', 15, '{"name": "test0015"}', 'f0fc462d-90b0-40d2-8871-ac7cf6d23adb', 'd8b29e8e-1bf4-495f-97cc-11f26451aff2', 'm10-test'),
    ('auth0|7e570016', 16, '{"name": "test0016"}', '7947d459-bef7-48dd-8c6c-fd03e43349ca', 'a4076c74-2205-4dc3-ab55-a554f00d2cbf', 'm10-test'),
    ('auth0|7e570017', 17, '{"name": "test0017"}', '22edf5ff-199e-4d12-8138-c5a14a1084ae', '03b99825-9194-4a49-b869-c3b3b99f08d0', 'm10-test');

INSERT INTO public.contacts
    (user_id, account_id, contact_data, rbac_role, account_set, tenant, retired_since)
VALUES
    ('auth0|7e570018', 18, '{"name": "test0018"}', 'c0cf4c55-054e-48af-9f82-b389ce153869', '8d1d1a13-1563-44e3-bc46-4d24ab256b64', 'm10-test', CURRENT_TIMESTAMP);

INSERT INTO public.assets
    (ledger_account_id, instrument, linked_account, tenant)
VALUES
    ('\x02800001000001000000000000000003', 'usd', 1, 'm10-test'),
    ('\x02800001000003000000000000000003', 'btc', 1, 'm10-test'),
    ('\x02800001000004000000000000000003', 'usd', 2, 'm10-test'),
    ('\x02800001000005000000000000000003', 'usd', 3, 'm10-test'),
    ('\x02800001000006000000000000000003', 'usd', 4, 'm10-test'),
    ('\x02800001000007000000000000000003', 'usd', 5, 'm10-test'),
    ('\x02800001000008000000000000000003', 'usd', 6, 'm10-test'),
    ('\x02800001000009000000000000000003', 'usd', 7, 'm10-test'),
    ('\x0280000100000a000000000000000003', 'usd', 8, 'm10-test'),
    ('\x0280000100000b000000000000000003', 'usd', 9, 'm10-test'),
    ('\x0280000100000c000000000000000003', 'btc', 9, 'm10-test'),
    ('\x0280000100000d000000000000000003', 'usd', 10, 'm10-test'),
    ('\x0280000100000e000000000000000003', 'usd', 11, 'm10-test'),
    ('\x0280000100000f000000000000000003', 'btc', 11, 'm10-test'),
    ('\x02800001000010000000000000000003', 'usd', 13, 'm10-test'),
    ('\x02800001000011000000000000000003', 'usd', 14, 'm10-test'),
    ('\x02800001000012000000000000000003', 'usd', 15, 'm10-test'),
    ('\x02800001000013000000000000000003', 'btc', 15, 'm10-test'),
    ('\x02800001000014000000000000000003', 'usd', 16, 'm10-test'),
    ('\x02800001000015000000000000000003', 'usd', 17, 'm10-test'),
    ('\x02800001000016000000000000000003', 'btc', 17, 'm10-test'),
    ('\x02800001000017000000000000000003', 'usd', 18, 'm10-test');