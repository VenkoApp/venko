import { newProgramMap } from "@saberhq/anchor-contrib";
import type { AugmentedProvider, Provider } from "@saberhq/solana-contrib";
import { SolanaAugmentedProvider } from "@saberhq/solana-contrib";
import type { Signer } from "@solana/web3.js";

import type { VenkoPrograms } from "./constants";
import { VENKO_ADDRESSES, VENKO_IDLS } from "./constants";
import { VenkoWrapper } from "./wrappers";

/**
 * Venko SDK.
 */
export class VenkoSDK {
  constructor(
    readonly provider: AugmentedProvider,
    readonly programs: VenkoPrograms
  ) {}

  /**
   * Creates a new instance of the SDK with the given keypair.
   */
  withSigner(signer: Signer): VenkoSDK {
    return VenkoSDK.load({
      provider: this.provider.withSigner(signer),
    });
  }

  /**
   * Loads the SDK.
   * @returns
   */
  static load({ provider }: { provider: Provider }): VenkoSDK {
    const programs: VenkoPrograms = newProgramMap<VenkoPrograms>(
      provider,
      VENKO_IDLS,
      VENKO_ADDRESSES
    );
    return new VenkoSDK(new SolanaAugmentedProvider(provider), programs);
  }

  /**
   * Venko program helpers.
   */
  get venko(): VenkoWrapper {
    return new VenkoWrapper(this);
  }
}
