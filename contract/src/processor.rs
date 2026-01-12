use crate::{ error::AMMError, state::{ AMMAccount } };
use num_traits::Float;
use solana_program::{
    account_info::{ AccountInfo, next_account_info },
    config::program,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use borsh::{ BorshSerialize, BorshDeserialize };
use solana_program::program_pack::Pack;
use solana_system_interface::instruction;
use bstr::concat;
use spl_token_interface::state::AccountState;

#[derive(BorshSerialize, BorshDeserialize)]
struct System_Instruction {
    pub lamports: u64,
    pub space: u64,
    pub owner: Pubkey,
}

// pub fn create_system_program_account(
//     instData: System_Instruction,
//     accounts: &[AccountInfo]
// ) -> Result<(), AMMError> {
//     Ok(())
// }

pub struct Processor;

impl Processor {
    pub fn initialize_token_pool_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        trade_fee: u64,
        initial_token_a_liquidity: u64,
        initial_token_b_liquidity: u64
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let admin_account = next_account_info(account_iter)?;

        let system_program_account = next_account_info(account_iter)?;

        let spl_token_program_account = next_account_info(account_iter)?;

        let token_a_mint_account = next_account_info(account_iter)?;

        let token_b_mint_account = next_account_info(account_iter)?;

        let admin_token_a_account = next_account_info(account_iter)?;

        let admin_token_b_account = next_account_info(account_iter)?;

        let lp_token_mint_account = next_account_info(account_iter)?;

        let admin_lp_token_account = next_account_info(account_iter)?;

        let token_a_pool_account = next_account_info(account_iter)?;

        let token_b_pool_account = next_account_info(account_iter)?;

        let amm_token_account = next_account_info(account_iter)?;

        let sysvar_rent_account = next_account_info(account_iter)?;

        let amm_program_account = next_account_info(account_iter)?;

        if admin_account.is_signer == false {
            msg!("Error: Admin account is not a signer");
            return Err(AMMError::InvalidSigner.into());
        }

        if *system_program_account.key != solana_system_interface::program::ID {
            msg!("Error: System program account is invalid");
            return Err(AMMError::InvalidSystemProgram.into());
        }

        if *spl_token_program_account.key != spl_token_interface::id() {
            msg!("Error: SPL token program account is invalid");
            return Err(AMMError::InvalidSPLTokenProgram.into());
        }

        if admin_token_a_account.is_writable == false || admin_token_b_account.is_writable == false {
            msg!("Error: Admin token A account is not writable");
            return Err(AMMError::NotWritable.into());
        }
        if lp_token_mint_account.is_writable == false {
            msg!("Error:LP Token mint account are not writable");
            return Err(AMMError::NotWritable.into());
        }
        if admin_lp_token_account.is_writable == false {
            msg!("Error: Admin LP token account is not writable");
            return Err(AMMError::NotWritable.into());
        }
        if token_a_pool_account.is_writable == false || token_b_pool_account.is_writable == false {
            msg!("Error: Token pool accounts are not writable!");
            return Err(AMMError::NotWritable.into());
        }
        if amm_token_account.is_writable == false {
            msg!("Error: AMM token accoutn is not writable!");
            return Err(AMMError::NotWritable.into());
        }

        let admin_token_a_data = spl_token_interface::state::Account::unpack_from_slice(
            admin_token_a_account.data.borrow().as_ref()
        )?;
        if admin_token_a_data.is_frozen() {
            msg!("Error: Admin token A account is frozen");
            return Err(AMMError::TokenAccountFrozen.into());
        }
        if admin_token_a_data.state != AccountState::Initialized {
            msg!("Error: Admin token A account is not initialized");
            return Err(AMMError::TokenAccountFrozen.into());
        }

        let admin_token_b_data = spl_token_interface::state::Account::unpack_from_slice(
            admin_token_b_account.data.borrow().as_ref()
        )?;
        if admin_token_b_data.is_frozen() {
            msg!("Error: Admin token A account is frozen");
            return Err(AMMError::TokenAccountFrozen.into());
        }
        if admin_token_b_data.state != AccountState::Initialized {
            msg!("Error: Admin token A account is not initialized");
            return Err(AMMError::TokenAccountNotInitialized.into());
        }

        if
            admin_token_a_data.amount < initial_token_a_liquidity ||
            admin_token_b_data.amount < initial_token_b_liquidity
        {
            msg!("Error: Insufficient token balance in admin account");
            return Err(AMMError::InsufficientTokenBalance.into());
        }

        if *token_a_mint_account.key != admin_token_a_data.mint {
            msg!("Error: Admin token A account mint does not match provided token A mint account");
            return Err(AMMError::InvalidMintAccount.into());
        }
        if *token_b_mint_account.key != admin_token_b_data.mint {
            msg!("Error: Admin token B account mint does not match provided token B mint account");
            return Err(AMMError::InvalidMintAccount.into());
        }

        //validating amm token account
        let (amm_token_account_pda, amm_token_account_bump) = get_lexicographical_token_pda(
            token_a_mint_account.key,
            token_b_mint_account.key,
            program_id
        );
        if amm_token_account_pda != *amm_token_account.key {
            msg!("Error: Invalid AMM token account provided.");
            return Err(AMMError::InvalidAMMTokenAccount.into());
        }
        //creating and initializing amm pool account
        initialize_amm_pool_account(
            admin_account,
            program_id,
            amm_token_account,
            amm_token_account_bump,
            system_program_account,
            spl_token_program_account,
            token_a_mint_account.key,
            token_b_mint_account.key,
            token_a_pool_account.key,
            token_b_pool_account.key
        )?;

        // updating data of amm token account
        // let amm_token_account_data = AMMAccount::unpack_from_slice(
        //     &amm_token_account.data.borrow()
        // )?;
        let updated_amm_token_account_data = AMMAccount::Initialized {
            pool_authority: *program_id,
            token_a_mint: *token_a_mint_account.key,
            token_b_mint: *token_b_mint_account.key,
            lp_token_mint: *lp_token_mint_account.key,
            token_a_pool: *token_a_pool_account.key,
            token_b_pool: *token_b_pool_account.key,
            trade_fee: trade_fee,
        };
        updated_amm_token_account_data.pack_into_slice(&mut amm_token_account.data.borrow_mut());

        //validating lp token mint account
        let (lp_token_mint_account_pda, lp_token_mint_account_bump) = get_lexicographical_mint_pda(
            b"mint",
            &amm_token_account_pda,
            token_a_mint_account.key,
            token_b_mint_account.key,
            program_id
        );
        // msg!("aaaaaaaaaaaaaaaaaaa{}",lp_token_mint_account_pda);
        if lp_token_mint_account_pda != *lp_token_mint_account.key {
            msg!(
                "Error: Invalid LP token mint account provided. {}, {}",
                lp_token_mint_account_pda,
                lp_token_mint_account.key
            );
            return Err(AMMError::InvalidLPTokenMintAccount.into());
        }
        //creating and initializing lp token mint account
        initialize_lp_token_mint_account(
            admin_account,
            program_id,
            &amm_token_account_pda,
            lp_token_mint_account,
            system_program_account,
            spl_token_program_account,
            lp_token_mint_account_bump,
            token_a_mint_account,
            token_b_mint_account,
            sysvar_rent_account
        )?;

        //validating token a pool account
        let (token_a_pool_account_pda, token_a_pool_account_bump) = Pubkey::find_program_address(
            &[b"pool", amm_token_account_pda.as_array(), token_a_mint_account.key.as_array()],
            program_id
        );
        if token_a_pool_account_pda != *token_a_pool_account.key {
            msg!("Error: Invalid token A pool account provided.");
            return Err(AMMError::InvalidPDA.into());
        }
        //validating token b pool account
        let (token_b_pool_account_pda, token_b_pool_account_bump) = Pubkey::find_program_address(
            &[b"pool", amm_token_account_pda.as_array(), token_b_mint_account.key.as_array()],
            program_id
        );
        if token_b_pool_account_pda != *token_b_pool_account.key {
            msg!("Error: Invalid token B pool account provided.");
            return Err(AMMError::InvalidPDA.into());
        }
        initialize_token_pool_accounts(
            admin_account,
            program_id,
            &amm_token_account_pda,
            system_program_account,
            spl_token_program_account,
            token_a_mint_account,
            token_b_mint_account,
            token_a_pool_account,
            token_b_pool_account,
            token_a_pool_account_bump,
            token_b_pool_account_bump,
            amm_token_account,
            sysvar_rent_account,
            amm_program_account
        )?;

        //validating admin lp token account
        let (admin_lp_token_account_pda, admin_lp_token_account_bump) =
            get_lexicographical_lp_token_pda(
                admin_account.key,
                &amm_token_account_pda,
                token_a_mint_account.key,
                token_b_mint_account.key,
                program_id
            );

        if admin_lp_token_account_pda != *admin_lp_token_account.key {
            msg!("Error: Invalid admin lp token account proviided.");
            return Err(AMMError::InvalidPDA.into());
        }
        //initializing admin lp token account
        initialize_admin_lp_token_account(
            admin_account,
            program_id,
            &amm_token_account_pda,
            admin_lp_token_account,
            spl_token_program_account,
            system_program_account,
            lp_token_mint_account,
            token_a_mint_account,
            token_b_mint_account,
            admin_lp_token_account_bump,
            sysvar_rent_account,
            amm_program_account
        )?;

        //collecting token a and token b in pool account
        let token_a_admin_to_pool_transfer_instruction = spl_token_interface::instruction::transfer(
            spl_token_program_account.key,
            admin_token_a_account.key,
            token_a_pool_account.key,
            admin_account.key,
            &[admin_account.key],
            initial_token_a_liquidity
        )?;
        msg!("outside  1");
        solana_program::program::invoke(
            &token_a_admin_to_pool_transfer_instruction,
            &[
                spl_token_program_account.clone(),
                admin_token_a_account.clone(),
                token_a_pool_account.clone(),
                admin_account.clone(),
            ]
        )?;
        let token_b_admin_to_pool_transfer_instruction = spl_token_interface::instruction::transfer(
            spl_token_program_account.key,
            admin_token_b_account.key,
            token_b_pool_account.key,
            admin_account.key,
            &[admin_account.key],
            initial_token_b_liquidity
        )?;
        msg!("outside  2");
        solana_program::program::invoke(
            &token_b_admin_to_pool_transfer_instruction,
            &[
                spl_token_program_account.clone(),
                admin_token_b_account.clone(),
                token_b_pool_account.clone(),
                admin_account.clone(),
            ]
        )?;
        //calculating the required lp token mint amount

        let lp_token_mint_amount = (
            (initial_token_a_liquidity as f64) * (initial_token_b_liquidity as f64)
        )
            .sqrt()
            .floor() as u64;
        //lp token mint in admin lp token account
        let lp_token_mint_to_admin_instruction = spl_token_interface::instruction::mint_to(
            spl_token_program_account.key,
            lp_token_mint_account.key,
            admin_lp_token_account.key,
            lp_token_mint_account.key,
            &[lp_token_mint_account.key],
            lp_token_mint_amount
        )?;
        msg!("outside 3");
        solana_program::program::invoke_signed(
            &lp_token_mint_to_admin_instruction,
            &[
                spl_token_program_account.clone(),
                lp_token_mint_account.clone(),
                admin_lp_token_account.clone(),
                amm_token_account.clone(),
            ],
            &[
                &[
                    b"mint",
                    amm_token_account_pda.as_array(),
                    token_a_mint_account.key.as_array(),
                    token_b_mint_account.key.as_array(),
                    &[lp_token_mint_account_bump],
                ],
            ]
        )?;
        Ok(())
    }

    pub fn add_liquidity(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount_a_max: u64,
        amount_b_max: u64,
        minimum_lp_tokens: u64
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        //1. liquidity_provider_account(signer, writable)
        let liquidity_provider_account = next_account_info(accounts_iter)?;

        //2.spl_token_account (readonly)
        let spl_token_account = next_account_info(accounts_iter)?;

        //3. amm_token_account(writable)
        let amm_token_account = next_account_info(accounts_iter)?;

        //4. amm_token_a_pool_account(writable)
        let amm_token_a_pool_account = next_account_info(accounts_iter)?;

        //5. amm_token_b_pool_account(writable)
        let amm_token_b_pool_account = next_account_info(accounts_iter)?;

        //6. provider_token_a_account(writabler)
        let provider_token_a_account = next_account_info(accounts_iter)?;

        //7. provider_token_b_account(writable)
        let provider_token_b_account = next_account_info(accounts_iter)?;
        //8. provider_lp_token_account(writable)
        let provider_lp_token_account = next_account_info(accounts_iter)?;

        //9. lp_token_mint_account(writable)
        let lp_token_mint_account = next_account_info(accounts_iter)?;

        //10.sysvar_rent_account (readonly)
        let sysvar_rent_account = next_account_info(accounts_iter)?;

        //11. token_a_mint_account(read only)
        let token_a_mint_account = next_account_info(accounts_iter)?;

        //12. token_b_mint_account(read only)
        let token_b_mint_account = next_account_info(accounts_iter)?;

        //13. system_program_account(read only)
        let system_program_account = next_account_info(accounts_iter)?;

        //14. amm_program_account (read only)
        let amm_program_account = next_account_info(accounts_iter)?;

        //validating amm token account
        let (amm_token_account_pda, amm_token_account_bump) = get_lexicographical_token_pda(
            token_a_mint_account.key,
            token_b_mint_account.key,
            program_id
        );
        if amm_token_account_pda != *amm_token_account.key {
            msg!("Error: Invalid AMM token account provided.");
            return Err(AMMError::InvalidAMMTokenAccount.into());
        }

        //validating token a pool account
        let (token_a_pool_account_pda, token_a_pool_account_bump) = Pubkey::find_program_address(
            &[b"pool", amm_token_account_pda.as_array(), token_a_mint_account.key.as_array()],
            program_id
        );
        if token_a_pool_account_pda != *amm_token_a_pool_account.key {
            msg!("Error: Invalid token A pool account provided.");
            return Err(AMMError::InvalidPDA.into());
        }

        //validating token b pool account
        let (token_b_pool_account_pda, token_b_pool_account_bump) = Pubkey::find_program_address(
            &[b"pool", amm_token_account_pda.as_array(), token_b_mint_account.key.as_array()],
            program_id
        );
        if token_b_pool_account_pda != *amm_token_b_pool_account.key {
            msg!("Error: Invalid token B pool account provided.");
            return Err(AMMError::InvalidPDA.into());
        }

        //validating lp token mint account
        let (lp_token_mint_account_pda, lp_token_mint_account_bump) = get_lexicographical_mint_pda(
            b"mint",
            &amm_token_account_pda,
            token_a_mint_account.key,
            token_b_mint_account.key,
            program_id
        );

        if lp_token_mint_account_pda != *lp_token_mint_account.key {
            msg!(
                "Error: Invalid LP token mint account provided. {}, {}",
                lp_token_mint_account_pda,
                lp_token_mint_account.key
            );
            return Err(AMMError::InvalidLPTokenMintAccount.into());
        }

        let token_a_pool_data = spl_token_interface::state::Account::unpack_from_slice(
            &amm_token_a_pool_account.data.borrow()
        )?;

        let token_b_pool_data = spl_token_interface::state::Account::unpack_from_slice(
            &amm_token_b_pool_account.data.borrow()
        )?;

        // let constant_k = token_a_pool_data.amount * token_b_pool_data.amount;

        let lp_token_mint_data = spl_token_interface::state::Mint::unpack_from_slice(
            &lp_token_mint_account.data.borrow()
        )?;

        // let withdraw_amount_a = amount_a_max;
        // let withdraw_amount_b = constant_k/ withdraw_amount_a;
        let calculated_amount_a: u64;
        let calculated_amount_b: u64;
        let current_ratio: f64 =
            (token_a_pool_data.amount as f64) / (token_b_pool_data.amount as f64);
        let difference_ratio: f64 = (amount_a_max as f64) / (amount_b_max as f64);

        if current_ratio < difference_ratio {
            calculated_amount_b = amount_b_max;
            calculated_amount_a = ((amount_b_max as f64) * current_ratio).ceil() as u64;
        } else {
            calculated_amount_a = amount_a_max;
            calculated_amount_b = ((amount_a_max as f64) / current_ratio).ceil() as u64;
        }
        let lp_token_by_a =
            ((calculated_amount_a as f64) / (token_a_pool_data.amount as f64)) *
            (lp_token_mint_data.supply as f64);
        let lp_token_by_b =
            ((calculated_amount_b as f64) / (token_b_pool_data.amount as f64)) *
            (lp_token_mint_data.supply as f64);
        let lp_token_mint_amount: u64;
        if lp_token_by_a < lp_token_by_b {
            lp_token_mint_amount = lp_token_by_a.floor() as u64;
        } else {
            lp_token_mint_amount = lp_token_by_b.floor() as u64;
        }

        if lp_token_mint_amount < minimum_lp_tokens {
            return Err(AMMError::InsufficientLpTokensAvailable.into());
        }
        msg!(
            "a: {},poola: {},  b: {}, poolb: {}, lp: {}, supply: {}",
            calculated_amount_a,
            token_a_pool_data.amount,
            calculated_amount_b,
            token_b_pool_data.amount,
            lp_token_mint_amount,
            lp_token_mint_data.supply
        );

        // invoke the spl - transfer to withdraw token a and token b from the provider to pool

        //withdrawing token a from provider to pool
        let token_a_transfer_instruction = spl_token_interface::instruction::transfer(
            spl_token_account.key,
            provider_token_a_account.key,
            amm_token_a_pool_account.key,
            liquidity_provider_account.key,
            &[liquidity_provider_account.key],
            calculated_amount_a
        )?;
        msg!("add 1");
        solana_program::program::invoke(
            &token_a_transfer_instruction,
            &[
                spl_token_account.clone(),
                provider_token_a_account.clone(),
                amm_token_a_pool_account.clone(),
                liquidity_provider_account.clone(),
            ]
        )?;

        //withdrawing token b from provider to pool
        let token_b_transfer_instruction = spl_token_interface::instruction::transfer(
            spl_token_account.key,
            provider_token_b_account.key,
            amm_token_b_pool_account.key,
            liquidity_provider_account.key,
            &[liquidity_provider_account.key],
            calculated_amount_b
        )?;
        msg!("add 2");
        solana_program::program::invoke(
            &token_b_transfer_instruction,
            &[
                spl_token_account.clone(),
                provider_token_b_account.clone(),
                amm_token_b_pool_account.clone(),
                liquidity_provider_account.clone(),
            ]
        )?;
        let (provider_lp_token_account_pda, provider_lp_token_account_bump) =
            get_lexicographical_lp_token_pda(
                liquidity_provider_account.key,
                &amm_token_account_pda,
                token_a_mint_account.key,
                token_b_mint_account.key,
                program_id
            );

        if provider_lp_token_account_pda != *provider_lp_token_account.key {
            msg!("Error: Invalid lp token account provided.");
            return Err(AMMError::InvalidPDA.into());
        }
        //initializing admin lp token account
        initialize_admin_lp_token_account(
            liquidity_provider_account,
            program_id,
            &amm_token_account_pda,
            provider_lp_token_account,
            spl_token_account,
            system_program_account,
            lp_token_mint_account,
            token_a_mint_account,
            token_b_mint_account,
            provider_lp_token_account_bump,
            sysvar_rent_account,
            amm_program_account
        )?;
        // invoke spl - mint_to to mint lp token in provider lp account and create lp account for provider if doesn't exist
        let lp_token_mint_instruction = spl_token_interface::instruction::mint_to(
            spl_token_account.key,
            lp_token_mint_account.key,
            provider_lp_token_account.key,
            lp_token_mint_account.key,
            &[lp_token_mint_account.key],
            lp_token_mint_amount
        )?;
        msg!("add 3");
        solana_program::program::invoke_signed(
            &lp_token_mint_instruction,
            &[
                spl_token_account.clone(),
                lp_token_mint_account.clone(),
                provider_lp_token_account.clone(),
                lp_token_mint_account.clone(),
            ],
            &[
                &[
                    b"mint",
                    amm_token_account_pda.as_array(),
                    token_a_mint_account.key.as_array(),
                    token_b_mint_account.key.as_array(),
                    &[lp_token_mint_account_bump],
                ],
            ]
        )?;

        // amm_token_a_pool_account.data
        // spl_token_interface::program::instruction::transfer()

        Ok(())
    }

    pub fn withdraw_liquidity(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount_a_min: u64,
        amount_b_min: u64,
        maximum_lp_tokens: u64
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        //1. liquidity_provider_account(signer, writable)
        let liquidity_provider_account = next_account_info(accounts_iter)?;

        //2.spl_token_account (readonly)
        let spl_token_account = next_account_info(accounts_iter)?;

        //3. amm_token_account(writable)
        let amm_token_account = next_account_info(accounts_iter)?;

        //4. amm_token_a_pool_account(writable)
        let amm_token_a_pool_account = next_account_info(accounts_iter)?;

        //5. amm_token_b_pool_account(writable)
        let amm_token_b_pool_account = next_account_info(accounts_iter)?;

        //6. provider_token_a_account(writabler)
        let provider_token_a_account = next_account_info(accounts_iter)?;

        //7. provider_token_b_account(writable)
        let provider_token_b_account = next_account_info(accounts_iter)?;
        //8. provider_lp_token_account(writable)
        let provider_lp_token_account = next_account_info(accounts_iter)?;

        //9. lp_token_mint_account(writable)
        let lp_token_mint_account = next_account_info(accounts_iter)?;

        //10.sysvar_rent_account (readonly)
        let sysvar_rent_account = next_account_info(accounts_iter)?;

        //11. token_a_mint_account(read only)
        let token_a_mint_account = next_account_info(accounts_iter)?;

        //12. token_b_mint_account(read only)
        let token_b_mint_account = next_account_info(accounts_iter)?;

        //13. system_program_account(read only)
        let system_program_account = next_account_info(accounts_iter)?;

        //14. amm_program_account (read only)
        let amm_program_account = next_account_info(accounts_iter)?;

        //validating amm token account
        let (amm_token_account_pda, amm_token_account_bump) = get_lexicographical_token_pda(
            token_a_mint_account.key, token_b_mint_account.key,
            program_id
        );
        if amm_token_account_pda != *amm_token_account.key {
            msg!("Error: Invalid AMM token account provided.");
            return Err(AMMError::InvalidAMMTokenAccount.into());
        }

        //validating token a pool account
        let (token_a_pool_account_pda, token_a_pool_account_bump) = Pubkey::find_program_address(
            &[b"pool", amm_token_account_pda.as_array(), token_a_mint_account.key.as_array()],
            program_id
        );
        if token_a_pool_account_pda != *amm_token_a_pool_account.key {
            msg!("Error: Invalid token A pool account provided.");
            return Err(AMMError::InvalidPDA.into());
        }

        //validating token b pool account
        let (token_b_pool_account_pda, token_b_pool_account_bump) = Pubkey::find_program_address(
            &[b"pool", amm_token_account_pda.as_array(), token_b_mint_account.key.as_array()],
            program_id
        );
        if token_b_pool_account_pda != *amm_token_b_pool_account.key {
            msg!("Error: Invalid token B pool account provided.");
            return Err(AMMError::InvalidPDA.into());
        }

        //validating lp token mint account
        let (lp_token_mint_account_pda, lp_token_mint_account_bump) = get_lexicographical_mint_pda(
                b"mint",
                &amm_token_account_pda,
                token_a_mint_account.key,
                token_b_mint_account.key,
            program_id
        );

        if lp_token_mint_account_pda != *lp_token_mint_account.key {
            msg!(
                "Error: Invalid LP token mint account provided. {}, {}",
                lp_token_mint_account_pda,
                lp_token_mint_account.key
            );
            return Err(AMMError::InvalidLPTokenMintAccount.into());
        }

        let spl_state = spl_token_interface::state::Account::unpack_from_slice(
            &provider_lp_token_account.data.borrow()
        )?;
        if !spl_state.is_initialized() {
            msg!("please provide liquidity first before withdrawing");
            return Err(AMMError::AccountNotInitialized.into());
        }

        let token_a_pool_data = spl_token_interface::state::Account::unpack_from_slice(
            &amm_token_a_pool_account.data.borrow()
        )?;

        let token_b_pool_data = spl_token_interface::state::Account::unpack_from_slice(
            &amm_token_b_pool_account.data.borrow()
        )?;

        let constant_k = token_a_pool_data.amount * token_b_pool_data.amount;

        let lp_token_mint_data = spl_token_interface::state::Mint::unpack_from_slice(
            &lp_token_mint_account.data.borrow()
        )?;

        // let withdraw_amount_a = amount_a_max;
        // let withdraw_amount_b = constant_k/ withdraw_amount_a;
        let calculated_amount_a: u64;
        let calculated_amount_b: u64;
        let current_ratio: f64 =
            (token_a_pool_data.amount as f64) / (token_b_pool_data.amount as f64);
        let difference_ratio: f64 = (amount_a_min as f64) / (amount_b_min as f64);

        if current_ratio < difference_ratio {
            calculated_amount_a = amount_a_min;
            calculated_amount_b = ((amount_a_min as f64) / current_ratio).ceil() as u64;
        } else {
            calculated_amount_b = amount_b_min;
            calculated_amount_a = ((amount_b_min as f64) * current_ratio).ceil() as u64;
        }
        let lp_token_by_a =
            ((calculated_amount_a as f64) / (token_a_pool_data.amount as f64)) *
            (lp_token_mint_data.supply as f64);
        let lp_token_by_b =
            ((calculated_amount_b as f64) / (token_b_pool_data.amount as f64)) *
            (lp_token_mint_data.supply as f64);
        let lp_token_mint_amount: u64;
        if lp_token_by_a < lp_token_by_b {
            lp_token_mint_amount = lp_token_by_a.floor() as u64;
        } else {
            lp_token_mint_amount = lp_token_by_b.floor() as u64;
        }
        msg!(
            "a: {},poola: {},  b: {}, poolb: {}, lp: {}, supply: {}",
            calculated_amount_a,
            token_a_pool_data.amount,
            calculated_amount_b,
            token_b_pool_data.amount,
            lp_token_mint_amount,
            lp_token_mint_data.supply
        );

        // invoke the spl - transfer to withdraw token a and token b from the provider to pool

        //withdrawing token a from provider to pool
        let token_a_transfer_instruction = spl_token_interface::instruction::transfer(
            spl_token_account.key,
            amm_token_a_pool_account.key,
            provider_token_a_account.key,
            amm_token_a_pool_account.key,
            &[amm_token_a_pool_account.key],
            calculated_amount_a
        )?;
        msg!("withdraw 1");
        solana_program::program::invoke_signed(
            &token_a_transfer_instruction,
            &[
                spl_token_account.clone(),
                amm_token_a_pool_account.clone(),
                provider_token_a_account.clone(),
                amm_token_a_pool_account.clone(),
            ],
            &[
                &[
                    b"pool",
                    amm_token_account_pda.as_array(),
                    token_a_mint_account.key.as_array(),
                    &[token_a_pool_account_bump],
                ],
            ]
        )?;

        //withdrawing token b from provider to pool
        let token_b_transfer_instruction = spl_token_interface::instruction::transfer(
            spl_token_account.key,
            amm_token_b_pool_account.key,
            provider_token_b_account.key,
            amm_token_b_pool_account.key,
            &[amm_token_b_pool_account.key],
            calculated_amount_b
        )?;
        msg!("withdraw 2");
        solana_program::program::invoke_signed(
            &token_b_transfer_instruction,
            &[
                spl_token_account.clone(),
                amm_token_b_pool_account.clone(),
                provider_token_b_account.clone(),
                amm_token_b_pool_account.clone(),
            ],
            &[
                &[
                    b"pool",
                    amm_token_account_pda.as_array(),
                    token_b_mint_account.key.as_array(),
                    &[token_b_pool_account_bump],
                ],
            ]
        )?;
        let (provider_lp_token_account_pda, provider_lp_token_account_bump) =
            get_lexicographical_lp_token_pda(
                
                    liquidity_provider_account.key,
                    &amm_token_account_pda,
                    token_a_mint_account.key,
                    token_b_mint_account.key,
                program_id
            );

        if provider_lp_token_account_pda != *provider_lp_token_account.key {
            msg!("Error: Invalid lp token account provided.");
            return Err(AMMError::InvalidPDA.into());
        }

        //initializing admin lp token account
        // initialize_admin_lp_token_account(
        //     liquidity_provider_account,
        //     program_id,
        //     &amm_token_account_pda,
        //     provider_lp_token_account,
        //     spl_token_account,
        //     system_program_account,
        //     lp_token_mint_account,
        //     token_a_mint_account,
        //     token_b_mint_account,
        //     provider_lp_token_account_bump,
        //     sysvar_rent_account,
        //     amm_program_account
        // )?;
        // invoke spl - mint_to to mint lp token in provider lp account and create lp account for provider if doesn't exist
        let lp_token_mint_instruction = spl_token_interface::instruction::burn(
            spl_token_account.key,
            provider_lp_token_account.key,
            lp_token_mint_account.key,
            liquidity_provider_account.key,
            &[liquidity_provider_account.key],
            lp_token_mint_amount
        )?;
        msg!("withdraw 3");
        solana_program::program::invoke(
            &lp_token_mint_instruction,
            &[
                spl_token_account.clone(),
                provider_lp_token_account.clone(),
                lp_token_mint_account.clone(),
                liquidity_provider_account.clone(),
            ]
            // ,
            // &[
            // &[
            // b"mint",
            // amm_token_account_pda.as_array(),
            // token_a_mint_account.key.as_array(),
            // token_b_mint_account.key.as_array(),
            // &[lp_token_mint_account_bump],
            // ],
            // ]
        )?;

        // amm_token_a_pool_account.data
        // spl_token_interface::program::instruction::transfer()

        Ok(())
    }

    pub fn swap_tokens(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount_in: u64,
        minimum_amount_out: u64,
        mint_address_in: Pubkey,
        mint_address_out: Pubkey
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();

        //0. swapper_account(signer, writable)
        let swapper_account = next_account_info(account_iter)?;

        //1. spl_token_account (readonly)
        let spl_token_account = next_account_info(account_iter)?;

        //2. amm_token_account(writable)
        let amm_token_account = next_account_info(account_iter)?;

        //3. amm_token_a_pool_account(writable)
        let amm_token_a_pool_account = next_account_info(account_iter)?;

        //4. amm_token_b_pool_account(writable)
        let amm_token_b_pool_account = next_account_info(account_iter)?;

        //5. swapper_token_a_account(writable)
        let swapper_token_a_account = next_account_info(account_iter)?;

        //6. swapper_token_b_account(writable)
        let swapper_token_b_account = next_account_info(account_iter)?;

        //7. token_a_mint_account(read only)
        let token_a_mint_account = next_account_info(account_iter)?;

        //8. token_b_mint_account(read only)
        let token_b_mint_account = next_account_info(account_iter)?;

        //9. system_program_account(read only)
        let system_program_account = next_account_info(account_iter)?;

        //10. amm_program_account (read only)
        let amm_program_account = next_account_info(account_iter)?;
        msg!("1");
        if mint_address_in != *token_a_mint_account.key {
            msg!("base mint address doesn't match the provided account");
            return Err(AMMError::InvalidMintAccount.into());
        }
        if mint_address_out != *token_b_mint_account.key {
            msg!("target mint address doesn't match the provided account");
            return Err(AMMError::InvalidMintAccount.into());
        }
        msg!("1");
        let (amm_token_account_pda, amm_token_account_bump) = get_lexicographical_token_pda(
            token_a_mint_account.key, token_b_mint_account.key,
            program_id
        );
        if amm_token_account_pda != *amm_token_account.key {
            msg!("Error: Invalid AMM token account provided.");
            return Err(AMMError::InvalidAMMTokenAccount.into());
        }
        msg!("1");
        //validating token a pool account
        let (token_a_pool_account_pda, token_a_pool_account_bump) = Pubkey::find_program_address(
            &[b"pool", amm_token_account_pda.as_array(), token_a_mint_account.key.as_array()],
            program_id
        );
        if token_a_pool_account_pda != *amm_token_a_pool_account.key {
            msg!("Error: Invalid token A pool account provided.");
            return Err(AMMError::InvalidPDA.into());
        }
        msg!("1");
        //validating token b pool account
        let (token_b_pool_account_pda, token_b_pool_account_bump) = Pubkey::find_program_address(
            &[b"pool", amm_token_account_pda.as_array(), token_b_mint_account.key.as_array()],
            program_id
        );
        if token_b_pool_account_pda != *amm_token_b_pool_account.key {
            msg!("Error: Invalid token B pool account provided.");
            return Err(AMMError::InvalidPDA.into());
        }
        msg!("1");
        let token_a_pool_data = spl_token_interface::state::Account::unpack_from_slice(
            &amm_token_a_pool_account.data.borrow()
        )?;
        // if token_b_pool_data.amount < minimum_amount_out {
        //     msg!("bpool: {}, out: {}", token_b_pool_data.amount, minimum_amount_out);
        //     return Err(AMMError::InsufficientTokenBalance.into());
        // }
        let token_b_pool_data = spl_token_interface::state::Account::unpack_from_slice(
            &amm_token_b_pool_account.data.borrow()
        )?;
        if token_b_pool_data.amount < minimum_amount_out {
            msg!("bpool: {}, out: {}", token_b_pool_data.amount, minimum_amount_out);
            return Err(AMMError::InsufficientTokenBalance.into());
        }
        msg!("1");
        let constant_k = token_a_pool_data.amount * token_b_pool_data.amount;

        let calculated_amount_in = (
            (constant_k as f64) /
                ((token_b_pool_data.amount as f64) - (minimum_amount_out as f64)) -
            (token_a_pool_data.amount as f64)
        ).ceil() as u64;
        msg!("1");
        msg!(
            "in: {}, out: {}, a: {}, b: {}",
            calculated_amount_in,
            minimum_amount_out,
            token_a_pool_data.amount,
            token_b_pool_data.amount
        );
        if calculated_amount_in > amount_in {
            msg!("calc: {}, a in: {}", calculated_amount_in, amount_in);
            return Err(AMMError::PriceTooHigh.into());
        }
        //moving tokens from swapper to token pool
        let token_a_admin_to_pool_transfer_instruction = spl_token_interface::instruction::transfer(
            spl_token_account.key,
            swapper_token_a_account.key,
            amm_token_a_pool_account.key,
            swapper_account.key,
            &[swapper_account.key],
            calculated_amount_in
        )?;
        msg!("swapper to pool");
        solana_program::program::invoke(
            &token_a_admin_to_pool_transfer_instruction,
            &[
                spl_token_account.clone(),
                swapper_token_a_account.clone(),
                amm_token_a_pool_account.clone(),
                swapper_account.clone(),
            ]
        )?;
        let token_b_transfer_instruction = spl_token_interface::instruction::transfer(
            spl_token_account.key,
            amm_token_b_pool_account.key,
            swapper_token_b_account.key,
            amm_token_b_pool_account.key,
            &[amm_token_b_pool_account.key],
            minimum_amount_out
        )?;
        msg!("pool to swapper");
        solana_program::program::invoke_signed(
            &token_b_transfer_instruction,
            &[
                spl_token_account.clone(),
                amm_token_b_pool_account.clone(),
                swapper_token_b_account.clone(),
                amm_token_b_pool_account.clone(),
            ],
            &[
                &[
                    b"pool",
                    amm_token_account_pda.as_array(),
                    token_b_mint_account.key.as_array(),
                    &[token_b_pool_account_bump],
                ],
            ]
        )?;

        Ok(())
    }
}

