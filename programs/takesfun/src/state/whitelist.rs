use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Whitelist {
    pub creator: Pubkey,
    pub first_swap_timestamp: i64,
    pub is_allow: u8,
}

impl Whitelist {
    pub const SEED_PREFIX: &'static str = "wl-seed";

    pub fn is_whitelister(&mut self, limit_timestamp: i64, time_stamp: i64) -> Result<bool> {
        let last_time_stamp = self.first_swap_timestamp + limit_timestamp;

        if last_time_stamp >= time_stamp {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
