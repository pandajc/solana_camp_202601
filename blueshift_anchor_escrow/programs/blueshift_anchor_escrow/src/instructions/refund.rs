use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked, close_account, transfer_checked}};

use crate::{errors::EscrowError, state::Escrow};


// 关闭托管 PDA，并将其租金 lamports 返还给创建者。
// 将金库中的全部 Token A 余额转回创建者，然后关闭金库账户。

// 账户
// maker：决定交换条款的用户
// escrow：存储所有交换条款的账户
// mint_a：maker 存入的代币
// vault：与 escrow 和 mint_a 关联的代币账户，代币已存入其中
// maker_ata_a：与 maker 和 mint_a 关联的代币账户，将从 vault 接收代币
// associated_token_program：用于创建关联代币账户的关联代币程序
// token_program：用于 CPI 转账的代币程序
// system_program：用于创建 Escrow 的系统程序

#[derive(Accounts)]
pub struct Refund<'info> {

    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        
        has_one = maker @ EscrowError::InvalidMaker,
        has_one = mint_a @ EscrowError::InvalidMintA,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mint::token_program = token_program)]
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {

    
    fn refund(&mut self) -> Result<()> {
        let seeds_bytes = self.escrow.seed.to_le_bytes();
        let signer_seeds: [&[&[u8]]; 1] = [
            &[b"escrow", self.maker.to_account_info().key.as_ref(), seeds_bytes.as_ref(), &[self.escrow.bump]]
        ];
        if self.vault.amount > 0 {
            transfer_checked(
            CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            TransferChecked { 
                from: self.vault.to_account_info(), 
                mint: self.mint_a.to_account_info(), 
                to: self.maker_ata_a.to_account_info(), 
                authority: self.escrow.to_account_info() 
            },
            &signer_seeds), 
            self.vault.amount, self.mint_a.decimals)?;
        }
        close_account(CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            CloseAccount{
                account: self.vault.to_account_info(),
                destination: self.maker.to_account_info(),
                authority: self.escrow.to_account_info()
            }, &signer_seeds))?;
        Ok(())
    }
}

pub fn handler(ctx: Context<Refund>) -> Result<()> {
    ctx.accounts.refund()?;
    Ok(())
}