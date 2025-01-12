CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    stellar_address TEXT NOT NULL UNIQUE,
    account_type TEXT NOT NULL CHECK (account_type IN ('issuer', 'distributor', 'user')),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    status TEXT NOT NULL CHECK (status IN ('active', 'suspended', 'closed'))
);

CREATE TABLE trustlines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID REFERENCES accounts(id) ON DELETE CASCADE,
    asset_code TEXT NOT NULL,
    asset_issuer TEXT NOT NULL,
    trust_limit NUMERIC DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    status TEXT NOT NULL CHECK (status IN ('active', 'revoked'))
);

CREATE TABLE tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_code TEXT NOT NULL UNIQUE,
    issuer_account_id UUID REFERENCES accounts(id),
    total_supply NUMERIC DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_account_id UUID REFERENCES accounts(id),
    destination_account_id UUID REFERENCES accounts(id),
    transaction_hash TEXT NOT NULL UNIQUE,
    amount NUMERIC NOT NULL,
    asset_code TEXT NOT NULL,
    memo TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    status TEXT NOT NULL CHECK (status IN ('pending', 'completed', 'failed'))
);

CREATE TABLE transaction_errors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id UUID REFERENCES transactions(id),
    error_code TEXT NOT NULL,
    error_message TEXT NOT NULL,
    occurred_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE encrypted_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID REFERENCES accounts(id) ON DELETE CASCADE,
    encrypted_key BYTEA NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
