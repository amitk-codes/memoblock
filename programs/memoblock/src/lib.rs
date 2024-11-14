use anchor_lang::prelude::*;

declare_id!("62yyqxq4mfFatbWwMjXh7hn4149RFCZm7RxMNcWChZjN");

#[program]
pub mod memoblock {
    use super::*;

    pub fn create_memory(
        ctx: Context<CreateMemory>,
        id: Pubkey,
        title: String,
        description: String,
    ) -> Result<()> {
        let memory_account = &mut ctx.accounts.memory_account;
        memory_account.id = id;
        memory_account.owner = ctx.accounts.payer.key();
        memory_account.title = title;
        memory_account.description = description;
        Ok(())
    }

    pub fn update_memory(
        ctx: Context<UpdateMemory>,
        _id: Pubkey,
        new_title: String,
        new_description: String,
    ) -> Result<()> {
        let memory_account = &mut ctx.accounts.memory_account;
        
        memory_account.title = new_title;
        memory_account.description = new_description;
        Ok(())
    }

    pub fn delete_memory(_ctx: Context<DeleteMemory>, _id: Pubkey) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: Pubkey)]
pub struct CreateMemory<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = Memory::INIT_SPACE,
        seeds = [id.as_ref(), payer.key().as_ref()], 
        bump,
    )]
    pub memory_account: Account<'info, Memory>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: Pubkey)]
pub struct UpdateMemory<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        realloc = 8 + Memory::INIT_SPACE, 
        // realloc is actually used to return the extra lamport or take the extra lamport by comparing current account space and previous account space
        realloc::payer = payer,
        realloc::zero = true,
        seeds = [id.as_ref(), payer.key().as_ref()], 
        bump,
    )]
    pub memory_account: Account<'info, Memory>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: Pubkey)]
pub struct DeleteMemory<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [id.as_ref(), owner.key().as_ref()],
        bump,
        close = owner
    )]
    pub memory_account: Account<'info, Memory>,

    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct Memory {
    pub id: Pubkey,
    pub owner: Pubkey,

    #[max_len(100)]
    pub title: String,

    #[max_len(1000)]
    pub description: String,
}
