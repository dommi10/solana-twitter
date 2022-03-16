use anchor_lang::prelude::*;

declare_id!("G2QLaNJt9aycxHURsJhqo89MjPp7peLKQyFjvvABwZQA");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
