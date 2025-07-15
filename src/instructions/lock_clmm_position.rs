use core::slice::from_raw_parts;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

use crate::{write_bytes, UNINIT_BYTE};

pub struct LockClmmPosition<'a> {
    pub authority: &'a AccountInfo,                    // []          (readonly)
    pub payer: &'a AccountInfo,                        // [SIGNER]    (signer + mutable)
    pub position_nft_owner: &'a AccountInfo,           // [SIGNER]    (signer)
    pub fee_nft_owner: &'a AccountInfo,                // []          (readonly)
    pub position_nft_account: &'a AccountInfo,         // [WRITE]     (mutable)
    pub personal_position: &'a AccountInfo,            // []          (readonly)
    pub position_nft_mint: &'a AccountInfo,            // []          (readonly)
    pub locked_nft_account: &'a AccountInfo,           // [WRITE]     (mutable)
    pub locked_position: &'a AccountInfo,              // [WRITE]     (mutable)
    pub fee_nft_mint: &'a AccountInfo,                 // [WRITE]     (mutable)
    pub fee_nft_account: &'a AccountInfo,              // [WRITE]     (mutable)
    pub metadata_account: &'a AccountInfo,             // [WRITE]     (mutable)
    pub metadata_program: &'a AccountInfo,             // []          (readonly)
    pub associated_token_program: &'a AccountInfo,     // []          (readonly)
    pub rent: &'a AccountInfo,                         // []          (readonly)
    pub fee_nft_token_program: &'a AccountInfo,        // []          (readonly)
    pub locked_nft_token_program: &'a AccountInfo,     // []          (readonly)
    pub system_program: &'a AccountInfo,               // []          (readonly)
    
    pub with_metadata: bool,
}

impl LockClmmPosition<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let account_metas: [AccountMeta; 18] = [
            AccountMeta::readonly(self.authority.key()),
            AccountMeta::writable_signer(self.payer.key()),
            AccountMeta::readonly_signer(self.position_nft_owner.key()),
            AccountMeta::readonly(self.fee_nft_owner.key()),
            AccountMeta::writable(self.position_nft_account.key()),
            AccountMeta::readonly(self.personal_position.key()),
            AccountMeta::readonly(self.position_nft_mint.key()),
            AccountMeta::writable(self.locked_nft_account.key()),
            AccountMeta::writable(self.locked_position.key()),
            AccountMeta::writable(self.fee_nft_mint.key()),
            AccountMeta::writable(self.fee_nft_account.key()),
            AccountMeta::writable(self.metadata_account.key()),
            AccountMeta::readonly(self.metadata_program.key()),
            AccountMeta::readonly(self.associated_token_program.key()),
            AccountMeta::readonly(self.rent.key()),
            AccountMeta::readonly(self.fee_nft_token_program.key()),
            AccountMeta::readonly(self.locked_nft_token_program.key()),
            AccountMeta::readonly(self.system_program.key()),
        ];

        // Instruction data layout:
        // -  [0..8]: instruction discriminator (8 bytes)
        // -  [8]: with_metadata (1 byte, bool)
        let mut instruction_data = [UNINIT_BYTE; 9];

        // Set discriminator (8 bytes) bc25b38352965449
        let discriminator: [u8; 8] = [0xbc, 0x25, 0xb3, 0x83, 0x52, 0x96, 0x54, 0x49]; 
        write_bytes(&mut instruction_data[0..8], &discriminator);
        
        // Set with_metadata (1 byte)
        write_bytes(&mut instruction_data[8..], &[self.with_metadata as u8]);

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 9) },
        };

        let accounts = [
            self.authority, self.payer, self.position_nft_owner, self.fee_nft_owner,
            self.position_nft_account, self.personal_position, self.position_nft_mint,
            self.locked_nft_account, self.locked_position, self.fee_nft_mint,
            self.fee_nft_account, self.metadata_account, self.metadata_program,
            self.associated_token_program, self.rent, self.fee_nft_token_program,
            self.locked_nft_token_program, self.system_program,
        ];

        invoke_signed(&instruction, &accounts, signers)
    }
} 