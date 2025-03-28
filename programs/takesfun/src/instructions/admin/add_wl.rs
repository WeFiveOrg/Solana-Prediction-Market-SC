use crate::*;

use constants::{CONFIG, WHITELIST};

use crate::state::whitelist::*;

use crate::errors::*;

#[derive(Accounts)]
#[instruction(new_creator: Pubkey)]
pub struct AddWl<'info> {
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<Whitelist>(),
        seeds = [WHITELIST.as_bytes(), &user.key().to_bytes()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    #[account(
        mut,
        constraint = admin.key() == global_config.backend_sign_authority.key() @ TakesFunError::IncorrectAuthority
    )]
    pub admin: Signer<'info>,
    #[account(mut,
        constraint = user.key() == new_creator.key() @ TakesFunError::IncorrectAuthority
    )]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddWl<'info> {
    pub fn handler(&mut self, new_whitelister: Pubkey) -> Result<()> {
        let whitelist = &mut self.whitelist;
        if whitelist.is_allow == 0 {
            whitelist.creator = new_whitelister.key();
            whitelist.is_allow = 1;
            whitelist.first_swap_timestamp = 0;
        }
        Ok(())
    }
}
