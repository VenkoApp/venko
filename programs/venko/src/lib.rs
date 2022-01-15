//! Venko: Rails for realtime finance on Solana.
//!
//! # About
//!
//! Venko is a protocol for issuing streams of tokens. It is designed for a variety
//! of usecases, including:
//!
//! - **Token lockups.** Issue team tokens over a schedule, irrevocably.
//! - **Grants.** Issue revocable grants with a cliff and release schedule.
//! - **Escrow.** Send tokens to someone with a cancellation period, allowing you to cancel the payment if it was sent to the wrong address.
//!
//! We're in active development. For the latest updates, please join our community:
//!
//! - Twitter: <https://twitter.com/VenkoApp>
//!
//! # Note
//!
//! - **Venko is in active development, so all APIs are subject to change.**
//! - **This code is unaudited. Use at your own risk.**
//!
//! # Addresses
//!
//! Program addresses are the same on devnet, testnet, and mainnet-beta.
//!
//! - Venko: [`AnatoLyYrd5iaAe36Lvq2oS4nuVDnRAb3KBVCARt4XiZ`](https://explorer.solana.com/address/AnatoLyYrd5iaAe36Lvq2oS4nuVDnRAb3KBVCARt4XiZ)
//!
//! # Contribution
//!
//! Thank you for your interest in contributing to Venko Protocol! All contributions are welcome no matter how big or small. This includes
//! (but is not limited to) filing issues, adding documentation, fixing bugs, creating examples, and implementing features.
//!
//! When contributing, please make sure your code adheres to some basic coding guidlines:
//!
//! - Code must be formatted with the configured formatters (e.g. `rustfmt` and `prettier`).
//! - Comment lines should be no longer than 80 characters and written with proper grammar and punctuation.
//! - Commit messages should be prefixed with the package(s) they modify. Changes affecting multiple packages should list all packages. In rare cases, changes may omit the package name prefix.
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
        revoker: Pubkey,
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
