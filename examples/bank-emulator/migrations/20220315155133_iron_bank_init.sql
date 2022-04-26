-- Create Account types with ranges
INSERT INTO public.bank_account_ranges
    (account_type, idx, max_number)
VALUES
    ('holding', 100000000, 9999999),
    ('loan', 200000000, 99999999),
    ('card', 300000000, 99999999),
    ('savings', 400000000, 99999999),
    ('checking', 500000000, 99999999);

-- Create Holding accounts
SELECT public.open_account(100000000, 'holding-usd', 'USD', 1000000000);
SELECT public.open_account(100000000, 'holding-cad', 'CAD', 1000000000);
SELECT public.open_account(100000000, 'holding-eur', 'EUR', 1000000000);