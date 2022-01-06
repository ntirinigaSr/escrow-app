use {
    anchor_lang::prelude::*,
    anchor_spl::token::{self,Mint,CloseAccount,SetAuthority,TokenAccount,Transfer},
    spl_token::instruction::AuthorityType
};

declare_id!("DcDKhEcU5gn8vFsL1ojkxKwZHzEdPueZEVyVaZVzUJvD");

#[program]
pub mod escrow_app {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    pub fn initialize(
        ctx: Context<Initialize>,
        _vault_account_bump: u8,
        initializer_amount: u64,
        taker_amount: u64
    ) -> ProgramResult {

        ctx.accounts
            .escrow_account
            .initializer_key = *ctx
            .accounts
            .initializer
            .key;

        ctx.accounts
            .escrow_account
            .initializer_deposit_token_account = *ctx
            .accounts
            .initializer_deposit_token_account
            .to_account_info()
            .key;
        
        ctx.accounts.escrow_account.initializer_amount = initializer_amount;
        ctx.accounts.escrow_account.taker_amount = taker_amount;

        let (vault_authority, _vault_account_bump) = 
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        
        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority)
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            ctx.accounts.escrow_account.initializer_amount
        )?;
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



impl<'info> Initialize<'info>{

    fn into_transfer_to_pda_context(&self)-> CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let cpi_accounts = Transfer{
            from: self.initializer_deposit_token_account.to_account_info().clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.initializer.clone()
        };

        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
    
    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>>{
        let cpi_accounts = SetAuthority{
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.initializer.clone()
        };

        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

}




















