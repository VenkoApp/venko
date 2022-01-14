//! Instruction handler for [crate::venko::redeem].

use crate::*;
use anchor_spl::token::{self, Burn, Mint, TokenAccount};
use vipers::{assert_keys_eq, invariant, unwrap_int, Validate};

/// Accounts for [venko::redeem].
#[derive(Accounts)]
pub struct Redeem<'info> {
    /// [token::Mint] of the [Stream].
    /// This account is `mut` because tokens are burned.
    #[account(mut)]
    pub stream_mint: Account<'info, Mint>,
    /// [Stream] account.
    #[account(mut)]
    pub stream: Box<Account<'info, Stream>>,

    /// The [TokenAccount] holding the [Self::user_authority]'s
    /// stream tokens.
    #[account(mut)]
    pub source_stream_tokens: Account<'info, TokenAccount>,

    /// Underlying tokens of the [Stream].
    #[account(mut)]
    pub underlying_tokens: Account<'info, TokenAccount>,
    /// Destination of the underlying tokens backing the [Stream].
    #[account(mut)]
    pub destination_tokens: Account<'info, TokenAccount>,

    /// The [crate_token::CrateToken].
    pub crate_token: Box<Account<'info, crate_token::CrateToken>>,

    /// User redeeming the tokens.
    pub user_authority: Signer<'info>,

    /// [System] program.
    pub system_program: Program<'info, System>,
    /// [crate_token] program.
    pub crate_token_program: Program<'info, crate_token::program::CrateToken>,
    /// SPL [token] program.
    pub token_program: Program<'info, token::Token>,
}

impl<'info> Redeem<'info> {
    fn amount_released(&self) -> Result<u64> {
        let amount_released = unwrap_int!(self
            .stream
            .available_for_withdrawal(Clock::get()?.unix_timestamp, self.underlying_tokens.amount));
        Ok(amount_released)
    }

    fn redeem(&self, amount: u64) -> ProgramResult {
        let amount_released = self.amount_released()?;

        // Has the given amount released?
        invariant!(amount <= amount_released, InsufficientWithdrawalBalance);

        // redeem the crate tokens
        self.burn_stream_tokens(amount)?;
        self.withdraw_crate_tokens(amount)?;

        Ok(())
    }

    fn burn_stream_tokens(&self, amount: u64) -> ProgramResult {
        token::burn(
            CpiContext::new(
                self.token_program.to_account_info(),
                Burn {
                    mint: self.stream_mint.to_account_info(),
                    to: self.source_stream_tokens.to_account_info(),
                    authority: self.user_authority.to_account_info(),
                },
            ),
            amount,
        )
    }

    fn withdraw_crate_tokens(&self, amount: u64) -> ProgramResult {
        let signer_seeds: &[&[&[u8]]] = stream_seeds!(self.stream);
        crate_token::cpi::withdraw(
            CpiContext::new_with_signer(
                self.crate_token_program.to_account_info(),
                crate_token::cpi::accounts::Withdraw {
                    crate_token: self.crate_token.to_account_info(),
                    crate_underlying: self.underlying_tokens.to_account_info(),
                    withdraw_authority: self.stream.to_account_info(),
                    withdraw_destination: self.destination_tokens.to_account_info(),
                    author_fee_destination: self.destination_tokens.to_account_info(),
                    protocol_fee_destination: self.destination_tokens.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )
    }
}

pub fn handler(ctx: Context<Redeem>, amount: u64) -> ProgramResult {
    ctx.accounts.redeem(amount)?;

    let stream = &mut ctx.accounts.stream;
    stream.redeemed_amount = unwrap_int!(stream.redeemed_amount.checked_add(amount));

    let amount_remaining = unwrap_int!(stream.initial_amount.checked_sub(stream.redeemed_amount));

    emit!(RedeemEvent {
        stream: stream.key(),
        mint: stream.underlying_mint,
        amount,
        amount_remaining,
    });

    Ok(())
}

/// Emitted on [crate::venko::redeem].
#[event]
pub struct RedeemEvent {
    /// The [Stream].
    #[index]
    pub stream: Pubkey,
    /// Mint of the underlying token.
    #[index]
    pub mint: Pubkey,

    /// Total tokens redeemed
    pub amount: u64,
    /// Total tokens remaining
    pub amount_remaining: u64,
}

impl<'info> Validate<'info> for Redeem<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys_eq!(self.stream_mint, self.stream.mint);

        assert_keys_eq!(self.source_stream_tokens.owner, self.user_authority);
        assert_keys_eq!(self.source_stream_tokens.mint, self.stream.mint);
        invariant!(
            self.source_stream_tokens.amount > 0,
            InsufficientStreamTokens
        );

        assert_keys_eq!(self.underlying_tokens, self.stream.underlying_tokens);
        assert_keys_eq!(self.destination_tokens.mint, self.stream.underlying_mint);

        assert_keys_eq!(self.crate_token, self.stream.crate_token);
        Ok(())
    }
}
