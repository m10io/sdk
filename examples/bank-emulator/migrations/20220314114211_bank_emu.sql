CREATE TYPE bank_account_type as ENUM('checking', 'savings', 'loan', 'card', 'holding');
CREATE TYPE bank_account_status as ENUM('pending', 'open', 'pending_closure', 'closed');
CREATE TYPE bank_contact_type as ENUM('individual', 'legal_entity');
CREATE TYPE bank_contact_status as ENUM('pending', 'approved', 'denied', 'frozen', 'retired');
CREATE TYPE bank_document_type as ENUM('id', 'proof_of_address', 'proof_of_formation', 'proof_of_ownership');
CREATE TYPE bank_document_status as ENUM('pending', 'verified', 'rejected');
CREATE TYPE bank_transaction_type as ENUM('credit', 'debit');
CREATE TYPE bank_transaction_status as ENUM('pending', 'settled', 'en_route', 'canceled');
CREATE TYPE bank_transaction_data AS (
    account bigint,
    account_entry bigint,
    other_account bigint,
    initial_status bank_transaction_status
); 

CREATE TABLE public.bank_accounts (
    id BIGSERIAL PRIMARY KEY,
    account_status bank_account_status DEFAULT 'pending' NOT NULL,
    account_type bank_account_type DEFAULT 'checking' NOT NULL,
    account_number integer NOT NULL UNIQUE,
    iban text,
    balance bigint DEFAULT 0,
    currency text,
    display_name text,
    next_entry bigint DEFAULT 1,
    allowed_overdraft bigint DEFAULT 0,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);

CREATE TABLE public.bank_account_ranges (
    id SERIAL PRIMARY KEY,
    account_type bank_account_type NOT NULL,
    idx integer NOT NULL UNIQUE,
    max_number integer NOT NULL,
    last_used_number integer DEFAULT 0
);

CREATE TABLE public.bank_contacts (
    id BIGSERIAL PRIMARY KEY,
    account bigint REFERENCES public.bank_accounts(id),
    contact_type bank_contact_type NOT NULL DEFAULT 'individual',
    contact_status bank_contact_status DEFAULT 'pending' NOT NULL,
    data JSONB,
    payment_methods JSONB,
    issues JSONB,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);

CREATE TABLE public.bank_support_documents (
    id BIGSERIAL PRIMARY KEY,
    contact bigint REFERENCES public.bank_contacts(id) ON DELETE SET NULL,
    document_type bank_document_type NOT NULL,
    document_status bank_document_status DEFAULT 'pending' NOT NULL,
    front_uri text NOT NULL,
    back_uri text,
    issues JSONB,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);

