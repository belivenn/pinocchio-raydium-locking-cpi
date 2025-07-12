use core::slice::from_raw_parts;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

use crate::{write_bytes, UNINIT_BYTE};

pub struct CollectClmmFeeAndReward<'a> {
    pub authority: &'a AccountInfo,                    // []          (readonly)
    pub fee_nft_owner: &'a AccountInfo,                // [SIGNER]    (signer)
    pub fee_nft_account: &'a AccountInfo,              // []          (readonly)
    pub locked_position: &'a AccountInfo,              // []          (readonly)
    pub clmm_program: &'a AccountInfo,                 // []          (readonly)
    pub locked_nft_account: &'a AccountInfo,           // [WRITE]     (mutable)
    pub personal_position: &'a AccountInfo,            // [WRITE]     (mutable)
    pub pool_state: &'a AccountInfo,                   // [WRITE]     (mutable)
    pub protocol_position: &'a AccountInfo,            // [WRITE]     (mutable)
    pub token_0_vault: &'a AccountInfo,                // [WRITE]     (mutable)
    pub token_1_vault: &'a AccountInfo,                // [WRITE]     (mutable)
    pub tick_array_lower: &'a AccountInfo,             // [WRITE]     (mutable)
    pub tick_array_upper: &'a AccountInfo,             // [WRITE]     (mutable)
    pub recipient_token_0_account: &'a AccountInfo,    // [WRITE]     (mutable)
    pub recipient_token_1_account: &'a AccountInfo,    // [WRITE]     (mutable)
    pub token_program: &'a AccountInfo,                // []          (readonly)
    pub token_program_2022: &'a AccountInfo,           // []          (readonly)
    pub memo_program: &'a AccountInfo,                 // []          (readonly)
    pub vault_0_mint: &'a AccountInfo,                 // []          (readonly)
    pub vault_1_mint: &'a AccountInfo,                 // []          (readonly)
}

impl CollectClmmFeeAndReward<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let account_metas: [AccountMeta; 20] = [
            AccountMeta::readonly(self.authority.key()),
            AccountMeta::readonly_signer(self.fee_nft_owner.key()),
            AccountMeta::readonly(self.fee_nft_account.key()),
            AccountMeta::readonly(self.locked_position.key()),
            AccountMeta::readonly(self.clmm_program.key()),
            AccountMeta::writable(self.locked_nft_account.key()),
            AccountMeta::writable(self.personal_position.key()),
            AccountMeta::writable(self.pool_state.key()),
            AccountMeta::writable(self.protocol_position.key()),
            AccountMeta::writable(self.token_0_vault.key()),
            AccountMeta::writable(self.token_1_vault.key()),
            AccountMeta::writable(self.tick_array_lower.key()),
            AccountMeta::writable(self.tick_array_upper.key()),
            AccountMeta::writable(self.recipient_token_0_account.key()),
            AccountMeta::writable(self.recipient_token_1_account.key()),
            AccountMeta::readonly(self.token_program.key()),
            AccountMeta::readonly(self.token_program_2022.key()),
            AccountMeta::readonly(self.memo_program.key()),
            AccountMeta::readonly(self.vault_0_mint.key()),
            AccountMeta::readonly(self.vault_1_mint.key()),
        ];

        // Instruction data layout:
        // -  [0..8]: instruction discriminator (8 bytes)
        // No additional parameters needed for this instruction
        let mut instruction_data = [UNINIT_BYTE; 8];

        // Set discriminator (8 bytes)
        let discriminator: [u8; 8] = [0x95, 0xcb, 0xcc, 0x11, 0x2d, 0x7a, 0x96, 0x9a]; 
        write_bytes(&mut instruction_data[0..8], &discriminator);

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 8) },
        };

        let accounts = [
            self.authority, self.fee_nft_owner, self.fee_nft_account, self.locked_position,
            self.clmm_program, self.locked_nft_account, self.personal_position, self.pool_state,
            self.protocol_position, self.token_0_vault, self.token_1_vault, self.tick_array_lower,
            self.tick_array_upper, self.recipient_token_0_account, self.recipient_token_1_account,
            self.token_program, self.token_program_2022, self.memo_program, self.vault_0_mint,
            self.vault_1_mint,
        ];

        invoke_signed(&instruction, &accounts, signers)
    }
} 