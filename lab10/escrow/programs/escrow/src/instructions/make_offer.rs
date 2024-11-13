use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken,  token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};

use crate::Offer;

pub fn process(ctx:Context<MakeOffer>, id:u64, token_mint_amount_a: u64, token_mint_amount_b: u64)-> Result<()>{

    send_user_tokens_to_vault( &ctx, token_mint_amount_a)?;
    store_offer_params(ctx, id, token_mint_amount_b);
    
    Ok(())
}

pub fn store_offer_params(ctx: Context<MakeOffer>, id:u64, token_mint_amount_b: u64){
    ctx.accounts.offer.set_inner(Offer {
        id, 
        maker: *ctx.accounts.maker.key,
        token_mint_a: ctx.accounts.token_mint_a.key(),
        token_mint_b: ctx.accounts.token_mint_b.key(),
        token_b_wanted_amount: token_mint_amount_b,
        bump: ctx.accounts.offer.bump
    });
}

pub fn send_user_tokens_to_vault(ctx: &Context<MakeOffer>, amount_to_transfer:u64) ->Result<()>{

    let transfer_accounts = TransferChecked{
        from: ctx.accounts.maker_token_account_a.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        to:ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),

    };

    let cpi_context = CpiContext::new(  ctx.accounts.token_program.to_account_info(), transfer_accounts);

    transfer_checked(cpi_context, amount_to_transfer, ctx.accounts.token_mint_a.decimals)?;

    Ok(())
} 

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init, 
        payer = maker, 
        space = Offer::INIT_SPACE + 8,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()], 
        bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    // pub btc: InterfaceAccount<'info, Mint>,
    // pub token_acc_btc: InterfaceAccount<'info, TokenAccount>,

    // pub eth: InterfaceAccount<'info, Mint>,
    // pub token_acc_eth: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init, 
        payer = maker, 
        associated_token::mint = token_mint_a, 
        associated_token::authority = maker, 
        associated_token::token_program = token_program)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}
