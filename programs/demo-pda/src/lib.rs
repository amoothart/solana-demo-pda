use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("9yQ5rLuamV6voCNZcx7NGUBAT3EXxJvpeGVXZ1MbM9vQ");

#[program]
pub mod demo_pda {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        //Get escrow account
        let escrow = &mut ctx.accounts.escrow;

        escrow.from = ctx.accounts.from.key();
        escrow.to = ctx.accounts.to.key();
        escrow.amount = amount;

        Ok(())
    }
} 

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(
        init,
        // Can only have 1 active transaction
        seeds = [b"escrow".as_ref(), from.key().as_ref(), to.key().as_ref()],
        bump,
        payer = from,
        space = size_of::<EscrowAccount>() + 16
    )]
    pub escrow: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub from: Signer<'info>,
    /// CHECK: safe
    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowAccount {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}