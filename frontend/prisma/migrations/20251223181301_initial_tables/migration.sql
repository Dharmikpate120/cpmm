-- CreateTable
CREATE TABLE "markets" (
    "amm_token_account" TEXT NOT NULL,
    "token_a_pool_account" TEXT NOT NULL,
    "token_b_pool_account" TEXT NOT NULL,
    "token_a_mint_account" TEXT NOT NULL,
    "token_b_mint_account" TEXT NOT NULL,
    "lp_token_mint_account" TEXT NOT NULL,

    CONSTRAINT "markets_pkey" PRIMARY KEY ("amm_token_account")
);

-- CreateTable
CREATE TABLE "users" (
    "user_publickey" TEXT NOT NULL,
    "amm_token_account" TEXT NOT NULL,
    "user_token_a_account" TEXT NOT NULL,
    "user_token_b_account" TEXT NOT NULL,
    "user_lp_token_account" TEXT NOT NULL,
    "user_type" TEXT NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("user_publickey","amm_token_account")
);

-- CreateTable
CREATE TABLE "liquidity_transactions" (
    "transaction_id" TEXT NOT NULL,
    "amm_token_account" TEXT NOT NULL,
    "user_publickey" TEXT NOT NULL,
    "transaction_type" TEXT NOT NULL,
    "amount_a" INTEGER NOT NULL,
    "amount_b" INTEGER NOT NULL,
    "lp_tokens" INTEGER NOT NULL,

    CONSTRAINT "liquidity_transactions_pkey" PRIMARY KEY ("transaction_id")
);

-- CreateTable
CREATE TABLE "swap_transactions" (
    "transaction_id" TEXT NOT NULL,
    "amm_token_account" TEXT NOT NULL,
    "user_publickey" TEXT NOT NULL,
    "amount_in" INTEGER NOT NULL,
    "amount_out" INTEGER NOT NULL,
    "token_in" TEXT NOT NULL,
    "token_out" TEXT NOT NULL,

    CONSTRAINT "swap_transactions_pkey" PRIMARY KEY ("transaction_id")
);

-- CreateIndex
CREATE UNIQUE INDEX "markets_amm_token_account_key" ON "markets"("amm_token_account");

-- CreateIndex
CREATE UNIQUE INDEX "markets_lp_token_mint_account_key" ON "markets"("lp_token_mint_account");
