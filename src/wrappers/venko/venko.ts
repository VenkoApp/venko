import {
  CRATE_ADDRESSES,
  generateCrateAddress,
} from "@crateprotocol/crate-sdk";
import type { TransactionEnvelope } from "@saberhq/solana-contrib";
import type { TokenAmount } from "@saberhq/token-utils";
import {
  createATAInstruction,
  createInitMintInstructions,
  getATAAddress,
  getOrCreateATA,
  getOrCreateATAs,
  SPLToken,
  Token,
  TOKEN_PROGRAM_ID,
} from "@saberhq/token-utils";
import type { Signer } from "@solana/web3.js";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import BN from "bn.js";

import { VENKO_CODERS } from "../..";
import type { StreamData, VenkoProgram } from "../../programs/venko";
import type { VenkoSDK } from "../../sdk";
import { findStreamAddress } from "./pda";

/**
 * Handles interacting with the Venko program.
 */
export class VenkoWrapper {
  readonly program: VenkoProgram;

  /**
   * Constructor for a {@link VenkoWrapper}.
   * @param sdk
   */
  constructor(readonly sdk: VenkoSDK) {
    this.program = sdk.programs.Venko;
  }

  get provider() {
    return this.sdk.provider;
  }

  /**
   * Fetches a Stream.
   * @param key
   * @returns
   */
  async fetchStream(key: PublicKey): Promise<StreamData | null> {
    return await this.program.account.stream.fetchNullable(key);
  }

  /**
   * Creates a Venko Stream.
   * @returns
   */
  async createStream({
    amount,
    startTS,
    cliffTS = startTS,
    endTS,
    mintKP = Keypair.generate(),
    revoker,
    owner = this.provider.wallet.publicKey,
    recipient = this.provider.wallet.publicKey,
    payer = this.provider.wallet.publicKey,
  }: {
    amount: TokenAmount;
    startTS: number;
    cliffTS?: number;
    endTS: number;
    mintKP?: Signer;
    revoker?: PublicKey;
    /**
     * Owner of the underlying tokens to be streamed.
     */
    owner?: PublicKey;
    /**
     * Recipient of the Stream tokens.
     */
    recipient?: PublicKey;
    /**
     * Payer of the initial tokens.
     */
    payer?: PublicKey;
  }): Promise<{
    stream: PublicKey;
    token: Token;
    tx: TransactionEnvelope;
  }> {
    const [stream, streamBump] = await findStreamAddress(mintKP.publicKey);
    const [crateToken, crateBump] = await generateCrateAddress(
      mintKP.publicKey
    );
    const ownerUnderlyingATA = await getOrCreateATA({
      provider: this.provider,
      mint: amount.token.mintAccount,
      owner,
    });
    const recipientStreamATA = await getOrCreateATA({
      provider: this.provider,
      mint: mintKP.publicKey,
      owner: recipient,
    });
    const underlyingTokensATA = await getATAAddress({
      mint: amount.token.mintAccount,
      owner: crateToken,
    });
    const underlyingTokensIX = createATAInstruction({
      address: underlyingTokensATA,
      mint: amount.token.mintAccount,
      owner: crateToken,
      payer,
    });
    const endDate = new Date(endTS * 1_000);
    const token = Token.fromMint(mintKP.publicKey, amount.token.decimals, {
      ...amount.token.info,
      name: `Venko ${
        amount.token.symbol
      } Stream (ends ${endDate.toLocaleString()})`,
      symbol: `v${amount.token.symbol}`,
    });

    return {
      stream,
      token,
      tx: (
        await createInitMintInstructions({
          provider: this.provider,
          mintKP,
          decimals: amount.token.decimals,
          mintAuthority: crateToken,
          freezeAuthority: crateToken,
        })
      ).combine(
        this.provider.newTX(
          [
            recipientStreamATA.instruction,
            ownerUnderlyingATA.instruction,
            underlyingTokensIX,
            SPLToken.createTransferInstruction(
              TOKEN_PROGRAM_ID,
              ownerUnderlyingATA.address,
              underlyingTokensATA,
              owner,
              [],
              amount.toU64()
            ),
            VENKO_CODERS.Venko.encodeIX(
              "createStream",
              {
                streamBump,
                crateBump,
                startTs: new BN(startTS),
                cliffTs: new BN(cliffTS),
                endTs: new BN(endTS),
                revoker: revoker ?? PublicKey.default,
              },
              {
                streamMint: mintKP.publicKey,
                stream,
                underlyingMint: amount.token.mintAccount,
                underlyingTokens: underlyingTokensATA,
                destination: recipientStreamATA.address,
                crateToken,
                payer,
                systemProgram: SystemProgram.programId,
                crateTokenProgram: CRATE_ADDRESSES.CrateToken,
                tokenProgram: TOKEN_PROGRAM_ID,
              }
            ),
          ],
          [mintKP]
        )
      ),
    };
  }

