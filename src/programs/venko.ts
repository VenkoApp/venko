import type { AnchorTypes } from "@saberhq/anchor-contrib";

import type { VenkoIDL } from "../idls/venko";

export * from "../idls/venko";

export type VenkoTypes = AnchorTypes<
  VenkoIDL,
  {
    stream: StreamData;
  }
>;

type Accounts = VenkoTypes["Accounts"];

export type StreamData = Accounts["Stream"];

export type VenkoProgram = VenkoTypes["Program"];
