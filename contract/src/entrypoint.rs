use solana_program::{
    entrypoint,
    account_info::{ AccountInfo, next_account_info },
    entrypoint::{ ProgramResult },
    pubkey::Pubkey,
};
// amm program id:DzJJz3MQJeqgfGePLndCFG64M8zKJAKaWHYgTzXfuPnZ

// admin:GK5uAKRv4Abn4szsDuhvcBDoYp8cwAcggkkynMjmSZf3
// (solana-keygen new --force)

// token a mint:6pXNd5iDL3kqz8MxGiqZu4hb9vB9KbrPknGJ5vXfkMgd
// (spl-token create-token)

// token b mint:DB1xwND2situnNxmSCF3XNHnXXayLPoNSHDKJEfNiwpP
// (spl-token create-token)

// token a admin: cfqVCaPpWougv7Kq8kamHk3oZRNYhrTK6QURcACe1xt
// (spl-token create-account token-a-mint, spl-token mint mint-a 1000 token-a-admin)

// token b admin: ADE1HAFKUiNSvWyLUDoQmLsNCqUKm297m6HRfx9GPynn
// (spl-token create-account token-b-mint, spl-token mint mint-b 10 token-b-admin)

//spl token: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

use crate::{ error::AMMError, instruction::AMMInstruction, processor::Processor };
use solana_program::msg;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!("Entrypoint: processing instruction");

    match AMMInstruction::unpack(instruction_data)? {
        AMMInstruction::InitializeTokenPool {
            trade_fee,
            initial_token_a_liquidity,
            initial_token_b_liquidity,
        } => {
            Processor::initialize_token_pool_account(
                program_id,
                accounts,
                trade_fee,
                initial_token_a_liquidity,
                initial_token_b_liquidity
            )
        }
        AMMInstruction::AddLiquidity { amount_a_max, amount_b_max, minimum_lp_tokens } => {
            Processor::add_liquidity(
                program_id,
                accounts,
                amount_a_max,
                amount_b_max,
                minimum_lp_tokens
            )
        }
        AMMInstruction::WithdrawLiquidity { amount_a_min, amount_b_min, maximum_lp_tokens } => {
            Processor::withdraw_liquidity(
                program_id,
                accounts,
                amount_a_min,
                amount_b_min,
                maximum_lp_tokens
            )
        }
        AMMInstruction::Swap {
            amount_in,
            minimum_amount_out,
            mint_address_in,
            mint_address_out,
        } => {
            Processor::swap_tokens(
                program_id,
                accounts,
                amount_in,
                minimum_amount_out,
                mint_address_in,
                mint_address_out
            )
        }
        _ => { Ok(()) }
    }
}
