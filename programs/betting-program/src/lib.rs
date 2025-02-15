use anchor_lang::prelude::*;

declare_id!("Bpeo86v5xF7jBoSD5MheMDeppmfEQKRSRgYoCXmoWvRp");

#[program]
pub mod betting_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
