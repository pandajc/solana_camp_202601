use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
mod instructions;
// pub mod instructions;

declare_id!("3E7zTnARoJix88NV5roQenzqXEfv5MUMp4GoMmXRURcA");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct Initialize {}
