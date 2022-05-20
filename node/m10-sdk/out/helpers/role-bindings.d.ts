/// <reference types="node" />
import type { LedgerClient } from "../client";
import * as utils from "../utils";
export declare function updateRoleBinding(client: LedgerClient, signer: utils.CryptoSigner, id: string, subjects: Uint8Array[]): Promise<void>;
export declare function setupRoleBindings(ledgerClient: LedgerClient, bankAdminSigner: utils.CryptoSigner, usersToAdd: Buffer[]): Promise<void>;
