# pinocchio-raydium-locking-program

Pinocchio helpers to invoke Raydium Locking Instructions (IXs) for both CP (Constant Product) and CLMM (Concentrated Liquidity) pools.

## Features

- **Lock CP Liquidity**: Lock constant product pool liquidity with optional metadata
- **Collect CP Fees**: Collect fees and rewards from locked CP positions
- **Lock CLMM Position**: Lock concentrated liquidity positions with optional metadata
- **Collect CLMM Fees**: Collect fees and rewards from locked CLMM positions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pinocchio-raydium-locking-program = "0.1.1"
```

## Usage

### Lock CP Liquidity

```rust
use pinocchio_raydium_locking_program::instructions::LockCpLiquidity;

let lock_ix = LockCpLiquidity {
    authority: &authority_account,
    payer: &payer_account,
    liquidity_owner: &liquidity_owner_account,
    fee_nft_owner: &fee_nft_owner_account,
    fee_nft_mint: &fee_nft_mint_account,
    fee_nft_account: &fee_nft_account,
    pool_state: &pool_state_account,
    locked_liquidity: &locked_liquidity_account,
    lp_mint: &lp_mint_account,
    liquidity_owner_lp: &liquidity_owner_lp_account,
    locked_lp_vault: &locked_lp_vault_account,
    token_0_vault: &token_0_vault_account,
    token_1_vault: &token_1_vault_account,
    metadata_account: &metadata_account,
    rent: &rent_account,
    system_program: &system_program_account,
    token_program: &token_program_account,
    associated_token_program: &associated_token_program_account,
    metadata_program: &metadata_program_account,
    lp_amount: 1000000,
    with_metadata: true,
};

lock_ix.invoke()?;
```

### Collect CP Fees

```rust
use pinocchio_raydium_locking_program::instructions::CollectCpFee;

let collect_ix = CollectCpFee {
    authority: &authority_account,
    fee_nft_owner: &fee_nft_owner_account,
    fee_nft_account: &fee_nft_account,
    locked_liquidity: &locked_liquidity_account,
    cpmm_program: &cpmm_program_account,
    cp_authority: &cp_authority_account,
    pool_state: &pool_state_account,
    lp_mint: &lp_mint_account,
    recipient_token_0_account: &recipient_token_0_account,
    recipient_token_1_account: &recipient_token_1_account,
    token_0_vault: &token_0_vault_account,
    token_1_vault: &token_1_vault_account,
    vault_0_mint: &vault_0_mint_account,
    vault_1_mint: &vault_1_mint_account,
    locked_lp_vault: &locked_lp_vault_account,
    token_program: &token_program_account,
    token_program_2022: &token_program_2022_account,
    memo_program: &memo_program_account,
    fee_lp_amount: 500000,
};

collect_ix.invoke()?;
```

### Lock CLMM Position

```rust
use pinocchio_raydium_locking_program::instructions::LockClmmPosition;

let lock_ix = LockClmmPosition {
    authority: &authority_account,
    payer: &payer_account,
    position_nft_owner: &position_nft_owner_account,
    fee_nft_owner: &fee_nft_owner_account,
    position_nft_account: &position_nft_account,
    personal_position: &personal_position_account,
    position_nft_mint: &position_nft_mint_account,
    locked_nft_account: &locked_nft_account,
    locked_position: &locked_position_account,
    fee_nft_mint: &fee_nft_mint_account,
    fee_nft_account: &fee_nft_account,
    metadata_account: &metadata_account,
    metadata_program: &metadata_program_account,
    associated_token_program: &associated_token_program_account,
    rent: &rent_account,
    fee_nft_token_program: &fee_nft_token_program_account,
    locked_nft_token_program: &locked_nft_token_program_account,
    system_program: &system_program_account,
    with_metadata: true,
};

lock_ix.invoke()?;
```

### Collect CLMM Fees

```rust
use pinocchio_raydium_locking_program::instructions::CollectClmmFeeAndReward;

let collect_ix = CollectClmmFeeAndReward {
    authority: &authority_account,
    fee_nft_owner: &fee_nft_owner_account,
    fee_nft_account: &fee_nft_account,
    locked_position: &locked_position_account,
    clmm_program: &clmm_program_account,
    locked_nft_account: &locked_nft_account,
    personal_position: &personal_position_account,
    pool_state: &pool_state_account,
    protocol_position: &protocol_position_account,
    token_0_vault: &token_0_vault_account,
    token_1_vault: &token_1_vault_account,
    tick_array_lower: &tick_array_lower_account,
    tick_array_upper: &tick_array_upper_account,
    recipient_token_0_account: &recipient_token_0_account,
    recipient_token_1_account: &recipient_token_1_account,
    token_program: &token_program_account,
    token_program_2022: &token_program_2022_account,
    memo_program: &memo_program_account,
    vault_0_mint: &vault_0_mint_account,
    vault_1_mint: &vault_1_mint_account,
};

collect_ix.invoke()?;
```

## Dependencies

- `pinocchio = "0.8.4"` - Pinocchio framework for Solana programs
- `pinocchio-system = "0.2.3"` - Pinocchio system program helpers

## License

Licensed under the Apache License, Version 2.0.

