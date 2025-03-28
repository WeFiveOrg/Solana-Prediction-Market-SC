use crate::errors::*;
use crate::events::*;
use crate::state::config::*;
use crate::utils::*;
use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use anchor_spl::token::Mint;
use anchor_spl::token::Token;

#[account]
pub struct Market {
    pub yes_token_mint: Pubkey,
    pub no_token_mint: Pubkey,

    pub creator: Pubkey,

    pub real_yes_token_reserves: u64,
    pub real_yes_sol_reserves: u64,

    pub virtual_yes_sol_reserves: u64,
    pub virtual_yes_token_reserves: u64,

    pub virtual_no_sol_reserves: u64,
    pub virtual_no_token_reserves: u64,

    pub real_no_token_reserves: u64,
    pub real_no_sol_reserves: u64,

    pub is_completed: bool,
    pub market_info: String,
}

#[derive(Debug, Clone)]
pub struct SellResult {
    pub token_amount: u64,
    pub sol_amount: u64,
}

#[derive(Debug, Clone)]
pub struct BuyResult {
    pub token_amount: u64,
    pub sol_amount: u64,
}

pub trait MarketAccount<'info> {
    fn swap(
        &mut self,
        global_config: &Account<'info, Config>,

        yes_token: &Account<'info, Mint>,
        global_yes_ata: &mut AccountInfo<'info>,
        user_yes_ata: &mut AccountInfo<'info>,

        no_token: &Account<'info, Mint>,
        global_no_ata: &mut AccountInfo<'info>,
        user_no_ata: &mut AccountInfo<'info>,

        source: &mut AccountInfo<'info>,
        team_wallet: &mut AccountInfo<'info>,
        team_wallet2: &mut AccountInfo<'info>,
        creator_vault: &mut AccountInfo<'info>,

        amount: u64,
        direction: u8,
        token_type: u8,
        minimum_receive_amount: u64,

        user: &Signer<'info>,
        signer: &[&[&[u8]]],

        is_small_fee: bool,

        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn apply_sell(&mut self, token_amount: u64, token_type: u8) -> Option<SellResult>;

    fn apply_buy(&mut self, sol_amount: u64, token_type: u8) -> Option<BuyResult>;

    fn get_sol_for_sell_tokens(&self, token_amount: u64, token_type: u8) -> Option<u64>;

    fn get_tokens_for_buy_sol(&self, sol_amount: u64, token_type: u8) -> Option<u64>;

    fn check_update_real_sol_reserves(
        &mut self,
        token_type: u8,
        global_config: &Account<'info, Config>,
    ) -> Option<u64>;

    fn calc_expected_real_sol_reserves(
        &mut self,
        token_type: u8,
        global_config: &Account<'info, Config>,
    ) -> Option<u64>;
}