CREATE TABLE public.bank_transfers (
    txn_id uuid PRIMARY KEY,
    reference text,
    amount bigint NOT NULL,
    routing jsonb,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE public.bank_transactions (
    txn_id uuid NOT NULL REFERENCES public.bank_transfers,
    account bigint NOT NULL REFERENCES public.bank_accounts(id),
    account_entry bigint NOT NULL,
    other_account bigint NOT NULL REFERENCES public.bank_accounts(id),
    transaction_type bank_transaction_type NOT NULL,
    transaction_status bank_transaction_status DEFAULT 'pending' NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);

CREATE FUNCTION public.new_account(p_idx integer, p_name text, p_currency text)
RETURNS bigint
LANGUAGE
plpgsql
AS
$$
DECLARE
    v_account_type record;
    v_account record;
    v_account_number integer;
BEGIN
    SELECT *
    INTO v_account_type
    FROM public.bank_account_ranges
    WHERE idx = p_idx;

    IF NOT found THEN
        raise 'No such account index';
    END IF;

    IF v_account_type.last_used_number = v_account_type.max_number THEN
        raise 'No numbers available';
    END IF;

    v_account_number := v_account_type.idx + v_account_type.last_used_number + 1;
    
    INSERT INTO public.bank_accounts (
        account_type, account_number, currency, display_name
    )
    VALUES
        (v_account_type.account_type, v_account_number, p_currency, p_name)
    RETURNING * INTO v_account;

    UPDATE public.bank_account_ranges
    SET last_used_number = v_account_type.last_used_number + 1
    WHERE idx = p_idx;

    return v_account.id;
END;
$$;

CREATE FUNCTION public.open_account(p_idx integer, p_name text, p_currency text, p_balance bigint)
RETURNS bigint
LANGUAGE
plpgsql
AS
$$
DECLARE
    account_id bigint;
BEGIN
    account_id := public.new_account(p_idx, p_name, p_currency);
    UPDATE public.bank_accounts
    SET account_status = 'open',
        balance = p_balance
    WHERE id = account_id;

    IF NOT found THEN
        raise 'No such account index';
    END IF;

    return account_id;    
END;
$$;

CREATE FUNCTION public.new_transfer(
    p_txn_id uuid,
    p_amount bigint,
    p_reference text,
    p_routing jsonb,
    p_debit bank_transaction_data,
    p_credit bank_transaction_data
)
RETURNS uuid
LANGUAGE
plpgsql
AS
$$
BEGIN
    INSERT INTO public.bank_transfers (
        txn_id, reference, amount, routing
    ) VALUES
        (p_txn_id, p_reference, p_amount, p_routing);

    IF p_debit IS NOT NULL THEN
        INSERT INTO public.bank_transactions (
            txn_id, account, account_entry, other_account, transaction_type, transaction_status
        ) VALUES
            (p_txn_id, p_debit.account, p_debit.account_entry, p_debit.other_account, 'debit', p_debit.initial_status);
    END IF;

    IF p_credit IS NOT NULL THEN
        INSERT INTO public.bank_transactions (
            txn_id, account, account_entry, other_account, transaction_type, transaction_status
        ) VALUES
            (p_txn_id, p_credit.account, p_credit.account_entry, p_credit.other_account, 'credit', p_credit.initial_status);
    END IF;

    return p_txn_id;
END;
$$;

CREATE FUNCTION public.instant_transfer(p_txn_id uuid, p_from_account bigint, p_to_account bigint, p_amount bigint, p_reference text)
RETURNS uuid
LANGUAGE
plpgsql
AS
$$
DECLARE
    source_account record;
    target_account record;
    debit_txn bank_transaction_data;
    credit_txn bank_transaction_data;
BEGIN
    -- check source account conditions
    SELECT *
    INTO source_account
    FROM public.bank_accounts
    WHERE id = p_from_account;
   
    IF NOT found THEN
        raise 'Source account with id % not found', p_from_account;
    END IF;

    IF NOT (source_account.account_status = 'open') THEN
        raise 'Source account not open';
    END IF;

    IF (source_account.balance + source_account.allowed_overdraft) < p_amount THEN
        raise 'Insufficient funds';
    END IF;

    -- check target account conditions
    SELECT *
    INTO target_account
    FROM public.bank_accounts
    WHERE id = p_to_account;

    IF NOT found THEN
        raise 'Target account with id % not found', p_to_account;
    END IF;

    IF NOT (target_account.account_status = 'open' OR target_account.account_status = 'pending') THEN
        raise 'Target account closed';
    END IF;

    IF NOT source_account.currency = target_account.currency THEN
        raise 'Accounts are for different currencies';
    END IF;

    -- log transfer
    debit_txn.account := p_from_account;
    debit_txn.account_entry := source_account.next_entry + 1;
    debit_txn.other_account := p_to_account;
    debit_txn.initial_status := 'settled';

    credit_txn.account := p_to_account;
    credit_txn.account_entry := target_account.next_entry + 1;
    credit_txn.other_account := p_from_account;
    credit_txn.initial_status := 'settled';

    PERFORM new_transfer(p_txn_id, p_amount, p_reference, jsonb_build_object('type', 'internal'), debit_txn, credit_txn);

    -- debit source account
    UPDATE public.bank_accounts
    SET next_entry = next_entry + 1,
        balance = balance - p_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = p_from_account;

    -- credit target account
    UPDATE public.bank_accounts
    SET next_entry = next_entry + 1,
        balance = balance + p_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = p_to_account;

    return p_txn_id;
END;
$$;

CREATE FUNCTION public.withdraw(p_txn_id uuid, p_from_account bigint, p_to_account bigint, p_amount bigint, p_reference text, p_routing jsonb)
RETURNS uuid
LANGUAGE
plpgsql
AS
$$
DECLARE
    source_account record;
    target_account record;
    debit_txn bank_transaction_data;
    credit_txn bank_transaction_data;
BEGIN
    -- check source account conditions
    SELECT *
    INTO source_account
    FROM public.bank_accounts
    WHERE id = p_from_account;
   
    IF NOT found THEN
        raise 'Source account with id % not found', p_from_account;
    END IF;

    IF NOT (source_account.account_status = 'open') THEN
        raise 'Source account not open';
    END IF;

    IF (source_account.balance + source_account.allowed_overdraft) < p_amount THEN
        raise 'Insufficient funds';
    END IF;

    -- check target account conditions
    SELECT *
    INTO target_account
    FROM public.bank_accounts
    WHERE id = p_to_account;

    IF NOT found THEN
        raise 'Target account with id % not found', p_to_account;
    END IF;

    IF NOT (target_account.account_status = 'open' OR target_account.account_status = 'pending') THEN
        raise 'Target account closed';
    END IF;

    IF NOT (target_account.account_type = 'holding') THEN
        raise 'Target account is not a holding account';
    END IF;

    IF NOT source_account.currency = target_account.currency THEN
        raise 'Accounts are for different currencies';
    END IF;

    -- log transfers
    debit_txn.account := p_from_account;
    debit_txn.account_entry := source_account.next_entry + 1;
    debit_txn.other_account := p_to_account;
    debit_txn.initial_status := 'pending';

    credit_txn.account := p_to_account;
    credit_txn.account_entry := target_account.next_entry + 1;
    credit_txn.other_account := p_from_account;
    credit_txn.initial_status := 'en_route';

    PERFORM new_transfer(p_txn_id, p_amount, p_reference, p_routing, debit_txn, credit_txn);

    -- reserve amount from source account
    UPDATE public.bank_accounts
    SET next_entry = next_entry + 1,
        balance = balance - p_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = p_from_account;

    -- credit holding account
    UPDATE public.bank_accounts
    SET next_entry = next_entry + 1,
        balance = balance + p_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = p_to_account;

    return p_txn_id;
END;
$$;

CREATE FUNCTION public.withdraw_with_contact_method(p_txn_id uuid, p_contact bigint, p_to_account bigint, p_amount bigint, p_reference text)
RETURNS uuid
LANGUAGE
plpgsql
AS
$$
DECLARE
    contact record;
    source_account record;
    target_account record;
    debit_txn bank_transaction_data;
    credit_txn bank_transaction_data;
BEGIN
    -- fetch contact
    SELECT *
    INTO contact
    FROM public.bank_contact
    WHERE id = p_from_account;

    IF NOT found THEN
        raise 'Contact with id % not found', p_contact;
    END IF;

    IF NOT (contact.contact_status = 'approved') THEN
        raise 'Contact %', contact.contact_status;
    END IF;

    -- check source account conditions
    SELECT *
    INTO source_account
    FROM public.bank_accounts
    WHERE id = contact.account;
   
    IF NOT found THEN
        raise 'Source account with id % not found', p_from_account;
    END IF;

    IF NOT (source_account.account_status = 'open') THEN
        raise 'Source account not open';
    END IF;

    IF (source_account.balance + source_account.allowed_overdraft) < p_amount THEN
        raise 'Insufficient funds';
    END IF;

    -- check target account conditions
    SELECT *
    INTO target_account
    FROM public.bank_accounts
    WHERE id = p_to_account;

    IF NOT found THEN
        raise 'Target account with id % not found', p_to_account;
    END IF;

    IF NOT (target_account.account_status = 'open' OR target_account.account_status = 'pending') THEN
        raise 'Target account closed';
    END IF;

    IF NOT (target_account.account_type = 'holding') THEN
        raise 'Target account is not a holding account';
    END IF;

    IF NOT source_account.currency = target_account.currency THEN
        raise 'Accounts are for different currencies';
    END IF;

    -- log transfers
    debit_txn.account := p_from_account;
    debit_txn.account_entry := source_account.next_entry + 1;
    debit_txn.other_account := p_to_account;
    debit_txn.initial_status := 'pending';

    credit_txn.account := p_to_account;
    credit_txn.account_entry := target_account.next_entry + 1;
    credit_txn.other_account := p_from_account;
    credit_txn.initial_status := 'en_route';

    PERFORM new_transfer(p_txn_id, p_amount, p_reference, contact.payment_methods, debit_txn, credit_txn);

    -- reserve amount from source account
    UPDATE public.bank_accounts
    SET next_entry = next_entry + 1,
        balance = balance - p_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = p_from_account;

    -- credit holding account
    UPDATE public.bank_accounts
    SET next_entry = next_entry + 1,
        balance = balance + p_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = p_to_account;

    return p_txn_id;
END;
$$;

CREATE FUNCTION public.settle_withdraw(p_txn_id uuid)
RETURNS bigint
LANGUAGE
plpgsql
AS
$$
DECLARE
    credit_account bigint;
    withdraw_amount bigint;
BEGIN
    -- update debit transaction status
    UPDATE public.bank_transactions
    SET transaction_status = 'settled',
        updated_at = CURRENT_TIMESTAMP
    WHERE txn_id = p_txn_id AND transaction_type = 'debit' AND transaction_status = 'pending';

    IF NOT found THEN
        raise 'No debit transaction found';
    END IF;

    -- update credit transaction status
    UPDATE public.bank_transactions
    SET transaction_status = 'settled',
        updated_at = CURRENT_TIMESTAMP
    WHERE txn_id = p_txn_id AND transaction_type = 'credit' AND transaction_status = 'en_route'
    RETURNING account INTO credit_account;

    IF NOT found THEN
        raise 'No credit transaction found';
    END IF;

    SELECT amount
    INTO withdraw_amount
    FROM public.bank_transfers
    WHERE txn_id = p_txn_id;

    IF NOT found THEN
        raise 'No transfer found';
    END IF;

    -- Simulated taking money out
    UPDATE public.bank_accounts
    SET balance = balance - withdraw_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = credit_account;

    return withdraw_amount;
END;
$$;

CREATE FUNCTION public.reverse_withdraw(p_txn_id uuid)
RETURNS bigint
LANGUAGE
plpgsql
AS
$$
DECLARE
    credit_transaction record;
    withdraw_amount bigint;
BEGIN
    -- update debit transaction status
    UPDATE public.bank_transactions
    SET transaction_status = 'canceled',
        updated_at = CURRENT_TIMESTAMP
    WHERE txn_id = p_txn_id AND transaction_type = 'debit' AND transaction_status = 'pending';

    IF NOT found THEN
        raise 'No debit transaction found';
    END IF;

    -- update credit transaction status
    UPDATE public.bank_transactions
    SET transaction_status = 'canceled',
        updated_at = CURRENT_TIMESTAMP
    WHERE txn_id = p_txn_id AND transaction_type = 'credit' AND transaction_status = 'en_route'
    RETURNING * INTO credit_transaction;

    IF NOT found THEN
        raise 'No credit transaction found';
    END IF;

    SELECT amount
    INTO withdraw_amount
    FROM public.bank_transfers
    WHERE txn_id = p_txn_id;

    IF NOT found THEN
        raise 'No transfer found';
    END IF;

    -- Reverse transfer for source account
    UPDATE public.bank_accounts
    SET balance = balance + withdraw_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = credit_transaction.other_account;

    -- Reverse transfer for holding account
    UPDATE public.bank_accounts
    SET balance = balance - withdraw_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = credit_transaction.account;

    return withdraw_amount;
END;
$$;

CREATE FUNCTION public.deposit(p_txn_id uuid, p_from_account bigint, p_to_account bigint, p_amount bigint, p_reference text, p_routing jsonb)
RETURNS uuid
LANGUAGE
plpgsql
AS
$$
DECLARE
    source_account record;
    target_account record;
    debit_txn bank_transaction_data;
    credit_txn bank_transaction_data;
BEGIN
    --- check source account conditions
    SELECT *
    INTO source_account
    FROM public.bank_accounts
    WHERE id = p_from_account;

    IF NOT found THEN
        raise 'Source account with id % not found', p_from_account;
    END IF;

    IF NOT (source_account.account_status = 'open') THEN
        raise 'Source account not open';
    END IF;

    IF NOT (source_account.account_type = 'holding') THEN
        raise 'Source account is not a holding account';
    END IF;

    -- check target account conditions
    SELECT *
    INTO target_account
    FROM public.bank_accounts
    WHERE id = p_to_account;

    IF NOT found THEN
        raise 'Target account with id % not found', p_to_account;
    END IF;

    IF NOT (target_account.account_status = 'open' OR target_account.account_status = 'pending') THEN
        raise 'Target account closed';
    END IF;

    IF NOT source_account.currency = target_account.currency THEN
        raise 'Accounts are for different currencies';
    END IF;

    -- log transfer
    debit_txn.account := p_from_account;
    debit_txn.account_entry := source_account.next_entry + 1;
    debit_txn.other_account := p_to_account;
    debit_txn.initial_status := 'en_route';

    PERFORM new_transfer(p_txn_id, p_amount, p_reference, p_routing, debit_txn, NULL);
 
    -- reserve money from holding account
    UPDATE public.bank_accounts
    SET next_entry = source_account.next_entry + 1,
        balance = balance - p_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = source_account.id;

    return p_txn_id;
END;
$$;

CREATE FUNCTION public.deposit_with_contact_method(p_txn_id uuid, p_from_account bigint, p_contact bigint, p_amount bigint, p_reference text, p_routing jsonb)
RETURNS uuid
LANGUAGE
plpgsql
AS
$$
DECLARE
    contact record;
    source_account record;
    target_account record;
    debit_txn bank_transaction_data;
    credit_txn bank_transaction_data;
BEGIN
    -- fetch contact
    SELECT *
    INTO contact
    FROM public.bank_contact
    WHERE id = p_from_account;

    IF NOT found THEN
        raise 'Contact with id % not found', p_contact;
    END IF;

    IF NOT (contact.contact_status = 'approved') THEN
        raise 'Contact %', contact.contact_status;
    END IF;

    --- check source account conditions
    SELECT *
    INTO source_account
    FROM public.bank_accounts
    WHERE id = p_from_account;

    IF NOT found THEN
        raise 'Source account with id % not found', p_from_account;
    END IF;

    IF NOT (source_account.account_status = 'open') THEN
        raise 'Source account not open';
    END IF;

    IF NOT (source_account.account_type = 'holding') THEN
        raise 'Source account is not a holding account';
    END IF;

    -- check target account conditions
    SELECT *
    INTO target_account
    FROM public.bank_accounts
    WHERE id = contact.account;

    IF NOT found THEN
        raise 'Target account with id % not found', p_to_account;
    END IF;

    IF NOT (target_account.account_status = 'open' OR target_account.account_status = 'pending') THEN
        raise 'Target account closed';
    END IF;

    IF NOT source_account.currency = target_account.currency THEN
        raise 'Accounts are for different currencies';
    END IF;

    -- log transfer
    debit_txn.account := p_from_account;
    debit_txn.account_entry := source_account.next_entry + 1;
    debit_txn.other_account := p_to_account;
    debit_txn.initial_status := 'en_route';

    PERFORM new_transfer(p_txn_id, p_amount, p_reference, contact.payment_methods, debit_txn, NULL);
 
    -- reserve money from holding account
    UPDATE public.bank_accounts
    SET next_entry = source_account.next_entry + 1,
        balance = balance - p_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = source_account.id;

    return p_txn_id;
END;
$$;

CREATE FUNCTION public.settle_deposit(p_txn_id uuid)
RETURNS bigint
LANGUAGE
plpgsql
AS
$$
DECLARE
    debit_transaction record;
    deposit_amount bigint;
    target_account record;
BEGIN
    -- update debit transaction status
    UPDATE public.bank_transactions
    SET transaction_status = 'settled',
        updated_at = CURRENT_TIMESTAMP
    WHERE txn_id = p_txn_id AND transaction_type = 'debit' AND transaction_status = 'en_route'
    RETURNING * INTO debit_transaction;
    
    IF NOT found THEN
        raise 'No debit transaction found';
    END IF;

    SELECT amount
    INTO deposit_amount
    FROM public.bank_transfers
    WHERE txn_id = p_txn_id;

    IF NOT found THEN
        raise 'No transfer found';
    END IF;

    -- check target account conditions again
    SELECT *
    INTO target_account
    FROM public.bank_accounts
    WHERE id = debit_transaction.other_account;

    IF NOT found THEN
        raise 'Target account with id % not found', p_to_account;
    END IF;

    IF NOT (target_account.account_status = 'open' OR target_account.account_status = 'pending') THEN
        raise 'Account closed';
    END IF;

    -- log credit transaction
    INSERT INTO public.bank_transactions (
            txn_id, account, account_entry, other_account, transaction_type, transaction_status
    ) VALUES
        (p_txn_id, debit_transaction.other_account, target_account.next_entry + 1, debit_transaction.account, 'credit', 'settled');

    -- move money into target account
    UPDATE public.bank_accounts
    SET next_entry = next_entry + 1,
        balance = balance + deposit_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = debit_transaction.other_account;

    -- Simulated incoming money
    UPDATE public.bank_accounts
    SET balance = balance + deposit_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = debit_transaction.account;

    return deposit_amount;
END;
$$;

CREATE FUNCTION public.reverse_deposit(p_txn_id uuid)
RETURNS bigint
LANGUAGE
plpgsql
AS
$$
DECLARE
    debit_account bigint;
    deposit_amount bigint;
BEGIN
    -- update debit transaction status
    UPDATE public.bank_transactions
    SET transaction_status = 'canceled',
        updated_at = CURRENT_TIMESTAMP
    WHERE txn_id = p_txn_id AND transaction_type = 'debit' AND transaction_status = 'en_route'
    RETURNING account INTO debit_account;
    
    IF NOT found THEN
        raise 'No debit transaction found';
    END IF;

    SELECT amount
    INTO deposit_amount
    FROM public.bank_transfers
    WHERE txn_id = p_txn_id;

    IF NOT found THEN
        raise 'No transfer found';
    END IF;

   -- Reverse transfer
    UPDATE public.bank_accounts
    SET balance = balance + deposit_amount,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = debit_account;

    return deposit_amount;
END;
$$;
