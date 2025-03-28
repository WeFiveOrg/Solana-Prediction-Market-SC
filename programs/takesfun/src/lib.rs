use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::{
    accept_authority::*, add_wl::*, change_creator::*, configure::*, create_market::*,
    create_market_second::*, creator_claim::*, creator_claim_second::*, mint_no_token::*,
    nominate_authority::*, swap::*, swap_second::*,
};
use state::config::*;

declare_id!("4D1RaYpBgEAj437RBaCkbKkpN2S2BA4CcmkE35MR1CZv");
#[program]
pub mod takesfun {
    use super::*;

    //  called by admin to set global config
    //  need to check the signer is authority
    pub fn configure(ctx: Context<Configure>, new_config: Config) -> Result<()> {
        msg!("configure: {:#?}", new_config);
        ctx.accounts.handler(new_config, ctx.bumps.config)
    }

    //  Admin can hand over admin role
    pub fn nominate_authority(ctx: Context<NominateAuthority>, new_admin: Pubkey) -> Result<()> {
        ctx.accounts.process(new_admin)
    }

    //  Pending admin should accept the admin role
    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn create_market(
        ctx: Context<CreateMarket>,
        // metadata
        yes_symbol: String,
        yes_uri: String,

        market_info: String,
    ) -> Result<()> {
        ctx.accounts
            .handler(yes_symbol, yes_uri, market_info, ctx.bumps.global_vault)
    }

    pub fn mint_no_token(
        ctx: Context<MintNoToken>,
        // metadata
        no_symbol: String,
        no_uri: String,
    ) -> Result<()> {
        ctx.accounts
            .handler(no_symbol, no_uri, ctx.bumps.global_vault)
    }

    //  amount - swap amount
    //  direction - 0: buy, 1: sell
    pub fn swap(
        ctx: Context<Swap>,
        amount: u64,
        direction: u8,
        token_type: u8,
        minimum_receive_amount: u64,
    ) -> Result<()> {
        ctx.accounts.handler(
            amount,
            direction,
            token_type,
            minimum_receive_amount,
            ctx.bumps.global_vault,
        )
    }

    pub fn add_wl(ctx: Context<AddWl>, new_whitelister: Pubkey) -> Result<()> {
        ctx.accounts.handler(new_whitelister)
    }

    pub fn creator_claim(ctx: Context<CreeatorClaim>) -> Result<()> {
        ctx.accounts.handler(ctx.bumps.creator_vault)
    }

    pub fn create_market_second(
        ctx: Context<CreateMarketSecond>, // metadata
        market_info: String,
        yes_symbol: String,
        yes_uri: String,
        creator_wallet: Pubkey,
    ) -> Result<()> {
        msg!("create_market_second: {:#?}", market_info);
        ctx.accounts.handler(
            market_info,
            yes_symbol,
            yes_uri,
            creator_wallet,
            ctx.bumps.global_vault,
        )
    }

    //  amount - swap amount
    //  direction - 0: buy, 1: sell
    pub fn swap_second(
        ctx: Context<SwapSecond>,
        market_info: String,
        amount: u64,
        direction: u8,
        token_type: u8,
        minimum_receive_amount: u64,
    ) -> Result<()> {
        ctx.accounts.handler(
            market_info,
            amount,
            direction,
            token_type,
            minimum_receive_amount,
            ctx.bumps.global_vault,
        )
    }

    pub fn creator_claim_second(
        ctx: Context<CreeatorClaimSecond>,
        market_info: String,
    ) -> Result<()> {
        ctx.accounts.handler(market_info, ctx.bumps.creator_vault)
    }

    pub fn change_creator(
        ctx: Context<ChangeCreator>,
        market_info: String,
        new_creator: Pubkey,
    ) -> Result<()> {
        ctx.accounts.handler(market_info, new_creator)
    }
}
