// use this import to gain access to common anchor features
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

// declare an id for your program
declare_id!("HzsrsmTepXZRCSc4jcUAdbPzLEmPAG9shr8DSzpGPbgp");

// write your business logic here
#[program]
mod hello_anchor {
    use super::*;
    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        ctx.accounts.my_account.data = data;
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct MyAccount {
    data: u64,
    mint: Pubkey
}

// validate incoming accounts here
#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
    #[account(
      constraint = my_account.mint == token_account.mint,
      has_one = owner
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>
}