use core::slice::from_raw_parts;
use core::mem::MaybeUninit;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

use crate::{write_bytes, UNINIT_BYTE};

pub struct LockCpLiquidity<'a> {
    pub authority: &'a AccountInfo,              // []          (readonly)
    pub payer: &'a AccountInfo,                  // [SIGNER]    (signer + mutable)
    pub liquidity_owner: &'a AccountInfo,        // [SIGNER]    (signer)
    pub fee_nft_owner: &'a AccountInfo,          // []          (readonly)
    pub fee_nft_mint: &'a AccountInfo,           // [WRITE]     (mutable)
    pub fee_nft_account: &'a AccountInfo,        // [WRITE]     (mutable)
    pub pool_state: &'a AccountInfo,             // []          (readonly)
    pub locked_liquidity: &'a AccountInfo,       // [WRITE]     (mutable)
    pub lp_mint: &'a AccountInfo,                // [WRITE]     (mutable)    
    pub liquidity_owner_lp: &'a AccountInfo,     // [WRITE]     (mutable)
    pub locked_lp_vault: &'a AccountInfo,        // [WRITE]     (mutable)
    pub token_0_vault: &'a AccountInfo,          // [WRITE]     (mutable)
    pub token_1_vault: &'a AccountInfo,          // [WRITE]     (mutable)   
    pub metadata_account: &'a AccountInfo,       // [WRITE]     (mutable)
    pub rent: &'a AccountInfo,                   // []          (readonly)   
    pub system_program: &'a AccountInfo,         // []          (readonly)
    pub token_program: &'a AccountInfo,          // []          (readonly)
    pub associated_token_program: &'a AccountInfo,// []         (readonly)
    pub metadata_program: &'a AccountInfo,       // []          (readonly)
    
    pub lp_amount: u64,
    pub with_metadata: bool,
}


impl LockCpLiquidity<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let account_metas: [AccountMeta; 19] = [
            AccountMeta::readonly(self.authority.key()),
            AccountMeta::writable_signer(self.payer.key()),
            AccountMeta::readonly_signer(self.liquidity_owner.key()),
            AccountMeta::readonly(self.fee_nft_owner.key()),
            AccountMeta::writable(self.fee_nft_mint.key()),
            AccountMeta::writable(self.fee_nft_account.key()),
            AccountMeta::readonly(self.pool_state.key()),
            AccountMeta::writable(self.locked_liquidity.key()),
            AccountMeta::writable(self.lp_mint.key()),
            AccountMeta::writable(self.liquidity_owner_lp.key()),
            AccountMeta::writable(self.locked_lp_vault.key()),
            AccountMeta::writable(self.token_0_vault.key()),
            AccountMeta::writable(self.token_1_vault.key()),
            AccountMeta::writable(self.metadata_account.key()),
            AccountMeta::readonly(self.rent.key()),
            AccountMeta::readonly(self.system_program.key()),
            AccountMeta::readonly(self.token_program.key()),
            AccountMeta::readonly(self.associated_token_program.key()),
            AccountMeta::readonly(self.metadata_program.key()),
        ];
        // Instruction data layout:
        // -  [0..8]: instruction discriminator (8 bytes)
        // -  [8..16]: lp_amount (8 bytes, u64)
        // -  [16]: with_metadata (1 byte, bool)
        let mut instruction_data = [UNINIT_BYTE; 17];

        // Set discriminator (8 bytes)
        let discriminator: [u8; 8] = [0xd8, 0x9d, 0x1d, 0x4e, 0x26, 0x33, 0x1f, 0x1a];
        write_bytes(&mut instruction_data[0..8], &discriminator);
        
        // Set lp_token_amount (8 bytes)
        write_bytes(&mut instruction_data[8..16], &self.lp_amount.to_le_bytes());

        // Set with_metadata (1 bytes)
        write_bytes(&mut instruction_data[16..], &[self.with_metadata as u8]);
        
        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 17) },
        };

        let accounts = [
            self.authority, self.payer, self.liquidity_owner, self.fee_nft_owner,
            self.fee_nft_mint, self.fee_nft_account, self.pool_state, self.locked_liquidity,
            self.lp_mint, self.liquidity_owner_lp, self.locked_lp_vault,
            self.token_0_vault, self.token_1_vault, self.metadata_account,
            self.rent, self.system_program, self.token_program,
            self.associated_token_program, self.metadata_program,
        ];

        invoke_signed(&instruction, &accounts, signers)
    }
} 