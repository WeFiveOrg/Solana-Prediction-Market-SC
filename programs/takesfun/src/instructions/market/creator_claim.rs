use crate::{
    constants::{ CREATOR,  MARKET},
    errors::*,
    state::{market::*},
};

use crate::utils::*;
use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    token::{ Mint},
};

#[derive(Accounts)]
pub struct CreeatorClaim<'info> {

    #[account(
        mut,
        constraint = market.creator == creator.key() @TakesFunError::IncorrectAuthority
    )]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKET.as_bytes(), &yes_token.key().to_bytes(), &no_token.key().to_bytes()], 
        bump
    )]
    market: Account<'info, Market>,

    /// CHECK: CREATOR vault pda which stores SOL
    #[account(
        mut,
        seeds = [CREATOR.as_bytes(), &creator.key().to_bytes(), &market.key().to_bytes()],
        bump,
    )]
    pub creator_vault: AccountInfo<'info>,

    pub yes_token: Box<Account<'info, Mint>>,

    pub no_token: Box<Account<'info, Mint>>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

}

impl<'info> CreeatorClaim<'info> { 
    pub fn handler(&mut self, creator_vault_bump:u8) -> Result<()> {

    let signer_seeds: &[&[&[u8]]] = &[&[
        CREATOR.as_bytes(),
        &self.creator.key().to_bytes(),
        &self.market.key().to_bytes(),
        &[creator_vault_bump],
    ]];

    let creator_vault = &mut self.creator_vault.to_account_info();
  
     // Transfer SOL to user
     sol_transfer_with_signer(
        creator_vault.clone(),
        self.creator.to_account_info(),
        &self.system_program,
        signer_seeds,
        creator_vault.lamports() - 1000000,
    )?;
    msg!("SOL to user transfer complete");

        Ok(())
    }
}