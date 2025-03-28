use crate::{
    constants::{CONFIG, MARKET, },
    errors::*,
    state::{config::*, market::*},
    utils::*,
};
use anchor_lang::{prelude::*,};
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction(market_info: String)]
pub struct ChangeCreator<'info> {
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        mut,
        constraint = admin.key() == global_config.backend_sign_authority.key() @ TakesFunError::IncorrectAuthority
    )]
    pub admin: Signer<'info>,

    pub yes_token: Box<Account<'info, Mint>>,

    pub no_token: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [MARKET.as_bytes(), &market_info.to_hashed_bytes()], 
        bump
    )]
    market: Account<'info, Market>,
}

impl<'info> ChangeCreator<'info> {
    pub fn handler(&mut self, market_info: String, new_creator: Pubkey) -> Result<()> {
        let market = &mut self.market;
        market.creator = new_creator;   
        Ok(())
    }
}


