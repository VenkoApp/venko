//! Venko: Rails for realtime finance on Solana.
//!
//! # About
//!
//! Venko is a protocol for issuing tokenized payments streams. It is designed for a variety
//! of common payments usecases, including:
//!
//! - **Token lockups.** Issue team tokens.
//! - **Grants.** Issue revocable grants with a cliff and release schedule.
//! - **Escrow.** Send tokens to someone with a cancellation period, allowing you to cancel the payment if it was sent to the wrong address.
//!
//! # License
//!
//! Venko Protocol is licensed under the GNU Affero General Public License v3.0.
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![deny(clippy::unwrap_used)]

mod macros;

use anchor_lang::prelude::*;
use vipers::Validate;

mod instructions;
mod state;

pub use instructions::*;
pub use state::*;

declare_id!("AnatoLyYrd5iaAe36Lvq2oS4nuVDnRAb3KBVCARt4XiZ");

/// The [venko] program.
#[program]
pub mod venko {
    use super::*;

    /// Creates a new [Stream].
    #[access_control(ctx.accounts.validate())]
    pub fn create_stream(
        ctx: Context<CreateStream>,
        stream_bump: u8,
        crate_bump: u8,
        start_ts: i64,
        cliff_ts: i64,
        end_ts: i64,
        revoker: Option<Pubkey>,
    ) -> ProgramResult {
        instructions::create_stream::handler(
            ctx,
            stream_bump,
            crate_bump,
            start_ts,
            cliff_ts,
            end_ts,
            revoker,
        )
    }

    /// Redeems [Stream] tokens for their underlying.
    #[access_control(ctx.accounts.validate())]
    pub fn redeem(ctx: Context<Redeem>, amount: u64) -> ProgramResult {
        instructions::redeem::handler(ctx, amount)
    }

    /// Revokes all underlying [Stream] tokens, invalidating them
    /// and sending all of the [Stream::underlying_tokens] to an address.
    ///
    /// Only the [Stream::revoker] may call this instruction.
    ///
    /// [Stream] tokens will still be in the user's wallet, so it is up to the
    /// [Stream] token holder to validate that the [Stream::underlying_tokens] account
    /// still holds the full balance of underlying tokens.
    #[access_control(ctx.accounts.validate())]
    pub fn revoke(ctx: Context<Revoke>) -> ProgramResult {
        instructions::revoke::handler(ctx)
    }
}

/// Errors.
#[error]
pub enum ErrorCode {
    #[msg("Stream must end after its start time.")]
    InvalidSchedule,
    #[msg("Insufficient withdrawal balance.")]
    InsufficientWithdrawalBalance,
    #[msg("Insufficient stream token balance.")]
    InsufficientStreamTokens,
    #[msg("Stream is irrevocable.")]
    Irrevocable,
    #[msg("Must be revoker to perform this operation.")]
    NotRevoker,
}
