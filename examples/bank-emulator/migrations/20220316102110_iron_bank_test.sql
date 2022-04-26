CREATE FUNCTION public.open_test_account(p_idx integer, p_name text, p_currency text, p_balance bigint)
RETURNS bigint
LANGUAGE
plpgsql
AS
$$
DECLARE
    account_id bigint;
    v_account record;
    v_bank_reference jsonb;
BEGIN
    account_id := public.new_account(p_idx, p_name, p_currency);
    UPDATE public.bank_accounts
    SET account_status = 'open',
        balance = p_balance
    WHERE id = account_id
    RETURNING * INTO v_account;

    IF NOT found THEN
        raise 'No such account index';
    END IF;

    UPDATE public.accounts
    SET bank_reference = jsonb_build_object (
        'id', account_id,
        'number', v_account.account_number,
        'name', v_account.display_name
    )
    WHERE id = 19;

    return account_id;    
END;
$$;

SELECT public.open_test_account(500000000, 'default-test-account', 'USD', 100000);

DROP FUNCTION public.open_test_account;

