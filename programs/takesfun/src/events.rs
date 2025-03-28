use anchor_lang::prelude::*;

#[event]
pub struct LaunchEvent {
    pub creator: Pubkey,
    pub market: Pubkey,

    pub yes_mint: Pubkey,
    pub yes_metadata: Pubkey,
    pub yes_real_reserve_lamport: u64,
    pub yes_real_reserve_token: u64,
    pub yes_virtual_reserve_lamport: u64,
    pub yes_virtual_reserve_token: u64,

    pub no_mint: Pubkey,
    pub no_metadata: Pubkey,
    pub no_real_reserve_lamport: u64,
    pub no_real_reserve_token: u64,
    pub no_virtual_reserve_lamport: u64,
    pub no_virtual_reserve_token: u64,

    pub market_info: String,
    pub token_supply: u64,
    pub decimals: u8,

    pub market_type: u8,
}

#[event]
pub struct TradeEvent {
    pub user: Pubkey,
    pub yes_token: Pubkey,
    pub no_token: Pubkey,
    pub market: Pubkey,

    pub sol_amount: u64,
    pub token_amount: u64,
    pub platform_fee_lamports: u64,
    pub creator_fee_lamports: u64,
    pub shift_lamports_real: u64,
    pub shift_lamports_virtual: u64,

    pub direction: u8,
    pub token_type: u8,
    pub timestamp: i64,

    pub yes_real_reserve_lamport: u64,
    pub yes_real_reserve_token: u64,
    pub yes_virtual_reserve_lamport: u64,
    pub yes_virtual_reserve_token: u64,

    pub no_real_reserve_lamport: u64,
    pub no_real_reserve_token: u64,
    pub no_virtual_reserve_lamport: u64,
    pub no_virtual_reserve_token: u64,
}
