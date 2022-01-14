import * as anchor from "@project-serum/anchor";
import { makeSaberProvider } from "@saberhq/anchor-contrib";
import { chaiSolana } from "@saberhq/chai-solana";
import chai from "chai";

import type { VenkoPrograms } from "../../src";
import { VenkoSDK } from "../../src";

chai.use(chaiSolana);

export type Workspace = VenkoPrograms;

export const makeSDK = (): VenkoSDK => {
  const anchorProvider = anchor.Provider.env();
  anchor.setProvider(anchorProvider);
  const provider = makeSaberProvider(anchorProvider);
  return VenkoSDK.load({
    provider,
  });
};
