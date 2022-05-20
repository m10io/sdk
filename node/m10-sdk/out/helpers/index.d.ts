import type { LedgerClient } from "../client";
import { CryptoSigner } from "../utils";
export declare function createUser(client: LedgerClient): Promise<[CryptoSigner, string]>;
export * as roleBindings from "./role-bindings";
export * as accounts from "./accounts";
