import { u64 } from "@saberhq/token-utils";

import type { StreamData } from "../../programs/venko";

/**
 * Computes the amount of tokens that may be redeemed on a Stream.
 * @param stream
 * @returns
 */
export const computeRedeemableAmount = (stream: StreamData): u64 => {
  const nowTs = new u64(Math.floor(new Date().getTime() / 1_000));
  const startTs = stream.startTs;
  const cliffTs = stream.cliffTs;
  const endTs = stream.endTs;
  if (nowTs.lt(cliffTs) || nowTs.lt(startTs)) {
    return new u64(0);
  }
  if (nowTs.gte(endTs)) {
    return stream.initialAmount.sub(stream.redeemedAmount);
  }
  const total = stream.initialAmount.sub(stream.redeemedAmount);
  return new u64(nowTs.sub(startTs).div(endTs.sub(startTs).mul(total)));
};
