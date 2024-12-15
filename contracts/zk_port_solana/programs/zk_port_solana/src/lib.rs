use anchor_lang::prelude::*;

declare_id!("Cd1LYGrLomHD7gFdrzU2m4PTCTycfCagJmUtDYVooHWo");

#[program]
pub mod zk_port_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
