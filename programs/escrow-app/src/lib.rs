use {
    anchor_lang::prelude::*,
    anchor_spl::token::{self,Mint,CloseAccount,SetAuthority,TokenAccount,Transfer},
    spl_token::instruction::AuthorityType
};

declare_id!("DcDKhEcU5gn8vFsL1ojkxKwZHzEdPueZEVyVaZVzUJvD");

#[program]
pub mod escrow_app {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}



#[derive(Accounts)]
#[instruction(vault_account_bump: u8, initializer_amount: u64)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    pub mint: Account<'info,Mint>,
    #[account(
        init,
        payer= initializer, 
        seeds = [b"token-seed".as_ref()],
        bump = vault_account_bump,
        token::mint = mint,
        token::authority = initializer)]
    pub vault_account:Account<'info,TokenAccount>,
    #[account(mut,constraint = initializer_deposit_token_account.amount >= initializer_amount)]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_reciever_token_account: Account<'info, TokenAccount>,
    #[account(zero)]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub rent: Sysvar<'info,Rent>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>

}

#[derive(Accounts)]
pub struct Cancel<'info>{
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    #[account(mut)]
    pub vault_account:Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub initializer_deposit_token_account: Account<'info,TokenAccount>,
    #[account(
        mut,
        constraint = escrow_account.initializer_key == *initializer.key,
        constraint = escrow_account.initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key
    )]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub token_program: AccountInfo<'info>
}


#[derive(Accounts)]
pub struct Exchange<'info>{
    #[account(signer)]
    pub taker: AccountInfo<'info>,
    #[account(mut)]
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub taker_receive_token_account: Account<'info,TokenAccount>,
    #[account(mut)]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_receive_token_account: Account<'info,TokenAccount>,
    #[account(mut)]
    pub initializer: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.taker_amount <= taker_deposit_token_account.amount,
        constraint = escrow_account.initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key,
        constraint = escrow_account.initializer_reciever_token_account == *initializer_receive_token_account.to_account_info().key,
        constraint = escrow_account.initializer_key == *initializer.key,
        close = initializer
    )]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub token_program: AccountInfo<'info>
}



#[account]
pub struct EscrowAccount{
    pub initializer_key : Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_reciever_token_account: Pubkey,
    pub initializer_amount : u64,
    pub taker_amount: u64
}