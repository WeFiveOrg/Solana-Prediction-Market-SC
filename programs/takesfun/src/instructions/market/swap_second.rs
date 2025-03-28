use anchor_lang::{system_program, prelude::*};
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token},
};
use crate::{
    constants::{MARKET, CONFIG, GLOBAL, WHITELIST, CREATOR}, 
    errors::*, 
    state::{market::*,  config::*, whitelist::*},
    utils::*
};

#[derive(Accounts)]
#[instruction(market_info: String)]
pub struct SwapSecond<'info> {
    #[account(
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,
    
    /// CHECK: should be same with the address in the global_config
    #[account(
        mut,
        constraint = global_config.team_wallet == team_wallet.key() @TakesFunError::IncorrectAuthority
    )]
    pub team_wallet: AccountInfo<'info>,

     /// CHECK: should be same with the address in the global_config
     #[account(
        mut,
        constraint = global_config.team_wallet2 == team_wallet2.key() @TakesFunError::IncorrectAuthority
    )]
    pub team_wallet2: AccountInfo<'info>,

     /// CHECK: should be same with the address in the global_config
     #[account(
        mut,
        constraint = market.creator == creator.key() @TakesFunError::IncorrectAuthority
    )]
    pub creator: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [MARKET.as_bytes(), &market_info.to_hashed_bytes()], 
        bump
    )]
    market: Account<'info, Market>,

    /// CHECK: global vault pda which stores SOL
    #[account(
        mut,
        seeds = [GLOBAL.as_bytes()],
        bump,
    )]
    pub global_vault: AccountInfo<'info>,

    /// CHECK: CREATOR vault pda which stores SOL
    #[account(
        mut,
        seeds = [CREATOR.as_bytes(), &market.key().to_bytes()],
        bump,
    )]
    pub creator_vault: AccountInfo<'info>,

    pub yes_token: Box<Account<'info, Mint>>,

    pub no_token: Box<Account<'info, Mint>>,

    /// CHECK: yes ata of global vault
    #[account(
        mut,
        seeds = [
            global_vault.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            yes_token.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    global_yes_ata: AccountInfo<'info>,

    /// CHECK: no ata of global vault
    #[account(
        mut,
        seeds = [
            global_vault.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            no_token.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    global_no_ata: AccountInfo<'info>,

    /// CHECK: ata of user
    #[account(
        mut,
        seeds = [
            user.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            yes_token.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    user_yes_ata: AccountInfo<'info>,

    /// CHECK: ata of user
    #[account(
        mut,
        seeds = [
            user.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            no_token.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    user_no_ata: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<Whitelist>(),
        seeds = [WHITELIST.as_bytes(), &user.key().to_bytes()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> SwapSecond<'info> { 
pub fn handler(&mut self, market_info: String, amount: u64, direction: u8, token_type: u8, minimum_receive_amount: u64,global_vault_bump:u8) -> Result<()> {

    msg!("amount: {:?}, direction: {:?}, token_type: {:?}, minimum_receive_amount: {:?}", amount, direction, token_type, minimum_receive_amount);
    let market = &mut self.market;

    let source = &mut self.global_vault.to_account_info();
    let creator_vault = &mut self.creator_vault.to_account_info();
    let team_wallet = &mut self.team_wallet;
    let team_wallet2 = &mut self.team_wallet2;

    let yes_token = &mut self.yes_token;
    let user_yes_ata = &mut self.user_yes_ata;

    let no_token = &mut self.no_token;
    let user_no_ata = &mut self.user_no_ata;

    let whitelist = &mut self.whitelist;

    let current_timestamp = Clock::get()?.unix_timestamp;
    if whitelist.is_allow == 1 && whitelist.first_swap_timestamp == 0
    {
        whitelist.first_swap_timestamp = current_timestamp;
    }

    let is_small_fee: bool;
    if whitelist.is_allow == 1
    {
        is_small_fee = whitelist.is_whitelister(self.global_config.limit_timestamp, current_timestamp)?;
    }
    else
    {
        is_small_fee = false;
    }

    if token_type == 0{
        //  create user wallet no ata, if it doesn't exit
        if user_no_ata.data_is_empty() {
            anchor_spl::associated_token::create(CpiContext::new(
                self.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: self.user.to_account_info(),
                    associated_token: user_no_ata.to_account_info(),
                    authority: self.user.to_account_info(),

                    mint: no_token.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                }
            ))?;
        }
    }else{
        //  create user wallet yes ata, if it doesn't exit
        if user_yes_ata.data_is_empty() {
            anchor_spl::associated_token::create(CpiContext::new(
                self.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: self.user.to_account_info(),
                    associated_token: user_yes_ata.to_account_info(),
                    authority: self.user.to_account_info(),

                    mint: yes_token.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                }
            ))?;
        }
    }

    let signer_seeds: &[&[&[u8]]] = &[&[
        GLOBAL.as_bytes(),
        &[global_vault_bump],
    ]];

     market.swap(
        &*self.global_config,

        yes_token.as_ref(),
        &mut self.global_yes_ata,
        user_yes_ata,

        no_token.as_ref(),
        &mut self.global_no_ata,
        user_no_ata,

        source,
        team_wallet,
        team_wallet2,
        creator_vault,

        amount,
        direction,
        token_type,
        minimum_receive_amount,

        &self.user,
        signer_seeds,

        is_small_fee,

        &self.token_program,
        &self.system_program,
    )?;

    Ok(())
}

}