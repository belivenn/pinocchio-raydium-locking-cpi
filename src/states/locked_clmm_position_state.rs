use pinocchio::{
    account_info::{AccountInfo, Ref},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::ID;

// Seed for LockedClmmPositionState account
pub const LOCKED_POSITION_SEED: &str = "locked_position";
pub const LOCK_CLMM_AUTH_SEED: &str = "program_authority_seed";

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LockedClmmPositionState {
    pub bump: [u8; 1],
    pub position_owner: Pubkey,
    pub pool_id: Pubkey,
    pub position_id: Pubkey,
    pub locked_nft_account: Pubkey,
    pub fee_nft_mint: Pubkey,
    pub recent_epoch: u64,
    pub padding: [u64; 8],
}

impl LockedClmmPositionState {
    pub const LEN: usize = 8 + 1 + 32 + 32 + 32 + 32 + 32 + 8 + (8 * 8);

    #[inline]
    pub fn from_account_info(account_info: &AccountInfo) -> Result<Ref<Self>, ProgramError> {
        if account_info.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if !account_info.is_owned_by(&ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
            Self::from_bytes(&data[8..])
        }))
    }

    #[inline]
    pub unsafe fn from_account_info_unchecked(
        account_info: &AccountInfo,
    ) -> Result<&Self, ProgramError> {
        if account_info.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if account_info.owner() != &ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Self::from_bytes(&account_info.borrow_data_unchecked()[8..]))
    }

    #[inline(always)]
    pub unsafe fn from_bytes(bytes: &[u8]) -> &Self {
        &*(bytes.as_ptr() as *const Self)
    }
}
