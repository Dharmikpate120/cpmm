export type AmmAccountData = {
    token_a_mint_account: string,
    token_b_mint_account: string,
    admin_token_a_account: string,
    admin_token_b_account: string,
    trade_fee: string,
    initial_token_a_liquidity: string,
    initial_token_b_liquidity: string,
    admin_account: string
}
export type AmmAddLiquidity = {
    token_a_mint_account: string,
    token_b_mint_account: string,
    provider_token_a_account: string,
    provider_token_b_account: string,
    amount_a_max: string,
    amount_b_max: string,
    minimum_lp_tokens: string,
    provider_account: string
}

export type AmmWithdrawLiquidity = {
    token_a_mint_account: string,
    token_b_mint_account: string,
    provider_token_a_account: string,
    provider_token_b_account: string,
    amount_a_min: string,
    amount_b_min: string,
    maximum_lp_tokens: string,
    provider_account: string
}

export type SwapTokens = {
    swapper_token_a_account: string,
    swapper_token_b_account: string,
    token_a_mint_account: string,
    token_b_mint_account: string,
    max_amount_in: string,
    minimum_amount_out: string,
    swapper_account: string // Assuming we need this similar to provider_account
}