import { buildCoderMap } from "@saberhq/anchor-contrib";
import { PublicKey } from "@solana/web3.js";

import type { VenkoProgram, VenkoTypes } from "./programs";
import { VenkoJSON } from "./programs";

/**
 * Venko program types.
 */
export interface VenkoPrograms {
  Venko: VenkoProgram;
}

/**
 * Venko addresses.
 */
export const VENKO_ADDRESSES = {
  Venko: new PublicKey("AnatoLyYrd5iaAe36Lvq2oS4nuVDnRAb3KBVCARt4XiZ"),
};

/**
 * Program IDLs.
 */
export const VENKO_IDLS = {
  Venko: VenkoJSON,
};

/**
 * Coders.
 */
export const VENKO_CODERS = buildCoderMap<{
  Venko: VenkoTypes;
}>(VENKO_IDLS, VENKO_ADDRESSES);
