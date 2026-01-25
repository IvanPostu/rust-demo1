CREATE SEQUENCE accounts_seq START WITH 1000;

CREATE TABLE accounts ( -- mydb.public.accounts 
    id BIGINT PRIMARY KEY DEFAULT nextval('accounts_seq'),
    owner_name VARCHAR(255) NOT NULL UNIQUE,
    balance NUMERIC(10, 2)  NOT NULL DEFAULT 0.00 CHECK (balance >= 0)
);
