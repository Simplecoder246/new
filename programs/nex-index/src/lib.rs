use anchor_lang::prelude::*;

declare_id!("6uhx1vSCyGVvDKGnvnS7eWUPFqiQAiDHfHfWEyUDRxGA");

#[program]
pub mod nex_index {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
