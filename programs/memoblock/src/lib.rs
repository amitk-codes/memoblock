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

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateMemory<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        realloc = 8 + Memory::INIT_SPACE, 
        // realloc is actually used to return the extra lamport or take the extra lamport by comparing current account space and previous account space
        realloc::payer = payer,
        realloc::zero = true,
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
