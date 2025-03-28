use crate::{
    constants::{CONFIG, CREATOR, GLOBAL, MARKET, METADATA, YES_NAME},
    errors::*,
    events::LaunchEvent,
    state::{config::*, market::*},
    utils::*,
};

use anchor_lang::{prelude::*, solana_program::sysvar::SysvarId, system_program};
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    metadata::{self, mpl_token_metadata::types::DataV2, Metadata},
    token::{self, spl_token::instruction::AuthorityType, Mint, Token},
};

#[derive(Accounts)]
#[instruction(market_info: String, yes_symbol: String, yes_uri: String, creator_wallet: Pubkey)]
pub struct CreateMarketSecond<'info> {
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

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

    #[account(mut)]
    first_client: Signer<'info>,

    #[account(
        init,
        payer = first_client,
        mint::decimals = global_config.token_decimals_config,
        mint::authority = global_vault.key(),
    )]
    yes_token: Box<Account<'info, Mint>>,

    pub no_token: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = first_client,
        space = 320,
        seeds = [MARKET.as_bytes(),&market_info.to_hashed_bytes()],
        bump
    )]
    market: Box<Account<'info, Market>>,

    /// CHECK: passed to token metadata program
    #[account(mut,
        seeds = [
            METADATA.as_bytes(),
            metadata::ID.as_ref(),
            yes_token.key().as_ref(),
        ],
        bump,
        seeds::program = metadata::ID
    )]
    yes_token_metadata_account: UncheckedAccount<'info>,

    /// CHECK: passed to token metadata program
    #[account(
        mut,
        seeds = [
            METADATA.as_bytes(),
            metadata::ID.as_ref(),
            no_token.key().as_ref(),
        ],
        bump,
        seeds::program = metadata::ID
    )]
    no_token_metadata_account: UncheckedAccount<'info>,

    /// CHECK: created in instruction
    #[account(
        mut,
        seeds = [
            global_vault.key().as_ref(),
            token::spl_token::ID.as_ref(),
            yes_token.key().as_ref(),
        ],
        bump,
        seeds::program = associated_token::ID
    )]
    global_yes_token_account: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = admin.key() == global_config.backend_sign_authority.key() @ TakesFunError::IncorrectAuthority
    )]
    pub admin: Signer<'info>,

    #[account(address = system_program::ID)]
    system_program: Program<'info, System>,
    #[account(address = Rent::id())]
    rent: Sysvar<'info, Rent>,
    #[account(address = token::ID)]
    token_program: Program<'info, Token>,
    #[account(address = associated_token::ID)]
    associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = metadata::ID)]
    mpl_token_metadata_program: Program<'info, Metadata>,
}

impl<'info> CreateMarketSecond<'info> {
    pub fn handler(
        &mut self,

        market_info: String,
        // metadata
        yes_symbol: String,
        yes_uri: String,

        creator_wallet: Pubkey,

        global_vault_bump: u8,
    ) -> Result<()> {
        msg!("CreateMarket start");
        let global_config = &self.global_config;
        let first_client = &self.first_client;
        let yes_token = &self.yes_token;
        let no_token = &self.no_token;
        let global_yes_token_account = &self.global_yes_token_account;
        let market = &mut self.market;
        let global_vault = &self.global_vault;
        let yes_name = YES_NAME;

        // create token launch pda
        market.yes_token_mint = yes_token.key();
        market.no_token_mint = no_token.key();
        market.creator = creator_wallet;

        //yes
        market.virtual_yes_sol_reserves = global_config.initial_virtual_yes_sol_reserves_config;
        market.virtual_yes_token_reserves = global_config.initial_virtual_yes_token_reserves_config;
        market.real_yes_sol_reserves = 0;
        market.real_yes_token_reserves = global_config.initial_real_yes_token_reserves_config;

        //no
        market.virtual_no_sol_reserves = global_config.initial_virtual_no_sol_reserves_config;
        market.virtual_no_token_reserves = global_config.initial_virtual_no_token_reserves_config;
        market.real_no_sol_reserves = 0;
        market.real_no_token_reserves = global_config.initial_real_no_token_reserves_config;

        //market info
        market.market_info = market_info;

        // create global yes token account
        associated_token::create(CpiContext::new(
            self.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: first_client.to_account_info(),
                associated_token: global_yes_token_account.to_account_info(),
                authority: global_vault.to_account_info(),
                mint: yes_token.to_account_info(),
                token_program: self.token_program.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        ))?;

        let signer_seeds: &[&[&[u8]]] = &[&[GLOBAL.as_bytes(), &[global_vault_bump]]];

        // mint yes tokens to market
        token::mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                token::MintTo {
                    mint: yes_token.to_account_info(),
                    to: global_yes_token_account.to_account_info(),
                    authority: global_vault.to_account_info(),
                },
                signer_seeds,
            ),
            global_config.token_supply_config,
        )?;

        // create yes token metadata
        metadata::create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                self.mpl_token_metadata_program.to_account_info(),
                metadata::CreateMetadataAccountsV3 {
                    metadata: self.yes_token_metadata_account.to_account_info(),
                    mint: yes_token.to_account_info(),
                    mint_authority: global_vault.to_account_info(),
                    payer: first_client.to_account_info(),
                    update_authority: global_vault.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
                signer_seeds,
            ),
            DataV2 {
                name: yes_name.to_string(),
                symbol: yes_symbol,
                uri: yes_uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            false,
            true,
            None,
        )?;

        //  revoke mint authority
        token::set_authority(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                token::SetAuthority {
                    current_authority: global_vault.to_account_info(),
                    account_or_mint: yes_token.to_account_info(),
                },
                signer_seeds,
            ),
            AuthorityType::MintTokens,
            None,
        )?;

        market.is_completed = false;

        msg!("LaunchEvent before");

        emit!(LaunchEvent {
            creator: market.creator,
            market: self.market.key(),

            yes_mint: self.yes_token.key(),
            yes_metadata: self.yes_token_metadata_account.key(),
            yes_real_reserve_lamport: self.market.real_yes_sol_reserves,
            yes_real_reserve_token: self.market.real_yes_token_reserves,
            yes_virtual_reserve_lamport: self.market.virtual_yes_sol_reserves,
            yes_virtual_reserve_token: self.market.virtual_yes_token_reserves,

            no_mint: self.no_token.key(),
            no_metadata: self.no_token_metadata_account.key(),
            no_real_reserve_lamport: self.market.real_no_sol_reserves,
            no_real_reserve_token: self.market.real_no_token_reserves,
            no_virtual_reserve_lamport: self.market.virtual_no_sol_reserves,
            no_virtual_reserve_token: self.market.virtual_no_token_reserves,

            market_info: self.market.market_info.clone(),
            token_supply: global_config.token_supply_config,
            decimals: global_config.token_decimals_config,
            market_type: 1,
        });

        //  initialize creator vault if needed
        if self.creator_vault.lamports() == 0 {
            sol_transfer_from_user(
                &self.first_client,
                self.creator_vault.clone(),
                &self.system_program,
                1000000,
            )?;
        }

        Ok(())
    }
}
