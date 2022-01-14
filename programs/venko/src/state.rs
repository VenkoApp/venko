//! Struct definitions for accounts that hold state.

use crate::*;
use num_traits::ToPrimitive;

/// A stream of tokens being sent, with a cliff and an optional [Self::revoker].
///
/// When a [Stream] is created, there is one token created for every underlying
/// token backing the [Stream].
#[account]
#[derive(Copy, Debug, Default)]
pub struct Stream {
    /// The mint of the [Stream] token.
    pub mint: Pubkey,
    /// Bump seed.
    pub bump: u8,

    /// An optional account which may invalidate this stream and receive all of the underlying tokens.
    pub revoker: Option<Pubkey>,
    /// The Crate Token.
    pub crate_token: Pubkey,
    /// The mint of the SPL token locked up.
    pub underlying_mint: Pubkey,
    /// Token account holding the underlying tokens.
    pub underlying_tokens: Pubkey,

    /// The starting balance of this release account, i.e., how much was
    /// originally deposited.
    pub initial_amount: u64,
    /// The total amount of tokens that have been redeemed from the [Stream].
    pub redeemed_amount: u64,

    /// The time at which the [Stream] begins.
    pub start_ts: i64,
    /// The time at which the [Stream] starts paying out its tokens.
    pub cliff_ts: i64,
    /// The time at which all tokens are released.
    pub end_ts: i64,
}

impl Stream {
    /// Computes the amount of tokens available for withdrawal.
    /// The `remaining_amount` should be the total supply of the [Stream] token.
    pub fn available_for_withdrawal(&self, current_ts: i64, remaining_amount: u64) -> Option<u64> {
        Some(self.outstanding_released(current_ts)?.min(remaining_amount))
    }

    /// The amount of outstanding locked tokens released.
    pub fn outstanding_released(&self, current_ts: i64) -> Option<u64> {
        self.total_released(current_ts)?
            .checked_sub(self.redeemed_amount)
    }

    /// Returns the total released amount up to the given ts, assuming zero
    /// withdrawals and zero funds sent to other programs.
    pub fn total_released(&self, current_ts: i64) -> Option<u64> {
        if current_ts <= self.cliff_ts {
            return Some(0);
        }

        if current_ts >= self.end_ts {
            return Some(self.initial_amount);
        }

        // Signed division not supported.
        let current_ts = current_ts.to_u64()?;
        let start_ts = self.start_ts.to_u64()?;
        let end_ts = self.end_ts.to_u64()?;

        if current_ts <= start_ts {
            return Some(0);
        }

        if current_ts >= end_ts {
            return Some(self.initial_amount);
        }

        (current_ts.checked_sub(start_ts)? as u128)
            .checked_mul(self.initial_amount.into())?
            .checked_div(end_ts.checked_sub(start_ts)?.into())?
            .to_u64()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_unlock_not_started() {
        let release = &mut Stream::default();
        release.start_ts = 100_000;
        release.end_ts = 200_000;
        release.initial_amount = 1_000_000;
        let amt = release.total_released(90_000).unwrap();
        assert_eq!(amt, 0);
    }

    #[test]
    fn test_linear_unlock_finished() {
        let release = &mut Stream::default();
        release.start_ts = 100_000;
        release.end_ts = 200_000;
        release.initial_amount = 1_000_000;
        let amt = release.total_released(290_000).unwrap();
        assert_eq!(amt, 1_000_000);
    }

    #[test]
    fn test_linear_unlock_halfway() {
        let release = &mut Stream::default();
        release.start_ts = 100_000;
        release.end_ts = 200_000;
        release.initial_amount = 1_000_000;
        let amt = release.total_released(150_000).unwrap();
        assert_eq!(amt, 500_000);
    }
}
