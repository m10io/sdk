import type { LedgerClient } from "../client";
import * as utils from "../utils";
export declare function getBankAdminAccount(client: LedgerClient, bankAdminSigner: utils.CryptoSigner, bankName: string): Promise<utils.AccountId>;
export declare function createLedgerAccount(client: LedgerClient, signer: utils.CryptoSigner, parentId: string): Promise<string>;
export declare function createAccount(client: LedgerClient, signer: utils.CryptoSigner, accountId: utils.AccountId, name: string): Promise<utils.AccountId>;
