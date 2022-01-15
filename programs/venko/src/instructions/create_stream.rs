//! Instruction handler for [crate::venko::create_stream].

use crate::*;
use anchor_spl::token::{self, Mint, TokenAccount};
use vipers::{assert_keys_eq, invariant, Validate};

/// Accounts for [venko::create_stream].
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreateStream<'info> {
    /// [token::Mint] of the [Stream].
    #[account(mut)]
    pub stream_mint: Account<'info, Mint>,
    /// [Stream] account.
    #[account(
        init,
        seeds = [
            b"Stream",
            stream_mint.key().as_ref()
        ],
        bump = bump,
        payer = payer
    )]
    pub stream: Account<'info, Stream>,
    /// Underlying mint.
    pub underlying_mint: Box<Account<'info, Mint>>,
    /// The [TokenAccount] holding the [Stream]'s tokens.
    /// Must be owned by the [Self::crate_token], and the amount should be > 0.
    pub underlying_tokens: Box<Account<'info, TokenAccount>>,
    /// Destination of the [Stream] tokens.
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,

    /// The [crate_token::CrateToken] to be created.
    #[account(mut)]
    pub crate_token: UncheckedAccount<'info>,
    /// Payer for the [Stream] account creation.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// [System] program.
    pub system_program: Program<'info, System>,
    /// [crate_token] program.
    pub crate_token_program: Program<'info, crate_token::program::CrateToken>,
    /// SPL [token] program.
    pub token_program: Program<'info, token::Token>,
}

impl<'info> CreateStream<'info> {
    fn init_crate(&self, crate_bump: u8) -> ProgramResult {
        crate_token::cpi::new_crate(
            CpiContext::new(
                self.crate_token_program.to_account_info(),
                crate_token::cpi::accounts::NewCrate {
                    crate_mint: self.stream_mint.to_account_info(),
                    crate_token: self.crate_token.to_account_info(),

                    // no fees
                    fee_to_setter: self.system_program.to_account_info(),
                    fee_setter_authority: self.system_program.to_account_info(),
                    author_fee_to: self.system_program.to_account_info(),

                    // authorities
                    issue_authority: self.stream.to_account_info(),
                    withdraw_authority: self.stream.to_account_info(),
                    payer: self.payer.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                },
            ),
            crate_bump,
        )
    }

    /// Issue the [Stream] tokens.
    fn issue_tokens(&self, amount: u64) -> ProgramResult {
        let signer_seeds: &[&[&[u8]]] = stream_seeds!(self.stream);
        crate_token::cpi::issue(
            CpiContext::new(
                self.crate_token_program.to_account_info(),
                crate_token::cpi::accounts::Issue {
                    crate_mint: self.stream_mint.to_account_info(),
                    crate_token: self.crate_token.to_account_info(),

                    // authorities
                    issue_authority: self.stream.to_account_info(),

                    mint_destination: self.destination.to_account_info(),
                    author_fee_destination: self.destination.to_account_info(),
                    protocol_fee_destination: self.destination.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            amount,
        )
    }

    fn init_stream(
        &mut self,
        stream_bump: u8,
        start_ts: i64,
        cliff_ts: i64,
        end_ts: i64,
        revoker: Pubkey,
    ) -> ProgramResult {
        let stream = &mut self.stream;
        stream.mint = self.stream_mint.key();
        stream.bump = stream_bump;

        stream.revoker = revoker;
        stream.crate_token = self.crate_token.key();
        stream.underlying_mint = self.underlying_tokens.mint.key();
        stream.underlying_tokens = self.underlying_tokens.key();

        stream.initial_amount = self.underlying_tokens.amount;
        stream.redeemed_amount = 0;

        stream.start_ts = start_ts;
        stream.cliff_ts = cliff_ts;
        stream.end_ts = end_ts;
        Ok(())
    }
}

pub fn handler(
    ctx: Context<CreateStream>,
    stream_bump: u8,
    crate_bump: u8,
    start_ts: i64,
    cliff_ts: i64,
    end_ts: i64,
    revoker: Pubkey,
) -> ProgramResult {
    invariant!(end_ts > start_ts, InvalidSchedule);

    invariant!(cliff_ts >= start_ts);
    invariant!(cliff_ts <= end_ts);

    let amount = ctx.accounts.underlying_tokens.amount;
    ctx.accounts.init_crate(crate_bump)?;
    ctx.accounts
        .init_stream(stream_bump, start_ts, cliff_ts, end_ts, revoker)?;
    ctx.accounts.issue_tokens(amount)?;

    let stream = &ctx.accounts.stream;
    emit!(StreamCreateEvent {
        stream: stream.key(),
        mint: stream.mint,
        amount: stream.initial_amount,
        start_ts: stream.start_ts,
        cliff_ts: stream.cliff_ts,
        end_ts: stream.end_ts,
    });

    Ok(())
}

#[event]
pub struct StreamCreateEvent {
    #[index]
    pub stream: Pubkey,
    #[index]
    pub mint: Pubkey,
    pub amount: u64,
    pub start_ts: i64,
    pub cliff_ts: i64,
    pub end_ts: i64,
}

impl<'info> Validate<'info> for CreateStream<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys_eq!(self.stream_mint.mint_authority.unwrap(), self.crate_token);
        assert_keys_eq!(self.stream_mint.freeze_authority.unwrap(), self.crate_token);
        invariant!(self.stream_mint.supply == 0);

        assert_keys_eq!(self.underlying_tokens.owner, self.crate_token);
        invariant!(self.underlying_tokens.amount > 0);
        invariant!(self.underlying_tokens.delegate.is_none());
        invariant!(self.underlying_tokens.close_authority.is_none());

        assert_keys_eq!(self.underlying_tokens.mint, self.underlying_mint);
        invariant!(self.underlying_mint.decimals == self.stream_mint.decimals);

        Ok(())
    }
}
