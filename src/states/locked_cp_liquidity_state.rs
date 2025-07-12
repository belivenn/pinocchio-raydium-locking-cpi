use pinocchio::{
    account_info::{AccountInfo, Ref},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::ID;

// Seed for LockedCpLiquidityState account
pub const LOCKED_LIQUIDITY_SEED: &str = "locked_liquidity";
pub const LOCK_CP_AUTH_SEED: &str = "lock_cp_authority_seed";

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LockedCpLiquidityState {
    pub locked_lp_amount: u64,
    pub claimed_lp_amount: u64,
    pub unclaimed_lp_amount: u64,
    pub last_lp: u64,
    pub last_k: u128,
    pub recent_epoch: u64,
    pub pool_id: Pubkey,
    pub fee_nft_mint: Pubkey,
    pub locked_owner: Pubkey,
    pub locked_lp_mint: Pubkey,
    pub padding: [u64; 8],
}

impl LockedCpLiquidityState {
    pub const LEN: usize = core::mem::size_of::<Self>() + 8;
    

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