pub fn initialize_amm_pool_account<'a>(
    admin_account: &AccountInfo<'a>,
    program_id: &Pubkey,
    amm_token_account: &AccountInfo<'a>,
    amm_token_account_bump: u8,
    system_program_account: &AccountInfo<'a>,
    _spl_token_program_account: &AccountInfo<'a>,
    token_a_mint_account: &Pubkey,
    token_b_mint_account: &Pubkey,
    token_a_pool_account: &Pubkey,
    token_b_pool_account: &Pubkey
) -> ProgramResult {
    //creating amm pool account if not already created
    if amm_token_account.data.borrow().len() == 0 {
        let lp_token_mint_account_create_instruction =
            solana_system_interface::instruction::create_account(
                admin_account.key,
                amm_token_account.key,
                Rent::get()?.minimum_balance(AMMAccount::LEN),
                AMMAccount::LEN as u64,
                amm_token_account.key
            );
        msg!("5");
        solana_program::program::invoke_signed(
            &lp_token_mint_account_create_instruction,
            &[system_program_account.clone(), admin_account.clone(), amm_token_account.clone()],
            &[
                &[
                    token_a_mint_account.as_array(),
                    token_b_mint_account.as_array(),
                    &[amm_token_account_bump],
                ],
            ]
        )?;
    }

    //initializing lp token mint account data
    let lp_token_mint_account_data = AMMAccount::Initialized {
        pool_authority: *program_id,
        token_a_mint: *token_a_mint_account,
        token_b_mint: *token_b_mint_account,
        lp_token_mint: *program_id,
        token_a_pool: *token_a_pool_account,
        token_b_pool: *token_b_pool_account,
        trade_fee: 300000,
        // admin_fee: 0,
    };

    lp_token_mint_account_data.pack_into_slice(&mut amm_token_account.data.borrow_mut());

    Ok(())
}

