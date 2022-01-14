//! Instruction handler for [crate::venko::revoke].

use crate::*;
use anchor_spl::token::{self, TokenAccount};
use vipers::{assert_keys_eq, invariant, Validate};

/// Accounts for [venko::revoke].
#[derive(Accounts)]
pub struct Revoke<'info> {
    /// [Stream] account.
    #[account(mut)]
    pub stream: Box<Account<'info, Stream>>,

    /// Crate token.
    pub crate_token: Box<Account<'info, crate_token::CrateToken>>,

    /// Underlying tokens of the [Stream].
    #[account(mut)]
    pub underlying_tokens: Account<'info, TokenAccount>,
    /// Destination of the underlying tokens backing the [Stream].
    #[account(mut)]
    pub destination_tokens: Account<'info, TokenAccount>,

    /// The [Stream::revoker].
    pub revoker: Signer<'info>,

    /// [crate_token] program.
    pub crate_token_program: Program<'info, crate_token::program::CrateToken>,
    /// SPL [token] program.
    pub token_program: Program<'info, token::Token>,
}

impl<'info> Revoke<'info> {
    fn revoke(&mut self) -> ProgramResult {
        // redeem the crate tokens
        self.withdraw_all_crate_tokens()?;

        // invalidate the stream
        let stream = &mut self.stream;
        stream.redeemed_amount = stream.initial_amount;
        stream.end_ts = Clock::get()?.unix_timestamp;

        Ok(())
    }

    fn withdraw_all_crate_tokens(&self) -> ProgramResult {
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
            self.underlying_tokens.amount,
        )
    }
}

pub fn handler(ctx: Context<Revoke>) -> ProgramResult {
    ctx.accounts.revoke()?;

    let stream = &ctx.accounts.stream;
    emit!(RevokeEvent {
        stream: stream.key(),
        mint: stream.underlying_mint,
        revoker: ctx.accounts.revoker.key(),
    });

    Ok(())
}

/// Emitted on [crate::venko::revoke].
#[event]
pub struct RevokeEvent {
    /// The [Stream].
    #[index]
    pub stream: Pubkey,
    /// Mint of the underlying token.
    #[index]
    pub mint: Pubkey,
    /// Account that revoked the [Stream].
    pub revoker: Pubkey,
}

impl<'info> Validate<'info> for Revoke<'info> {
    fn validate(&self) -> ProgramResult {
        invariant!(self.stream.revoker.is_some(), Irrevocable);
        assert_keys_eq!(
            #[allow(clippy::unwrap_used)]
            self.stream.revoker.unwrap(),
            self.revoker,
            NotRevoker
        );

        assert_keys_eq!(self.crate_token, self.stream.crate_token);
        assert_keys_eq!(self.underlying_tokens, self.stream.underlying_tokens);
        assert_keys_eq!(self.destination_tokens.mint, self.stream.underlying_mint);
        Ok(())
    }
}