impl<'info> MarketAccount<'info> for Account<'info, Market> {
    fn swap(
        &mut self,
        global_config: &Account<'info, Config>,

        yes_token: &Account<'info, Mint>,
        global_yes_ata: &mut AccountInfo<'info>,
        user_yes_ata: &mut AccountInfo<'info>,

        no_token: &Account<'info, Mint>,
        global_no_ata: &mut AccountInfo<'info>,
        user_no_ata: &mut AccountInfo<'info>,

        source: &mut AccountInfo<'info>,
        team_wallet: &mut AccountInfo<'info>,
        team_wallet2: &mut AccountInfo<'info>,
        creator_vault: &mut AccountInfo<'info>,

        amount: u64,
        direction: u8,
        token_type: u8,
        minimum_receive_amount: u64,

        user: &Signer<'info>,
        signer: &[&[&[u8]]],

        is_small_fee: bool,

        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        if amount <= 0 {
            return err!(TakesFunError::InvalidAmount);
        }

        let sol_amount: u64;
        let token_amount: u64;
        let platform_fee_lamports: u64;
        let creator_fee_lamports: u64;
        let mut actual_shift_lamports_real: u64;
        let mut actual_shift_lamports_virtual: u64;
        actual_shift_lamports_virtual = 0;
        actual_shift_lamports_real = 0;

        //check the real_sol_reserves is enough
        let real_sol_reserves_enough = self
            .check_update_real_sol_reserves(token_type, global_config)
            .ok_or(TakesFunError::InsufficientRealSolReserves)?;

        msg!("real_sol_reserves_enough: {}", real_sol_reserves_enough);

        if direction == 1 {
            //Sell tokens

            let sell_result = self
                .apply_sell(amount, token_type)
                .ok_or(TakesFunError::SellFailed)?;
            msg!("SellResult: {:#?}", sell_result);

            sol_amount = sell_result.sol_amount;
            token_amount = sell_result.token_amount;

            if is_small_fee == false {
                platform_fee_lamports =
                    bps_mul(global_config.platform_sell_fee, sol_amount, 10_000).unwrap();
                msg!("Platform Fee: {} SOL", platform_fee_lamports);
            } else {
                platform_fee_lamports =
                    bps_mul(global_config.platform_sell_small_fee, sol_amount, 10_000).unwrap();
                msg!("Platform small Fee: {} SOL", platform_fee_lamports);
            }

            creator_fee_lamports =
                bps_mul(global_config.creator_sell_fee, sol_amount, 10_000).unwrap();
            msg!("Creator Fee: {} SOL", creator_fee_lamports);

            //complete sell
            let sell_amount_minus_fee =
                sell_result.sol_amount - platform_fee_lamports - creator_fee_lamports;

            require!(
                sell_amount_minus_fee >= minimum_receive_amount,
                TakesFunError::SlippageExceeded,
            );

            if token_type == 0 {
                // Transfer no tokens to market
                token_transfer_user(
                    user_no_ata.clone(),
                    &user,
                    global_no_ata.clone(),
                    &token_program,
                    sell_result.token_amount,
                )?;
                msg!("no Token to market transfer complete");
            } else {
                // Transfer yes tokens to market
                token_transfer_user(
                    user_yes_ata.clone(),
                    &user,
                    global_yes_ata.clone(),
                    &token_program,
                    sell_result.token_amount,
                )?;
                msg!("yes Token to market transfer complete");
            }

            // Transfer SOL to user
            sol_transfer_with_signer(
                source.clone(),
                user.to_account_info(),
                &system_program,
                signer,
                sell_amount_minus_fee,
            )?;
            msg!("SOL to user transfer complete");

            if platform_fee_lamports > 0 {
                //Transfer SOL to team_wallet
                sol_transfer_with_signer(
                    source.clone(),
                    team_wallet.clone(),
                    &system_program,
                    signer,
                    platform_fee_lamports,
                )?;

                msg!("Fee to team_wallet transfer complete");
            }

            if creator_fee_lamports > 0 {
                //Transfer SOL to creator_wallet
                sol_transfer_with_signer(
                    source.clone(),
                    creator_vault.clone(),
                    &system_program,
                    signer,
                    creator_fee_lamports,
                )?;

                msg!("Fee to creator_wallet transfer complete");
            }
        } else {
            let buy_amount_applied: u64;

            if is_small_fee == false {
                platform_fee_lamports =
                    bps_mul(global_config.platform_buy_fee, amount, 10_000).unwrap();
                msg!("Platform Fee: {} SOL", platform_fee_lamports);
            } else {
                platform_fee_lamports =
                    bps_mul(global_config.platform_buy_small_fee, amount, 10_000).unwrap();
                msg!("Platform small Fee: {} SOL", platform_fee_lamports);
            }

            creator_fee_lamports = bps_mul(global_config.creator_buy_fee, amount, 10_000).unwrap();
            msg!("Creator Fee: {} SOL", creator_fee_lamports);

            buy_amount_applied = amount - platform_fee_lamports - creator_fee_lamports;

            // Buy tokens
            let buy_result = self
                .apply_buy(buy_amount_applied, token_type)
                .ok_or(TakesFunError::BuyFailed)?;

            msg!("BuyResult: {:#?}", buy_result);

            sol_amount = buy_result.sol_amount;
            token_amount = buy_result.token_amount;

            //complete buy
            require!(
                buy_result.token_amount >= minimum_receive_amount,
                TakesFunError::SlippageExceeded,
            );

            msg!("swap: token_type:{:?}", token_type);
            // Transfer tokens to user
            if token_type == 0 {
                token_transfer_with_signer(
                    global_no_ata.clone(),
                    source.clone(),
                    user_no_ata.clone(),
                    &token_program,
                    signer,
                    buy_result.token_amount,
                )?;
            } else {
                token_transfer_with_signer(
                    global_yes_ata.clone(),
                    source.clone(),
                    user_yes_ata.clone(),
                    &token_program,
                    signer,
                    buy_result.token_amount,
                )?;
            }
            msg!("Token transfer complete");

            //Transfer sol to market
            sol_transfer_from_user(
                &user,
                source.clone(),
                &system_program,
                buy_result.sol_amount,
            )?;
            msg!("SOL to bonding curve transfer complete");

            if platform_fee_lamports > 0 {
                //Transfer SOL to team_wallet
                sol_transfer_from_user(
                    &user,
                    team_wallet.clone(),
                    &system_program,
                    platform_fee_lamports,
                )?;
                // sol_transfer_with_signer(
                //     source.clone(),
                //     team_wallet.clone(),
                //     &system_program,
                //     signer,
                //     platform_fee_lamports,
                // )?;

                msg!("Fee to team_wallet transfer complete");
            }

            if creator_fee_lamports > 0 {
                //Transfer SOL to creator_wallet
                sol_transfer_from_user(
                    &user,
                    creator_vault.clone(),
                    &system_program,
                    creator_fee_lamports,
                )?;

                // sol_transfer_with_signer(
                //     source.clone(),
                //     creator.clone(),
                //     &system_program,
                //     signer,
                //     creator_fee_lamports,
                // )?;

                msg!("Fee to creator_wallet transfer complete");
            }

            // Cross effect on buy only
            let shift_sol = convert_from_float(
                convert_to_float(buy_result.sol_amount, 9) * global_config.cross_sol_factor,
                9,
            );

            msg!("self.real_yes_sol_reserves: {:?} self.real_no_sol_reserves: {:?} global_config.min_sol_liquidity: {:?}", self.real_yes_sol_reserves, self.real_no_sol_reserves, global_config.min_sol_liquidity);

            // Determine the maximum removable SOL
            let max_can_remove_virtual = match token_type {
                0 => self
                    .virtual_yes_sol_reserves
                    .saturating_sub(global_config.min_sol_liquidity),
                _ => self
                    .virtual_no_sol_reserves
                    .saturating_sub(global_config.min_sol_liquidity),
            };
            msg!(
                "shift_sol: {:?}, max_can_remove: {:?}",
                shift_sol,
                max_can_remove_virtual
            );

            // Set actual shift amount
            actual_shift_lamports_virtual = shift_sol.min(max_can_remove_virtual);
            msg!("actual_shift: {:?}", actual_shift_lamports_virtual);

            match token_type {
                1 => {
                    self.virtual_no_sol_reserves = self
                        .virtual_no_sol_reserves
                        .saturating_sub(actual_shift_lamports_virtual);
                }
                _ => {
                    self.virtual_yes_sol_reserves = self
                        .virtual_yes_sol_reserves
                        .saturating_sub(actual_shift_lamports_virtual);
                }
            }

            //Determine the expected Real Sol Reserves
            let expected_real_sol_reserves = self
                .calc_expected_real_sol_reserves(token_type, global_config)
                .ok_or(TakesFunError::InvalidExpectedRealSolReserves)?;

            let expected_shift_lamports_real = match token_type {
                0 => self
                    .real_yes_sol_reserves
                    .saturating_sub(expected_real_sol_reserves),
                _ => self
                    .real_no_sol_reserves
                    .saturating_sub(expected_real_sol_reserves),
            };

            // Determine the maximum removable SOL
            let max_can_remove_real = match token_type {
                0 => self
                    .real_yes_sol_reserves
                    .saturating_sub(global_config.min_sol_liquidity),
                _ => self
                    .real_no_sol_reserves
                    .saturating_sub(global_config.min_sol_liquidity),
            };
            msg!(
                "shift_sol: {:?}, max_can_remove: {:?}",
                shift_sol,
                max_can_remove_real
            );

            // Set actual shift amount
            actual_shift_lamports_real = expected_shift_lamports_real.min(max_can_remove_real);
            msg!("actual_shift: {:?}", actual_shift_lamports_real);

            match token_type {
                1 => {
                    self.real_no_sol_reserves = self
                        .real_no_sol_reserves
                        .saturating_sub(actual_shift_lamports_real);
                }
                _ => {
                    self.real_yes_sol_reserves = self
                        .real_yes_sol_reserves
                        .saturating_sub(actual_shift_lamports_real);
                }
            }

            if actual_shift_lamports_real > 0 {
                //Transfer SOL to team_wallet
                sol_transfer_with_signer(
                    source.clone(),
                    team_wallet2.clone(),
                    &system_program,
                    signer,
                    actual_shift_lamports_real,
                )?;

                msg!("Fee to team_wallet transfer complete");
            }
        }

        emit!(TradeEvent {
            user: user.key(),
            no_token: no_token.key(),
            yes_token: yes_token.key(),
            market: self.key(),

            sol_amount: sol_amount,
            token_amount: token_amount,
            platform_fee_lamports: platform_fee_lamports,
            creator_fee_lamports: creator_fee_lamports,
            shift_lamports_real: actual_shift_lamports_real,
            shift_lamports_virtual: actual_shift_lamports_virtual,

            direction: direction,
            token_type: token_type,
            timestamp: Clock::get()?.unix_timestamp,

            yes_virtual_reserve_lamport: self.virtual_yes_sol_reserves,
            yes_virtual_reserve_token: self.virtual_yes_token_reserves,
            yes_real_reserve_lamport: self.real_yes_sol_reserves,
            yes_real_reserve_token: self.real_yes_token_reserves,

            no_virtual_reserve_lamport: self.virtual_no_sol_reserves,
            no_virtual_reserve_token: self.virtual_no_token_reserves,
            no_real_reserve_lamport: self.real_no_sol_reserves,
            no_real_reserve_token: self.real_no_token_reserves,
        });

        Ok(())
    }

    fn get_sol_for_sell_tokens(&self, token_amount: u64, token_type: u8) -> Option<u64> {
        if token_amount == 0 {
            return None;
        }

        msg!(
            "GetSolForSellTokens: token_amount: {} token_type: {}",
            token_amount,
            token_type
        );

        // Select the correct reserves based on token_type
        let (current_sol, current_tokens) = if token_type == 0 {
            (
                self.virtual_no_sol_reserves as u128,
                self.virtual_no_token_reserves as u128,
            )
        } else {
            (
                self.virtual_yes_sol_reserves as u128,
                self.virtual_yes_token_reserves as u128,
            )
        };

        // Convert to a common decimal basis (using 9 decimals as base)
        let current_tokens = current_tokens
            .checked_mul(1_000_000_000)? // Scale tokens up to 9 decimals
            .checked_div(1_000_000)?; // Adjust from 6 decimals

        // Calculate new reserves using constant product formula
        let new_tokens = current_tokens.checked_add(
            (token_amount as u128)
                .checked_mul(1_000_000_000)? // Scale input tokens to 9 decimals
                .checked_div(1_000_000)?, // From 6 decimals
        )?;

        let new_sol = (current_sol.checked_mul(current_tokens)?).checked_div(new_tokens)?;

        let sol_out = current_sol.checked_sub(new_sol)?;

        <u128 as TryInto<u64>>::try_into(sol_out).ok()
    }

    fn get_tokens_for_buy_sol(&self, sol_amount: u64, token_type: u8) -> Option<u64> {
        if sol_amount == 0 {
            return None;
        }
        msg!(
            "GetTokensForBuySol: sol_amount: {} token_type: {}",
            sol_amount,
            token_type
        );

        // Select the correct reserves based on token_type
        let (current_sol, current_tokens) = if token_type == 0 {
            (
                self.virtual_no_sol_reserves as u128,
                self.virtual_no_token_reserves as u128,
            )
        } else {
            (
                self.virtual_yes_sol_reserves as u128,
                self.virtual_yes_token_reserves as u128,
            )
        };

        // Convert to a common decimal basis (using 9 decimals as base)
        let current_tokens = current_tokens
            .checked_mul(1_000_000_000)? // Scale tokens up to 9 decimals
            .checked_div(1_000_000)?; // Adjust from 6 decimals

        // Calculate new reserves using constant product formula
        let new_sol = current_sol.checked_add(sol_amount as u128)?;
        let new_tokens = (current_sol.checked_mul(current_tokens)?).checked_div(new_sol)?;

        let tokens_out = current_tokens.checked_sub(new_tokens)?;

        // Convert back to 6 decimal places for tokens
        let tokens_out = tokens_out
            .checked_mul(1_000_000)? // Convert to 6 decimals
            .checked_div(1_000_000_000)?; // From 9 decimals

        <u128 as TryInto<u64>>::try_into(tokens_out).ok()
    }

    fn apply_buy(&mut self, sol_amount: u64, token_type: u8) -> Option<BuyResult> {
        // Computing Token Amount out
        let token_amount = self.get_tokens_for_buy_sol(sol_amount, token_type)?;
        msg!(
            "ApplyBuy: sol_amount: {}, token_amount: {}",
            sol_amount,
            token_amount
        );

        // Ensure token_amount is within valid limits
        let real_reserves = if token_type == 0 {
            self.real_no_token_reserves
        } else {
            self.real_yes_token_reserves
        };

        if token_amount >= real_reserves {
            //some error
            msg!(
                "Error: BuyTokenAmountInvalid - token_amount {} exceeds real_reserves {}",
                token_amount,
                real_reserves
            );
            return None;
        }

        msg!(
            "ApplyBuy: virtual_no_token_reserves: {}, virtual_yes_token_reserves: {}",
            self.virtual_no_token_reserves,
            self.virtual_yes_token_reserves
        );

        // Adjusting token reserve values
        // New Virtual Token Reserves
        let new_virtual_token_reserves = if token_type == 0 {
            self.virtual_no_token_reserves as u128
        } else {
            self.virtual_yes_token_reserves as u128
        }
        .checked_sub(token_amount as u128)?;

        msg!(
            "ApplyBuy: new_virtual_token_reserves: {}",
            new_virtual_token_reserves
        );

        msg!(
            "ApplyBuy: real_no_token_reserves: {}, real_yes_token_reserves: {}",
            self.real_no_token_reserves,
            self.real_yes_token_reserves
        );

        // New Real Token Reserves
        let new_real_token_reserves = if token_type == 0 {
            self.real_no_token_reserves as u128
        } else {
            self.real_yes_token_reserves as u128
        }
        .checked_sub(token_amount as u128)?;

        msg!(
            "ApplyBuy: new_virtual_token_reserves: {}",
            new_real_token_reserves
        );

        msg!(
            "ApplyBuy: virtual_no_sol_reserves: {}, virtual_yes_sol_reserves: {}",
            self.virtual_no_sol_reserves,
            self.virtual_yes_sol_reserves
        );
        // Adjusting sol reserve values
        // New Virtual Sol Reserves
        let new_virtual_sol_reserves = if token_type == 0 {
            self.virtual_no_sol_reserves as u128
        } else {
            self.virtual_yes_sol_reserves as u128
        }
        .checked_add(sol_amount as u128)?;

        msg!(
            "ApplyBuy: new_virtual_sol_reserves: {}",
            new_virtual_sol_reserves
        );

        msg!(
            "ApplyBuy: real_no_sol_reserves: {}, real_yes_sol_reserves: {}",
            self.real_no_sol_reserves,
            self.real_yes_sol_reserves
        );
        // New Real Sol Reserves
        let new_real_sol_reserves = if token_type == 0 {
            self.real_no_sol_reserves as u128
        } else {
            self.real_yes_sol_reserves as u128
        }
        .checked_add(sol_amount as u128)?;

        msg!("ApplyBuy: new_real_sol_reserves: {}", new_real_sol_reserves);

        if token_type == 0 {
            self.virtual_no_token_reserves = new_virtual_token_reserves.try_into().ok()?;
            self.real_no_token_reserves = new_real_token_reserves.try_into().ok()?;
            self.virtual_no_sol_reserves = new_virtual_sol_reserves.try_into().ok()?;
            self.real_no_sol_reserves = new_real_sol_reserves.try_into().ok()?;
        } else {
            self.virtual_yes_token_reserves = new_virtual_token_reserves.try_into().ok()?;
            self.real_yes_token_reserves = new_real_token_reserves.try_into().ok()?;
            self.virtual_yes_sol_reserves = new_virtual_sol_reserves.try_into().ok()?;
            self.real_yes_sol_reserves = new_real_sol_reserves.try_into().ok()?;
        }

        Some(BuyResult {
            token_amount,
            sol_amount,
        })
    }

    fn apply_sell(&mut self, token_amount: u64, token_type: u8) -> Option<SellResult> {
        msg!(
            "apply_sell: token_amount: {} token_type: {}",
            token_amount,
            token_type
        );

        // Computing Sol Amount out
        let sol_amount = self.get_sol_for_sell_tokens(token_amount, token_type)?;
        msg!("apply_sell: sol_amount: {}", sol_amount);

        msg!(
            "ApplyBuy: virtual_no_token_reserves: {}, virtual_yes_token_reserves: {}",
            self.virtual_no_token_reserves,
            self.virtual_yes_token_reserves
        );

        // Adjusting token reserve values
        // New Virtual Token Reserves
        let new_virtual_token_reserves = if token_type == 0 {
            self.virtual_no_token_reserves as u128
        } else {
            self.virtual_yes_token_reserves as u128
        }
        .checked_add(token_amount as u128)?;
        msg!(
            "apply_sell: new_virtual_token_reserves: {}",
            new_virtual_token_reserves
        );

        msg!(
            "ApplyBuy: real_no_token_reserves: {}, real_yes_token_reserves: {}",
            self.real_no_token_reserves,
            self.real_yes_token_reserves
        );
        // New Real Token Reserves
        let new_real_token_reserves = if token_type == 0 {
            self.real_no_token_reserves as u128
        } else {
            self.real_yes_token_reserves as u128
        }
        .checked_add(token_amount as u128)?;
        msg!(
            "apply_sell: new_real_token_reserves: {}",
            new_real_token_reserves
        );

        msg!(
            "ApplyBuy: virtual_no_sol_reserves: {}, virtual_yes_sol_reserves: {}",
            self.virtual_no_sol_reserves,
            self.virtual_yes_sol_reserves
        );
        // Adjusting sol reserve values
        // New Virtual Sol Reserves
        let new_virtual_sol_reserves = if token_type == 0 {
            self.virtual_no_sol_reserves as u128
        } else {
            self.virtual_yes_sol_reserves as u128
        }
        .checked_sub(sol_amount as u128)?;
        msg!(
            "apply_sell: new_virtual_sol_reserves: {}",
            new_virtual_sol_reserves
        );

        msg!(
            "real_no_sol_reserves: {}, real_yes_sol_reserves: {}",
            self.real_no_sol_reserves,
            self.real_yes_sol_reserves
        );
        // New Real Sol Reserves
        let new_real_sol_reserves = if token_type == 0 {
            self.real_no_sol_reserves as u128
        } else {
            self.real_yes_sol_reserves as u128
        }
        .checked_sub(sol_amount as u128)?;
        msg!(
            "apply_sell: new_real_sol_reserves: {}",
            new_real_sol_reserves
        );

        if token_type == 0 {
            self.virtual_no_token_reserves = new_virtual_token_reserves.try_into().ok()?;
            self.real_no_token_reserves = new_real_token_reserves.try_into().ok()?;
            self.virtual_no_sol_reserves = new_virtual_sol_reserves.try_into().ok()?;
            self.real_no_sol_reserves = new_real_sol_reserves.try_into().ok()?;
        } else {
            self.virtual_yes_token_reserves = new_virtual_token_reserves.try_into().ok()?;
            self.real_yes_token_reserves = new_real_token_reserves.try_into().ok()?;
            self.virtual_yes_sol_reserves = new_virtual_sol_reserves.try_into().ok()?;
            self.real_yes_sol_reserves = new_real_sol_reserves.try_into().ok()?;
        }

        Some(SellResult {
            token_amount,
            sol_amount,
        })
    }

    fn check_update_real_sol_reserves(
        &mut self,
        token_type: u8,
        global_config: &Account<'info, Config>,
    ) -> Option<u64> {
        // Select the correct reserves based on token_type
        let (current_sol, current_tokens) = if token_type == 0 {
            (
                self.virtual_no_sol_reserves as u128,
                self.virtual_no_token_reserves as u128,
            )
        } else {
            (
                self.virtual_yes_sol_reserves as u128,
                self.virtual_yes_token_reserves as u128,
            )
        };

        let current_sol_reserves = if token_type == 0 {
            self.real_no_sol_reserves as u128
        } else {
            self.real_yes_sol_reserves as u128
        };

        let initial_virtual_token_reserves = if token_type == 0 {
            global_config.initial_virtual_no_token_reserves_config as u128
        } else {
            global_config.initial_virtual_yes_token_reserves_config as u128
        };

        let basic_virtual_sol_reserves = (current_sol.checked_mul(current_tokens)?)
            .checked_div(initial_virtual_token_reserves)?;
        let expected_sol_reserves = current_sol.checked_sub(basic_virtual_sol_reserves)?;

        msg!(
            "check_update_real_sol_reserves :: basic_virtual_sol_reserves: {}, expected_sol_reserves: {} current_sol_reserves: {}",
            basic_virtual_sol_reserves,
            expected_sol_reserves,
            current_sol_reserves
        );

        if expected_sol_reserves > current_sol_reserves {
            msg!("expected_sol_reserves > current_sol_reserves");
            if token_type == 0 {
                self.real_no_sol_reserves = expected_sol_reserves.try_into().ok()?;
            } else {
                self.real_yes_sol_reserves = expected_sol_reserves.try_into().ok()?;
            }
        }

        <u128 as TryInto<u64>>::try_into(expected_sol_reserves).ok()
    }

    fn calc_expected_real_sol_reserves(
        &mut self,
        token_type: u8,
        global_config: &Account<'info, Config>,
    ) -> Option<u64> {
        // Select the correct reserves based on token_type
        let (current_sol, current_tokens) = if token_type == 1 {
            (
                self.virtual_no_sol_reserves as u128,
                self.virtual_no_token_reserves as u128,
            )
        } else {
            (
                self.virtual_yes_sol_reserves as u128,
                self.virtual_yes_token_reserves as u128,
            )
        };

        let initial_virtual_token_reserves = if token_type == 1 {
            global_config.initial_virtual_no_token_reserves_config as u128
        } else {
            global_config.initial_virtual_yes_token_reserves_config as u128
        };

        let basic_virtual_sol_reserves = (current_sol.checked_mul(current_tokens)?)
            .checked_div(initial_virtual_token_reserves)?;
        let expected_sol_reserves = current_sol.checked_sub(basic_virtual_sol_reserves)?;

        msg!(
            "calc_expected_real_sol_reserves:: basic_virtual_sol_reserves: {}, expected_sol_reserves: {} ",
            basic_virtual_sol_reserves,
            expected_sol_reserves,
        );

        <u128 as TryInto<u64>>::try_into(expected_sol_reserves).ok()
    }
}
