use core::slice::from_raw_parts;
use core::mem::MaybeUninit;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

use crate::{write_bytes, UNINIT_BYTE};

pub struct CollectCpFee<'a> {
    pub authority: &'a AccountInfo,                    // []          (readonly)
    pub fee_nft_owner: &'a AccountInfo,                // [SIGNER]    (signer)
    pub fee_nft_account: &'a AccountInfo,              // []          (readonly)
    pub locked_liquidity: &'a AccountInfo,             // [WRITE]     (mutable)
    pub cpmm_program: &'a AccountInfo,                 // []          (readonly)
    pub cp_authority: &'a AccountInfo,                 // []          (readonly)
    pub pool_state: &'a AccountInfo,                   // [WRITE]     (mutable)
    pub lp_mint: &'a AccountInfo,                      // [WRITE]     (mutable)
    pub recipient_token_0_account: &'a AccountInfo,    // [WRITE]     (mutable)
    pub recipient_token_1_account: &'a AccountInfo,    // [WRITE]     (mutable)
    pub token_0_vault: &'a AccountInfo,                // [WRITE]     (mutable)
    pub token_1_vault: &'a AccountInfo,                // [WRITE]     (mutable)
    pub vault_0_mint: &'a AccountInfo,                 // []          (readonly)
    pub vault_1_mint: &'a AccountInfo,                 // []          (readonly)
    pub locked_lp_vault: &'a AccountInfo,              // [WRITE]     (mutable)
    pub token_program: &'a AccountInfo,                // []          (readonly)
    pub token_program_2022: &'a AccountInfo,           // []          (readonly)
    pub memo_program: &'a AccountInfo,                 // []          (readonly)
    
    pub fee_lp_amount: u64,
}

impl CollectCpFee<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let account_metas: [AccountMeta; 18] = [
            AccountMeta::readonly(self.authority.key()),
            AccountMeta::readonly_signer(self.fee_nft_owner.key()),
            AccountMeta::readonly(self.fee_nft_account.key()),
            AccountMeta::writable(self.locked_liquidity.key()),
            AccountMeta::readonly(self.cpmm_program.key()),
            AccountMeta::readonly(self.cp_authority.key()),
            AccountMeta::writable(self.pool_state.key()),
            AccountMeta::writable(self.lp_mint.key()),
            AccountMeta::writable(self.recipient_token_0_account.key()),
            AccountMeta::writable(self.recipient_token_1_account.key()),
            AccountMeta::writable(self.token_0_vault.key()),
            AccountMeta::writable(self.token_1_vault.key()),
            AccountMeta::readonly(self.vault_0_mint.key()),
            AccountMeta::readonly(self.vault_1_mint.key()),
            AccountMeta::writable(self.locked_lp_vault.key()),
            AccountMeta::readonly(self.token_program.key()),
            AccountMeta::readonly(self.token_program_2022.key()),
            AccountMeta::readonly(self.memo_program.key()),
        ];

        // Instruction data layout:
        // -  [0..8]: instruction discriminator (8 bytes)
        // -  [8..16]: fee_lp_amount (8 bytes, u64)
        let mut instruction_data = [UNINIT_BYTE; 16];

        // Set discriminator (8 bytes)
        let discriminator: [u8; 8] = [0x08, 0x1e, 0x33, 0xc7, 0xd1, 0xb8, 0xf7, 0x85]; 
        write_bytes(&mut instruction_data[0..8], &discriminator);
        
        // Set fee_lp_amount (8 bytes)
        write_bytes(&mut instruction_data[8..16], &self.fee_lp_amount.to_le_bytes());

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 16) },
        };

        let accounts = [
            self.authority, self.fee_nft_owner, self.fee_nft_account, self.locked_liquidity,
            self.cpmm_program, self.cp_authority, self.pool_state, self.lp_mint,
            self.recipient_token_0_account, self.recipient_token_1_account, self.token_0_vault,
            self.token_1_vault, self.vault_0_mint, self.vault_1_mint, self.locked_lp_vault,
            self.token_program, self.token_program_2022, self.memo_program,
        ];

        invoke_signed(&instruction, &accounts, signers)
    }
} 