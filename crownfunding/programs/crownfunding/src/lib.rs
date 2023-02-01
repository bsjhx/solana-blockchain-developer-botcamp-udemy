use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("9Yrm1ATYUHnUNGCgEpHpwK5WFJqKd1dUTHvQeZtcgdWx");

#[program]
pub mod crownfunding {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String, description: String) -> ProgramResult {
        let campaign_account = &mut ctx.accounts.campaign;
        campaign_account.name = name;
        campaign_account.description = description;
        campaign_account.amount_donated = 0;
        campaign_account.admin = *ctx.accounts.user.key;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let campaign_account = &mut ctx.accounts.campaign;
        let user_account = &mut ctx.accounts.user;
        if campaign_account.admin != *user_account.key {
            return Err(ProgramError::IncorrectProgramId);
        }

        let rent_balance =
            Rent::get()?.minimum_balance(campaign_account.to_account_info().data_len());
        if campaign_account.to_account_info().lamports() - rent_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        **campaign_account
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;
        **user_account.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> ProgramResult {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.campaign.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.campaign.to_account_info(),
            ],
        );

        (&mut ctx.accounts.campaign).amount_donated += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=9000, seeds=[b"CAMPAIGN_DEMO".as_ref(), user.key().as_ref()], bump)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
}
#[derive(Accounts)]
pub struct Donate<'info> {
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Campaign {
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub amount_donated: u64,
}
