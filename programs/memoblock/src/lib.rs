use anchor_lang::prelude::*;

declare_id!("62yyqxq4mfFatbWwMjXh7hn4149RFCZm7RxMNcWChZjN");

#[program]
pub mod memoblock {
    use super::*;

    pub fn create_memory(
        ctx: Context<CreateMemory>,
        title: String,
        description: String,
    ) -> Result<()> {
        let memory_account = &mut ctx.accounts.memory_account;
        memory_account.owner = ctx.accounts.payer.key();
        memory_account.title = title;
        memory_account.description = description;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateMemory<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = Memory::INIT_SPACE,
        seeds = [title.as_bytes(), payer.key().as_ref()],
        bump,
    )]
    pub memory_account: Account<'info, Memory>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Memory {
    pub owner: Pubkey,

    #[max_len(100)]
    pub title: String,

    #[max_len(1000)]
    pub description: String,
}
