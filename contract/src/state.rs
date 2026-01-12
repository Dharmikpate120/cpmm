use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use crate::error::AMMError;

/// The state of a counter account.
#[derive(BorshDeserialize, BorshSerialize)]
pub enum AMMAccount {
    Uninitialized,
    Initialized{
        pool_authority: Pubkey,
        token_a_mint: Pubkey,
        token_b_mint: Pubkey,
        lp_token_mint: Pubkey,
        token_a_pool: Pubkey,
        token_b_pool: Pubkey,
        trade_fee: u64,
        // admin_fee: u64,
    },
}

// The `Sealed` trait is a marker trait to prevent
// other crates from implementing `Pack` for our state.
impl Sealed for AMMAccount {}

impl IsInitialized for AMMAccount {
    /// Checks if the account has been initialized.
    fn is_initialized(&self) -> bool {
        match self{
            AMMAccount::Initialized{  .. } => true,
            _ => false,
        }
    }
}

impl Pack for AMMAccount {
    /// The length of the account's data in bytes.
    const LEN: usize = 1 + 32 + 32 + 32 + 32 + 32 + 32 + 8;
    //  + 8;

    /// Deserializes a byte slice into a [StakeAccount].
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let acc = Self::try_from_slice(src).map_err(|_| {
            msg!("Error: Failed to deserialize counter account data");
            ProgramError::InvalidAccountData
        })?;
        // if !acc.is_initialized{
        //     msg!("Error: Account is not initialized!");
        //     return Err(AMMError::AccountNotInitialized.into());
        // }
        Ok(acc)
    }

    /// Serializes a [StakeAccount] into a byte slice.
    fn pack_into_slice(&self, dst: &mut [u8]) {
        self.serialize(&mut &mut dst[..])
           .expect("Failed to serialize counter account");
    }
}
