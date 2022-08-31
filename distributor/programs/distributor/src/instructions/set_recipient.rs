use {
    crate::state::*,
    anchor_lang::{
        prelude::*,
        solana_program::{system_program, sysvar, instruction::Instruction},
    },
    anchor_spl::{
        associated_token::{self, get_associated_token_address}, token::{self, Mint}
    },
    clockwork_crank::{ 
        cpi::accounts::QueueUpdate,
        program::ClockworkCrank,
        state::{SEED_QUEUE, Queue},
    },
};

#[derive(Accounts)]
#[instruction(new_recipient: Option<Pubkey>)]
pub struct SetRecipient<'info> {
    /// CHECK: manually validated against distributor account and recipient's token account
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(address = clockwork_crank::ID)]
    pub clockwork_program: Program<'info, ClockworkCrank>,

    /// CHECK: manually validated against distributor account
    pub current_recipient: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SEED_DISTRIBUTOR, distributor.mint.as_ref(), distributor.admin.as_ref()],
        bump,
        has_one = mint,
        has_one = admin,
        constraint = distributor.recipient == current_recipient.key()
    )]
    pub distributor: Account<'info, Distributor>,

    #[account(
        mut, 
        seeds = [
            SEED_QUEUE, 
            distributor.key().as_ref(), 
            "distributor".as_bytes()
        ], 
        seeds::program = clockwork_crank::ID,
        bump
     )]
    pub distributor_queue: Account<'info, Queue>,
    
    /// CHECK: manually validated against distributor account and recipient's token account
    pub mint: Account<'info, Mint>,
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, SetRecipient<'info>>, new_recipient: Option<Pubkey>) -> Result<()> {
     // get accounts
    let admin = &ctx.accounts.admin;
    let clockwork_program = &ctx.accounts.clockwork_program;
    let distributor = &mut ctx.accounts.distributor;
    let distributor_queue = &mut ctx.accounts.distributor_queue;
    let mint = &ctx.accounts.mint;

    // update distributor with new recipient
    if let Some(new_recipient) = new_recipient {
        distributor.recipient = new_recipient;
    }

    let recipient_token_account_pubkey = get_associated_token_address(&distributor.recipient, &distributor.mint);

    // update queue with new ix data
    let mint_token_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(admin.key(), false),
            AccountMeta::new_readonly(associated_token::ID, false),
            AccountMeta::new_readonly(distributor.key(), false),
            AccountMeta::new(distributor_queue.key(), true),
            AccountMeta::new_readonly(mint.key(), false),
            AccountMeta::new(clockwork_crank::payer::ID, true),
            AccountMeta::new_readonly(distributor.recipient, false),
            AccountMeta::new(recipient_token_account_pubkey, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(token::ID, false),

        ],
        data: clockwork_crank::anchor::sighash("mint_token").to_vec()
    };

    clockwork_crank::cpi::queue_update(
    CpiContext::new(
    clockwork_program.to_account_info(),
        QueueUpdate {
                    authority: admin.to_account_info(), 
                    queue: distributor_queue.to_account_info()
                }),
    Some(mint_token_ix.into()), 
    None
    )?;


    Ok(())
}