use anchor_lang::prelude::*;

declare_id!("BxzVRSeYhiivdjUXU9Yx5EvJ6ndNZ6pKBBKz2GQH2VQ3");

#[program]
pub mod journal {
    use super::*;

    pub fn create_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = ctx.accounts.owner.key();
        journal_entry.title = title;
        journal_entry.message = message;

        Ok(())
    }

    pub fn update_entry(
        ctx: Context<UpdateEntry>,
        _title: String,
        new_message: String,
    ) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.message = new_message;

        Ok(())
    }

    pub fn delete_entry(_ctx: Context<DeleteEntry>, _title: String) -> Result<()> {
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
    pub owner: Pubkey,
    #[max_len(20)]
    pub title: String,
    #[max_len(200)]
    pub message: String,
    pub entry_id: u64,
}

#[derive(Accounts)]
// adding the title instruction since we are adding additional data (title)(probably from the front end)
#[instruction(title: String)]
///////////////////////////////////
pub struct CreateEntry<'info> {
    #[account(
    init,
    // we are specifying both the title and the owner as our PDA
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    payer = owner,
    space = 8 + JournalEntryState::INIT_SPACE,
  )]
    // defining the account
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateEntry<'info> {
    #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    // since we are updating, we have to recalculate rent
    realloc = 8 + JournalEntryState::INIT_SPACE,
    realloc::payer = owner,
    realloc::zero = true,
  )]
    // defining the account
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    close = owner,
  )]
    // defining the account
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
