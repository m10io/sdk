import { assert } from "chai";
import type { Context } from "mocha";

import { m10 } from "../protobufs";
import { AccountBuilder, DocumentBuilder, PageBuilder } from "../src";
import { M10Client } from "../src/client";
import { Collection, DocumentUpdate } from "../src/collections";
import { M10Error } from "../src/error";
import type { AccountId } from "../src/ids";
import { DocumentId } from "../src/ids";
import { arrayIsNotEmpty, CryptoSigner, unwrap } from "../src/utils";


export type InjectableContext = Readonly<{ context?: TestSuiteContext }>;

export interface TestSuiteContext {
    bankAdminSigner: CryptoSigner;
    client: M10Client;
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

            const BANK_ADMIN_KEY = "MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK";
            const BANK_NAME = "NodeTB TTT";
            const LEDGER_URL = process.env.LEDGER_URL || "develop.m10.net";

            // -------------------------------------------------------------------------
            // Get bank admin account
            // -------------------------------------------------------------------------

            const bankAdminSigner = CryptoSigner.getSignerFromPkcs8V1(BANK_ADMIN_KEY);
            const bankAdminClient = new M10Client(LEDGER_URL, bankAdminSigner);

            const accountMetadatas = await bankAdminClient.listAccountMetadatas(
                PageBuilder.byOwner(bankAdminSigner.getPublicKey()),
            );

            const parentAccount = accountMetadatas.find((account) => account.publicName === BANK_NAME);
            const parentAccountId = unwrap(parentAccount?.id, M10Error.Other("parentAccountId is None"));

            // -------------------------------------------------------------------------
            // Create user "alice"
            // -------------------------------------------------------------------------

            const aliceSigner = CryptoSigner.generateKeyPair();
            const aliceId = new DocumentId("account-set");

            const aliceCreationTxId = await bankAdminClient.documents(
                new DocumentBuilder()
                    .insert(
                        Collection.AccountSet,
                        new m10.sdk.model.AccountSet({
                            owner: aliceSigner.getPublicKey().toUint8Array(),
                            id: aliceId.toUint8Array(),
                        }),
                    ),
            );

            assert.isNotEmpty(aliceCreationTxId);

            // -------------------------------------------------------------------------
            // Create user "bob"
            // -------------------------------------------------------------------------

            const bobSigner = CryptoSigner.generateKeyPair();
            const bobId = new DocumentId("account-set");

            const bobCreationTxId = await bankAdminClient.documents(
                new DocumentBuilder()
                    .insert(
                        Collection.AccountSet,
                        new m10.sdk.model.AccountSet({
                            owner: bobSigner.getPublicKey().toUint8Array(),
                            id: bobId.toUint8Array(),
                        }),
                    ),
            );

            assert.isNotEmpty(bobCreationTxId);

            // -------------------------------------------------------------------------
            // Setup role bindings (necessary for users, so they can create accounts)
            // -------------------------------------------------------------------------

            const roleBindings = await bankAdminClient.listRoleBindings(PageBuilder.byName("node-test-customer"));

            assert.isNotEmpty(roleBindings);

            const roleBindingId = unwrap(roleBindings[0].id?.toString(), M10Error.Other("roleBinding.id is None"));
            const roleBindingSubjects = unwrap(roleBindings[0].subjects, M10Error.Other("roleBinding.subjects is None"));

            const subjects = [aliceSigner.getPublicKey(), bobSigner.getPublicKey()]
                .filter((publicKey) => !roleBindingSubjects.includes(publicKey));

            if (arrayIsNotEmpty(subjects)) {
                await bankAdminClient.documents(
                    new DocumentBuilder()
                        .update(
                            new DocumentUpdate(
                                Collection.RoleBinding,
                                new m10.sdk.RoleBinding({
                                    id: new DocumentId("role-binding", roleBindingId).toUint8Array(),
                                    subjects: subjects.map((pk) => pk.toUint8Array()),
                                }),
                                ["subjects"],
                            ),
                        ),
                );
            }

            // -------------------------------------------------------------------------
            // Create accounts
            // -------------------------------------------------------------------------

            const bobsClient = new M10Client(LEDGER_URL, bobSigner);

            const [,bobsAccountId] = await bobsClient.createAccount(new AccountBuilder(parentAccountId));

            const bobAccountMetadataName = `Bob ${Date.now()}`;
            await bobsClient.documents(
                new DocumentBuilder()
                    .insert(
                        Collection.AccountMetadata,
                        new m10.sdk.model.AccountMetadata({
                            id: bobsAccountId.toUint8Array(),
                            name: bobAccountMetadataName,
                            publicName: bobAccountMetadataName,
                            owner: bobSigner.getPublicKey().toUint8Array(),
                        }),
                    ),
            );

            assert.isNotEmpty(bobsAccountId);

            // -------------------------------------------------------------------------
            // Setup Context
            // -------------------------------------------------------------------------

            const injectableContext: InjectableContext = {
                context: {
                    bankAdminSigner,
                    client: bankAdminClient,
                    parentAccountId: parentAccountId,
                    bobsAccountId: bobsAccountId,
                },
            };

            Object.assign(this, injectableContext);
        },
    };
};