pub fn initialize_lp_token_mint_account<'a>(
    admin_account: &AccountInfo<'a>,
    program_id: &Pubkey,
    amm_token_account_pda: &Pubkey,
    lp_token_mint_account: &AccountInfo<'a>,
    system_program_account: &AccountInfo<'a>,
    spl_token_program_account: &AccountInfo<'a>,
    lp_token_mint_account_bump: u8,
    token_a_mint_account: &AccountInfo<'a>,
    token_b_mint_account: &AccountInfo<'a>,
    sysvar_rent_account: &AccountInfo<'a>
) -> ProgramResult {
    if lp_token_mint_account.data.borrow().len() == 0 {
        let lp_token_mint_account_create_instruction =
            solana_system_interface::instruction::create_account(
                admin_account.key,
                lp_token_mint_account.key,
                Rent::get()?.minimum_balance(spl_token_interface::state::Mint::LEN),
                spl_token_interface::state::Mint::LEN as u64,
                spl_token_program_account.key
            );
        msg!("6");

        solana_program::program::invoke_signed(
            &lp_token_mint_account_create_instruction,
            &[system_program_account.clone(), admin_account.clone(), lp_token_mint_account.clone()],
            &[
                &[
                    b"mint",
                    amm_token_account_pda.as_array(),
                    token_a_mint_account.key.as_array(),
                    token_b_mint_account.key.as_array(),
                    &[lp_token_mint_account_bump],
                ],
            ]
        )?;
    }

    let lp_token_mint_instruction = spl_token_interface::instruction::initialize_mint(
        spl_token_program_account.key,
        lp_token_mint_account.key,
        lp_token_mint_account.key,
        Some(lp_token_mint_account.key),
        0
    )?;
    msg!("7");

    let _ = solana_program::program::invoke_signed(
        &lp_token_mint_instruction,
        &[
            spl_token_program_account.clone(),
            lp_token_mint_account.clone(),
            sysvar_rent_account.clone(),
        ],
        &[
            &[
                b"mint",
                amm_token_account_pda.as_array(),
                token_a_mint_account.key.as_array(),
                token_b_mint_account.key.as_array(),
                &[lp_token_mint_account_bump],
            ],
        ]
    )?;
    Ok(())
}

