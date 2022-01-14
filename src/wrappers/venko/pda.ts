import { utils } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";

import { VENKO_ADDRESSES } from "../../constants";

/**
 * Finds the address of a Venko Stream.
 */
export const findStreamAddress = async (
  mint: PublicKey
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [utils.bytes.utf8.encode("Stream"), mint.toBuffer()],
    VENKO_ADDRESSES.Venko
  );
};
