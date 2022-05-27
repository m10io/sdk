import { assert } from "chai";
import type { Context } from "mocha";

import { LedgerClient } from "../src/client";
import { accounts, createUser, roleBindings } from "../src/helpers";
import type { AccountId } from "../src/utils";
import { CryptoSigner, getPrivateKey } from "../src/utils";


export type InjectableContext = Readonly<{ context?: TestSuiteContext }>;

export interface TestSuiteContext {
    ledgerClient: LedgerClient;
    bankAdminSigner: CryptoSigner;
    parentAccountId: AccountId;
    bobsAccountId: AccountId;
}

// TestContext will be used by all the test
export type TestContext = Mocha.Context & Context & InjectableContext;



// Too many async calls to finish under default timeout
const INCREASED_TEST_TIMEOUT: number = 6000;

export const mochaHooks = (): Mocha.RootHookObject => {
    return {
        beforeAll: async function(this: TestContext) {
            this.timeout(INCREASED_TEST_TIMEOUT);

            const BANK_ADMIN_KEY = getPrivateKey("MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK");
            const BANK_NAME = "NodeTB TTT";
            const LEDGER_URL = process.env.LEDGER_URL || "develop.m10.net";

            // -------------------------------------------------------------------------
            // Get bank admin account
            // -------------------------------------------------------------------------

            const ledgerClient = new LedgerClient(LEDGER_URL, true);

            const bankAdminSigner = new CryptoSigner(BANK_ADMIN_KEY);
            const parentAccountId = await accounts.getBankAdminAccount(ledgerClient, bankAdminSigner, BANK_NAME);

            // -------------------------------------------------------------------------
            // Create user "alice"
            // -------------------------------------------------------------------------

            const [ alice, aliceId ] = await createUser(ledgerClient);
            assert.isNotEmpty(aliceId);

            // -------------------------------------------------------------------------
            // Create user "bob"
            // -------------------------------------------------------------------------

            const [ bob, bobId ] = await createUser(ledgerClient);
            assert.isNotEmpty(bobId);

            // -------------------------------------------------------------------------
            // Setup role bindings (necessary for users, so they can create accounts)
            // -------------------------------------------------------------------------

            await roleBindings.setupRoleBindings(ledgerClient, bankAdminSigner, [ alice.getPublicKey(), bob.getPublicKey() ]);

            // -------------------------------------------------------------------------
            // Create accounts
            // -------------------------------------------------------------------------

            const bobsAccountId = await accounts.createLedgerAccount(ledgerClient, bob, parentAccountId);
            assert.isNotEmpty(bobsAccountId);

            const accountId = await accounts.createAccount(ledgerClient, bob, bobsAccountId, `Bob ${Date.now()}`);
            assert.isNotEmpty(accountId);


            // -------------------------------------------------------------------------
            // Setup Context
            // -------------------------------------------------------------------------

            const injectableContext: InjectableContext = {
                context: {
                    ledgerClient,
                    bankAdminSigner,
                    parentAccountId,
                    bobsAccountId,
                },
            };

            Object.assign(this, injectableContext);
        },
    };
};