pub fn initialize_token_pool_accounts<'a>(
    admin_account: &AccountInfo<'a>,
    program_id: &Pubkey,
    amm_token_account_pda: &Pubkey,
    system_program_account: &AccountInfo<'a>,
    spl_token_program_account: &AccountInfo<'a>,
    token_a_mint_account: &AccountInfo<'a>,
    token_b_mint_account: &AccountInfo<'a>,
    token_a_pool_account: &AccountInfo<'a>,
    token_b_pool_account: &AccountInfo<'a>,
    token_a_pool_account_bump: u8,
    token_b_pool_account_bump: u8,
    amm_token_account: &AccountInfo<'a>,
    sysvar_rent_account: &AccountInfo<'a>,
    amm_program_account: &AccountInfo<'a>
) -> ProgramResult {
    if token_a_pool_account.data.borrow().len() == 0 {
        let token_a_pool_account_create_instruction =
            solana_system_interface::instruction::create_account(
                admin_account.key,
                token_a_pool_account.key,
                Rent::get()?.minimum_balance(spl_token_interface::state::Account::LEN),
                spl_token_interface::state::Account::LEN as u64,
                spl_token_program_account.key
            );
        msg!("8 : {}", token_a_pool_account.key);

        solana_program::program::invoke_signed(
            &token_a_pool_account_create_instruction,
            &[system_program_account.clone(), admin_account.clone(), token_a_pool_account.clone()],
            &[
                &[
                    b"pool",
                    amm_token_account_pda.as_array(),
                    token_a_mint_account.key.as_array(),
                    &[token_a_pool_account_bump],
                ],
            ]
        )?;
    }
    if token_b_pool_account.data.borrow().len() == 0 {
        let token_b_pool_account_create_instruction =
            solana_system_interface::instruction::create_account(
                admin_account.key,
                token_b_pool_account.key,
                Rent::get()?.minimum_balance(spl_token_interface::state::Account::LEN),
                spl_token_interface::state::Account::LEN as u64,
                spl_token_program_account.key
            );
        msg!("9");

        solana_program::program::invoke_signed(
            &token_b_pool_account_create_instruction,
            &[system_program_account.clone(), admin_account.clone(), token_b_pool_account.clone()],
            &[
                &[
                    b"pool",
                    amm_token_account_pda.as_array(),
                    token_b_mint_account.key.as_array(),
                    &[token_b_pool_account_bump],
                ],
            ]
        )?;
    }

    let token_a_pool_account_initialize_instruction =
        spl_token_interface::instruction::initialize_account(
            spl_token_program_account.key,
            token_a_pool_account.key,
            token_a_mint_account.key,
            token_a_pool_account.key
        )?;
    msg!("10");

    let _ = solana_program::program::invoke_signed(
        &token_a_pool_account_initialize_instruction,
        &[
            spl_token_program_account.clone(),
            token_a_pool_account.clone(),
            token_a_mint_account.clone(),
            amm_program_account.clone(),
            sysvar_rent_account.clone(),
        ],
        &[
            &[
                b"pool",
                amm_token_account_pda.as_array(),
                token_a_mint_account.key.as_array(),
                &[token_a_pool_account_bump],
            ],
        ]
    );

    let token_b_pool_account_initialize_instruction =
        spl_token_interface::instruction::initialize_account(
            spl_token_program_account.key,
            token_b_pool_account.key,
            token_b_mint_account.key,
            token_b_pool_account.key
        )?;
    msg!("11");

    let _ = solana_program::program::invoke_signed(
        &token_b_pool_account_initialize_instruction,
        &[
            spl_token_program_account.clone(),
            token_b_pool_account.clone(),
            token_b_mint_account.clone(),
            amm_program_account.clone(),
            sysvar_rent_account.clone(),
        ],
        &[
            &[
                b"pool",
                amm_token_account_pda.as_array(),
                token_b_mint_account.key.as_array(),
                &[token_b_pool_account_bump],
            ],
        ]
    );
    Ok(())
}

