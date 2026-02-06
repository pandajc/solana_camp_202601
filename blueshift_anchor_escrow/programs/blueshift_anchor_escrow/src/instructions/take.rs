use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, close_account}, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};

use crate::{errors::EscrowError, state::Escrow};

// 关闭托管记录，将其租金 lamports 返还给创建者。

// 将 Token A 从保管库转移到接受者，然后关闭保管库。

// 将约定数量的 Token B 从接受者转移到创建者。


#[derive(Accounts)]
pub struct Take<'info>{
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
        mut,
        //  Closes the account by sending lamports to target and resetting data.
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker @ EscrowError::InvalidMaker, // maker与escrow中的maker保持一致
        has_one = mint_a @ EscrowError::InvalidMintA,
        has_one = mint_b @ EscrowError::InvalidMintB,
    )]
    // Box ?
    pub escrow: Box<Account<'info, Escrow>>,
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::authority = escrow,
        associated_token::mint = mint_a,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,


    #[account(
        init_if_needed,
        payer = taker,
        // ata的mint账户
        associated_token::mint = mint_a,
        // ata的账户所有者
        associated_token::authority = taker,
        // ata的owner
        associated_token::token_program = token_program
    )]
    // taker mint_a的ata，从vault接收token a
    pub taker_ata_a:Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    // taker mint_b的ata，将token b发送给maker
    pub taker_ata_b:Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    // maker mint_a的ata，接收taker_ata_b的token b
    pub maker_ata_b:Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    fn transfer_to_maker(&mut self) -> Result<()>{
        transfer_checked(CpiContext::new(
            self.token_program.to_account_info(), TransferChecked{
                from: self.taker_ata_b.to_account_info(),
                to: self.maker_ata_b.to_account_info(),
                mint: self.mint_b.to_account_info(),
                authority: self.taker.to_account_info()

            }), self.escrow.receive, self.mint_b.decimals)?;
            
        Ok(())
    }

    fn withdraw_and_close_vault(&mut self) -> Result<()>{
        // signer_seeds 是个数组，支持多个签名者，不同签名者的seeds放到一个数组里
        // [&[&[u8]]; 1] 代表数组只有1个元素，元素类型是&[&[u8]]，也就是某个seeds的引用
        // &[&[u8]] 指一个切片的引用，这个切片的元素类型是&[u8]
        // u8 代表一个字节，无符号8位整数，范围 0-255
        // b"escrow" 代表字节串，即&[u8; 5]，普通字符串的类型是&str
        let signer_seeds: [&[&[u8]]; 1] = [
            &[
                b"escrow", 
                self.maker.to_account_info().key.as_ref(),
                &self.escrow.seed.to_le_bytes()[..], //转为切片，因为元素类型是&[u8]，是切片的引用而不是数组的引用
                &[self.escrow.bump]
            ]
        ];
        transfer_checked(CpiContext::new_with_signer(
            self.token_program.to_account_info(),
         TransferChecked { 
            from: self.vault.to_account_info(), 
            mint: self.mint_a.to_account_info(), 
            to: self.taker_ata_a.to_account_info(), 
            authority: self.escrow.to_account_info() 
        }, 
        &signer_seeds
        ), 
        self.vault.amount, self.mint_a.decimals)?;
        close_account(CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            CloseAccount { 
                account: self.vault.to_account_info(), 
                destination: self.maker.to_account_info(), 
                authority: self.escrow.to_account_info() }, 
            &signer_seeds))?;
        Ok(())
    }
}

pub fn handler(ctx: Context<Take>) -> Result<()> {
    // token b转给maker
    ctx.accounts.transfer_to_maker()?;
    // 提取 token a，关闭vault
    ctx.accounts.withdraw_and_close_vault()?;
    Ok(())
}