  /**
   * Redeems Stream tokens.
   * @returns
   */
  async redeem({
    amount,
    owner = this.provider.wallet.publicKey,
  }: {
    /**
     * Amount of Stream tokens to redeem.
     */
    amount: TokenAmount;
    /**
     * Owner of the initial tokens.
     */
    owner?: PublicKey;
  }): Promise<TransactionEnvelope> {
    const [stream] = await findStreamAddress(amount.token.mintAccount);
    const streamData = await this.fetchStream(stream);
    if (!streamData) {
      throw new Error(`stream not found: ${stream.toString()}`);
    }
    const ownerATAs = await getOrCreateATAs({
      provider: this.provider,
      mints: {
        underlying: streamData.underlyingMint,
        stream: amount.token.mintAccount,
      },
      owner,
    });
    return this.provider.newTX([
      ...ownerATAs.instructions,
      VENKO_CODERS.Venko.encodeIX(
        "redeem",
        {
          amount: amount.toU64(),
        },
        {
          streamMint: amount.token.mintAccount,
          stream,
          sourceStreamTokens: ownerATAs.accounts.stream,
          underlyingTokens: streamData.underlyingTokens,
          destinationTokens: ownerATAs.accounts.underlying,
          crateToken: streamData.crateToken,
          userAuthority: owner,
          systemProgram: SystemProgram.programId,
          crateTokenProgram: CRATE_ADDRESSES.CrateToken,
          tokenProgram: TOKEN_PROGRAM_ID,
        }
      ),
    ]);
  }

  /**
   * Revokes a Stream.
   * @returns
   */
  async revoke({
    streamMint,
    owner = this.provider.wallet.publicKey,
    revoker = this.provider.wallet.publicKey,
  }: {
    /**
     * The mint of the Stream to revoke.
     */
    streamMint: PublicKey;
    /**
     * Owner to send the tokens to.
     */
    owner?: PublicKey;
    revoker?: PublicKey;
  }): Promise<TransactionEnvelope> {
    const [stream] = await findStreamAddress(streamMint);
    const streamData = await this.fetchStream(stream);
    if (!streamData) {
      throw new Error(`stream not found: ${stream.toString()}`);
    }
    const ownerATAs = await getOrCreateATAs({
      provider: this.provider,
      mints: {
        underlying: streamData.underlyingMint,
      },
      owner,
    });
    return this.provider.newTX([
      ...ownerATAs.instructions,
      VENKO_CODERS.Venko.encodeIX(
        "revoke",
        {},
        {
          stream,
          crateToken: streamData.crateToken,
          underlyingTokens: streamData.underlyingTokens,
          destinationTokens: ownerATAs.accounts.underlying,
          revoker,
          crateTokenProgram: CRATE_ADDRESSES.CrateToken,
          tokenProgram: TOKEN_PROGRAM_ID,
        }
      ),
    ]);
  }
}