pub fn initialize_admin_lp_token_account<'a>(
    admin_account: &AccountInfo<'a>,
    program_id: &Pubkey,
    amm_token_account_pda: &Pubkey,
    admin_lp_token_account: &AccountInfo<'a>,
    spl_token_program_account: &AccountInfo<'a>,
    system_program_account: &AccountInfo<'a>,
    lp_token_mint_account: &AccountInfo<'a>,
    token_a_mint_account: &AccountInfo<'a>,
    token_b_mint_account: &AccountInfo<'a>,
    admin_lp_token_account_bump: u8,
    sysvar_rent_account: &AccountInfo<'a>,
    amm_program_account: &AccountInfo<'a>
) -> ProgramResult {
    if admin_lp_token_account.data.borrow().len() == 0 {
        let admin_lp_token_account_create_instruction =
            solana_system_interface::instruction::create_account(
                admin_account.key,
                admin_lp_token_account.key,
                Rent::get()?.minimum_balance(spl_token_interface::state::Account::LEN),
                spl_token_interface::state::Account::LEN as u64,
                spl_token_program_account.key
            );
        msg!("12");

        solana_program::program::invoke_signed(
            &admin_lp_token_account_create_instruction,
            &[
                system_program_account.clone(),
                admin_account.clone(),
                admin_lp_token_account.clone(),
            ],
            &[
                &[
                    admin_account.key.as_array(),
                    amm_token_account_pda.as_array(),
                    token_a_mint_account.key.as_array(),
                    token_b_mint_account.key.as_array(),
                    &[admin_lp_token_account_bump],
                ],
            ]
        )?;
    }
    let spl_state = spl_token_interface::state::Account::unpack_from_slice(
        &admin_lp_token_account.data.borrow()
    )?;
    if !spl_state.is_initialized() {
        let admin_lp_token_account_initialize_instruction =
            spl_token_interface::instruction::initialize_account(
                spl_token_program_account.key,
                admin_lp_token_account.key,
                lp_token_mint_account.key,
                admin_account.key
            )?;
        msg!("13");
        msg!(
            "{}, {}, {}, {}",
            spl_token_program_account.key,
            admin_lp_token_account.key,
            lp_token_mint_account.key,
            sysvar_rent_account.key
        );
        solana_program::program::invoke_signed(
            &admin_lp_token_account_initialize_instruction,
            &[
                spl_token_program_account.clone(),
                admin_lp_token_account.clone(),
                lp_token_mint_account.clone(),
                amm_program_account.clone(),
                sysvar_rent_account.clone(),
            ],
            &[
                &[
                    admin_account.key.as_array(),
                    amm_token_account_pda.as_array(),
                    token_a_mint_account.key.as_array(),
                    token_b_mint_account.key.as_array(),
                    &[admin_lp_token_account_bump],
                ],
            ]
        )?;
    }
    Ok(())
}

