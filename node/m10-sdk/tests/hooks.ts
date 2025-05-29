import { assert } from "chai";
import type { Context } from "mocha";

import { CryptoSigner, M10Client, AccountId, Collection, getFISAccessToken, ResourceId } from "../src";
import { AccountMetadata, AccountSet } from "../src/protobufs/sdk/model/model";
import { RoleBinding } from "../src/protobufs/sdk/rbac";
import { Operation_InsertDocument, Operation_UpdateDocument, Value } from "../src/protobufs/sdk/document";
import { FieldMask } from "../src/protobufs/google/protobuf/field_mask";

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
        beforeAll: async function(done) {
            this.timeout(INCREASED_TEST_TIMEOUT);

            const BANK_ADMIN_KEY = "MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK";
            const BANK_NAME = "NodeTB TTT";
            const LEDGER_URL = process.env.LEDGER_URL || "develop.m10.net";



            const fisAccessToken = await getFISAccessToken(
                process.env.FIS_OAUTH_URL || "https://api.m10.net",
                process.env.FIS_CLIENT_ID || "m10-sdk-test",
                process.env.FIS_CLIENT_SECRET || "m10-sdk-test",
            );

            // -------------------------------------------------------------------------
            // Get bank admin account
            // -------------------------------------------------------------------------

            const bankAdminSigner = await CryptoSigner.fromPkcs8Pem(BANK_ADMIN_KEY);
            const bankAdminClient = new M10Client(LEDGER_URL, bankAdminSigner, fisAccessToken.accessToken);

            const accountMetadatas = await bankAdminClient.listAccountMetadata({
                filter: {
                    oneofKind: "owner",
                    owner: bankAdminSigner.getPublicKey().toUint8Array(),
                },
            });

            const parentAccount = accountMetadatas.accounts.find((account) => account.publicName === BANK_NAME);
            const parentAccountId = parentAccount?.id;
            if (!parentAccountId) {
                throw new TypeError(`Parent account not found for ${BANK_NAME}`);
            }

            // -------------------------------------------------------------------------
            // Create user "alice"
            // -------------------------------------------------------------------------

            const aliceSigner = await CryptoSigner.generateKeyPair();
            const aliceAccountSetId = ResourceId.generate().bytes;

            const aliceCreationTxId = await bankAdminClient.documents([{
                oneofKind: "insertDocument",
                insertDocument: Operation_InsertDocument.create({
                    collection: Collection.AccountSet,
                    document: AccountSet.toBinary(
                        AccountSet.create({
                            owner: aliceSigner.getPublicKey().toUint8Array(),
                            id: aliceAccountSetId,
                        }),
                    ),
                }),
            }]);

            assert.isNotEmpty(aliceCreationTxId);

            // -------------------------------------------------------------------------
            // Create user "bob"
            // -------------------------------------------------------------------------

            const bobSigner = await CryptoSigner.generateKeyPair();
            const bobAccountSetId = ResourceId.generate().bytes;

            const bobCreationTxId = await bankAdminClient.documents([{
                oneofKind: "insertDocument",
                insertDocument: Operation_InsertDocument.create({
                    collection: Collection.AccountSet,
                    document: AccountSet.toBinary(
                        AccountSet.create({
                            owner: bobSigner.getPublicKey().toUint8Array(),
                            id: bobAccountSetId,
                        }),
                    ),
                }),
            }]);

            assert.isNotEmpty(bobCreationTxId);

            // -------------------------------------------------------------------------
            // Setup role bindings (necessary for users, so they can create accounts)
            // -------------------------------------------------------------------------

            const { roleBindings } = await bankAdminClient.listRoleBindings({
                filter: {
                    oneofKind: "name",
                    name: "node-test-customer",
                },
            });

            assert.isNotEmpty(roleBindings);

            const roleBindingId = roleBindings[0].id?.toString();
            const roleBindingSubjects = roleBindings[0].subjects;

            const subjects = [aliceSigner.getPublicKey(), bobSigner.getPublicKey()]
                .filter((publicKey) => !roleBindingSubjects.includes(publicKey.toUint8Array()));

            if (subjects?.length) {
                const document = RoleBinding.create({
                    id: ResourceId.fromString(roleBindingId).bytes,
                    subjects: subjects.map((pk) => pk.toUint8Array()),
                });
                await bankAdminClient.documents([{
                    oneofKind: "updateDocument",
                    updateDocument: Operation_UpdateDocument.create({
                        collection: Collection.RoleBinding,
                        primaryKey: Value.create({
                            value: {
                                oneofKind: "bytesValue",
                                bytesValue: document.id,
                            },
                        }),
                        document: RoleBinding.toBinary(document),
                        fieldMask: FieldMask.create({ paths: ["subjects"] }),
                        mergeRepeated: false,
                    }),
                }]);
            }

            // -------------------------------------------------------------------------
            // Create accounts
            // -------------------------------------------------------------------------

            const bobsClient = new M10Client(LEDGER_URL, bobSigner, fisAccessToken.accessToken);

            const [, bobsAccountId] = await bobsClient.createAccount({
                parentId: parentAccountId,
                frozen: false,
                issuance: true,
            });

            const bobAccountMetadataName = `Bob ${Date.now()}`;
            await bobsClient.documents([{
                oneofKind: "insertDocument",
                insertDocument: Operation_InsertDocument.create({
                    collection: Collection.AccountMetadata,
                    document: AccountMetadata.toBinary(
                        AccountMetadata.create({
                            id: bobsAccountId.bytes,
                            name: bobAccountMetadataName,
                            publicName: bobAccountMetadataName,
                            owner: bobSigner.getPublicKey().toUint8Array(),
                        }),
                    ),
                }),
            }]);

            assert.isNotEmpty(bobsAccountId);

            // -------------------------------------------------------------------------
            // Setup Context
            // -------------------------------------------------------------------------

            const injectableContext: InjectableContext = {
                context: {
                    bankAdminSigner,
                    client: bankAdminClient,
                    parentAccountId: AccountId.fromBytes(parentAccountId),
                    bobsAccountId: bobsAccountId,
                },
            };

            Object.assign(this, injectableContext);

            done();
        },
    };
};

export const parseUnits = (value: string | number | bigint, decimalPlaces: number | string): bigint => {
    /* eslint-disable @typescript-eslint/no-magic-numbers */

    const decimalPlacesBigInt = BigInt(decimalPlaces);
    const valueBigInt = BigInt(value);
    if (valueBigInt < 0n) {
        throw new TypeError("Value must be a positive number");
    }

    if (decimalPlacesBigInt < 0n) {
        throw new TypeError("Decimal places must be a positive number");
    }

    return valueBigInt * (10n ** decimalPlacesBigInt) ** 2n; // FIXME: remove double **2
    /* eslint-enable @typescript-eslint/no-magic-numbers */
};

export const sleep = (ms: number): Promise<void> => {
    return new Promise((resolve) => {
        setTimeout(resolve, ms);
    });
};
