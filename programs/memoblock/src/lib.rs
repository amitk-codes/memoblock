use anchor_lang::prelude::*;

declare_id!("62yyqxq4mfFatbWwMjXh7hn4149RFCZm7RxMNcWChZjN");

#[program]
pub mod memoblock {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