pub fn get_lexicographical_token_pda(a: &Pubkey, b: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    if *a.to_string() > *b.to_string() {
        Pubkey::find_program_address(&[b.as_array(), a.as_array()], program_id)
    } else {
        Pubkey::find_program_address(&[a.as_array(), b.as_array()], program_id)
    }
}

pub fn get_lexicographical_mint_pda(
    str: &[u8],
    amm_token_account: &Pubkey,
    a: &Pubkey,
    b: &Pubkey,
    program_id: &Pubkey
) -> (Pubkey, u8) {
    if *a.to_string() > *b.to_string() {
        Pubkey::find_program_address(
            &[str, amm_token_account.as_array(), b.as_array(), a.as_array()],
            program_id
        )
    } else {
        Pubkey::find_program_address(
            &[str, amm_token_account.as_array(), a.as_array(), b.as_array()],
            program_id
        )
    }
}

pub fn get_lexicographical_lp_token_pda(
    owner_address: &Pubkey,
    amm_token_account: &Pubkey,
    a: &Pubkey,
    b: &Pubkey,
    program_id: &Pubkey
) -> (Pubkey, u8) {
    if *a.to_string() > *b.to_string() {
        Pubkey::find_program_address(
            &[owner_address.as_array(), amm_token_account.as_array(), b.as_array(), a.as_array()],
            program_id
        )
    } else {
        Pubkey::find_program_address(
            &[owner_address.as_array(), amm_token_account.as_array(), a.as_array(), b.as_array()],
            program_id
        )
    }
}
