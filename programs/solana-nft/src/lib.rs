use anchor_lang::prelude::*;

declare_id!("GPLXPsrEmVBcfv2xf79GUQzdADHFxs3rF1Z3ceTLo8mv");

#[program]
pub mod solana_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
