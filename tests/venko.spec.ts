import { expectTXTable } from "@saberhq/chai-solana";
import {
  createMint,
  getOrCreateATA,
  sleep,
  SPLToken,
  Token,
  TOKEN_PROGRAM_ID,
  TokenAmount,
} from "@saberhq/token-utils";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

import { VenkoSDK } from "../src";
import { makeSDK } from "./workspace/workspace";

describe("Venko", () => {
  const { provider } = makeSDK();

  const adminKP = Keypair.generate();
  const adminSDK = VenkoSDK.load({
    provider: provider.withSigner(adminKP),
  });

  const recipientKP = Keypair.generate();
  const recipientSDK = VenkoSDK.load({
    provider: provider.withSigner(recipientKP),
  });

  before(async () => {
    await (
      await adminSDK.provider.requestAirdrop(LAMPORTS_PER_SOL * 10)
    ).wait();
    await (
      await recipientSDK.provider.requestAirdrop(LAMPORTS_PER_SOL * 10)
    ).wait();
  });

  it("should allow creating an irrevocable stream", async () => {
    const underlyingToken = Token.fromMint(
      await createMint(adminSDK.provider, undefined, 6),
      6
    );
    const amount = TokenAmount.parse(underlyingToken, "10");

    const adminUnderlyingTokens = await getOrCreateATA({
      provider: adminSDK.provider,
      owner: adminSDK.provider.wallet.publicKey,
      mint: underlyingToken.mintAccount,
    });

    await expectTXTable(
      adminSDK.provider.newTX([
        adminUnderlyingTokens.instruction,
        SPLToken.createMintToInstruction(
          TOKEN_PROGRAM_ID,
          underlyingToken.mintAccount,
          adminUnderlyingTokens.address,
          adminSDK.provider.wallet.publicKey,
          [],
          amount.toU64()
        ),
      ])
    ).to.be.fulfilled;

    const nowTS = Math.floor(new Date().getTime() / 1_000);

    const { tx, token: streamToken } = await adminSDK.venko.createStream({
      amount,
      startTS: nowTS,
      endTS: nowTS + 3,
      recipient: recipientKP.publicKey,
    });
    await expectTXTable(tx, "create stream", {
      verbosity: "error",
    }).to.be.fulfilled;

    // wait for stream to be over...
    await sleep(5_000);

    const claimTX = await recipientSDK.venko.redeem({
      amount: TokenAmount.parse(streamToken, "1"),
    });
    await expectTXTable(claimTX, "redeem stream", {
      verbosity: "error",
    }).to.be.fulfilled;
  });

  it("should allow creating a revocable stream", async () => {
    const underlyingToken = Token.fromMint(
      await createMint(adminSDK.provider, undefined, 6),
      6
    );
    const amount = TokenAmount.parse(underlyingToken, "10");

    const adminUnderlyingTokens = await getOrCreateATA({
      provider: adminSDK.provider,
      owner: adminSDK.provider.wallet.publicKey,
      mint: underlyingToken.mintAccount,
    });

    await expectTXTable(
      adminSDK.provider.newTX([
        adminUnderlyingTokens.instruction,
        SPLToken.createMintToInstruction(
          TOKEN_PROGRAM_ID,
          underlyingToken.mintAccount,
          adminUnderlyingTokens.address,
          adminSDK.provider.wallet.publicKey,
          [],
          amount.toU64()
        ),
      ])
    ).to.be.fulfilled;

    const nowTS = Math.floor(new Date().getTime() / 1_000);

    const { tx, token: streamToken } = await adminSDK.venko.createStream({
      amount,
      startTS: nowTS,
      endTS: nowTS + 3,
      recipient: recipientKP.publicKey,
      revoker: adminSDK.provider.wallet.publicKey,
    });
    await expectTXTable(tx, "create stream", {
      verbosity: "error",
    }).to.be.fulfilled;

    // wait for stream to be over...
    await sleep(5_000);

    const claimTX = await recipientSDK.venko.redeem({
      amount: TokenAmount.parse(streamToken, "1"),
    });
    await expectTXTable(claimTX, "redeem stream", {
      verbosity: "error",
    }).to.be.fulfilled;
  });
});
