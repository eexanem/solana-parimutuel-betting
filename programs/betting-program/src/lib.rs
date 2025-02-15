use anchor_lang::prelude::*;

declare_id!("3VCmwMXEsnak5A58TYDfrtNWd8crioEhkytCmAr91ZsG");

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
