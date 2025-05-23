import { expect } from "chai";
import { beforeEach, describe, it } from "mocha";

import {
    convertMemoToAny,
    CryptoSigner,
    M10Client,
    Collection,
    type AccountId,
    // getFISAccessToken,
    ResourceId,
} from "../../src";
import { AccountMetadata, Bank, BankAccountRef, BankAccountRef_BankAccountType } from "../../src/protobufs/sdk/model/model";
import { RoleBinding } from "../../src/protobufs/sdk/rbac";
import { Instrument } from "../../src/protobufs/sdk/transaction/transaction";
import { Operation_InsertDocument, Operation_UpdateDocument, Value } from "../../src/protobufs/sdk/document";
import { parse } from "uuid";
import { LEDGER_URL, OPERATOR_KEY } from "../config";

const INCREASED_TEST_TIMEOUT: number = 12000;

describe("Hybrid CBDC Bootstrap", () => {
    let operatorSigner: CryptoSigner;
    let bankAdminSigner: CryptoSigner;
    let cbdcSigner: CryptoSigner;

    let operatorClient: M10Client;
    let cbdcClient: M10Client;
    let currencyLedgerAccountId: AccountId;

    let bankName: string;
    let instrumentCode: string;
    let instrumentDesc: string;
    let createdAccount: AccountId;

    const CREATE_ISSUANCE_AMOUNT: number = 10000;
    const DESTROY_ISSUANCE_AMOUNT: number = 9000;

    const BANK_ADMIN_KEY = "MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK";
    const CBDC_KEY = "MFMCAQEwBQYDK2VwBCIEIPx4/t2ofbHbOzNFDHIppkxFmdcOwCrb2wihczFB23sUoSMDIQA5vchjxwfcRTl7c1OBpIoQ47ah5iMaebXWO8ASVZzrJQ==";

    // these are generated out of the box
    const ISSUER_ROLE_UUID = "77436769-8bf9-4581-b0cb-e62b66d5d9a9";
    const SANDBOX_NOSTRO_ROLE = "0c8a1d5f-ec36-4342-93ca-0893103e9b60";

    beforeEach(async () => {
        bankAdminSigner = await CryptoSigner.fromPkcs8Pem(BANK_ADMIN_KEY);
        cbdcSigner = await CryptoSigner.fromPkcs8Pem(CBDC_KEY);
        operatorSigner = await CryptoSigner.fromPkcs8Pem(OPERATOR_KEY);

        // const fisAccessToken = await getFISAccessToken(
        //     process.env.FIS_OAUTH_URL || LEDGER_URL,
        //     process.env.FIS_CLIENT_ID || "m10-sdk-test",
        //     process.env.FIS_CLIENT_SECRET || "m10-sdk-test",
        // );

        cbdcClient = new M10Client(
            LEDGER_URL,
            cbdcSigner,
            // fisAccessToken.accessToken
        );
        operatorClient = new M10Client(
            LEDGER_URL,
            operatorSigner,
            // fisAccessToken.accessToken
        );

        bankName = "Test Bank";
        instrumentCode = "XYZ";
        instrumentDesc = "XYZ Currency";

        /* eslint-disable no-console */
        console.log("Operator key account", operatorSigner.getPublicKey().toString());
    });

    it("Create currency", async () => {
        const CURRENCY_DECIMALS = 2;

        // Create a ledger account for the instrument
        const [, accountId] = await operatorClient.createAccount({
            instrument: Instrument.create({
                code: instrumentCode,
                decimalPlaces: CURRENCY_DECIMALS,
                description: instrumentDesc,
            }),
            frozen: false,
            issuance: true,
            balanceLimit: 0n,
        });
        currencyLedgerAccountId = accountId;

        expect(currencyLedgerAccountId).to.exist;

        /* eslint-disable no-console */
        console.log(`Ledger currency ${instrumentCode} created with ID: ${currencyLedgerAccountId.hex}`);

        // Create an associated account with more public information
        const currencyLedgerAccountMetadataTxId = await operatorClient.documents([{
            oneofKind: "insertDocument",
            insertDocument: {
                collection: Collection.AccountMetadata,
                document: AccountMetadata.toBinary(
                    AccountMetadata.create({
                        id: currencyLedgerAccountId.bytes,
                        name: "XYZ Account",
                        publicName: "XYZ Account",
                        owner: operatorSigner.getPublicKey().toUint8Array(),
                    }),
                ),
            },
        }]);

        /* eslint-disable no-console */
        console.log(`Account for ${instrumentCode} created with ID: ${currencyLedgerAccountId.hex}`);

        expect(currencyLedgerAccountMetadataTxId).to.exist;

        const roleBindingDocument = RoleBinding.create({
            id: parse(ISSUER_ROLE_UUID),
            subjects: [cbdcSigner.getPublicKey().toUint8Array()],
        });
        // Add the central bank key to the Issuer role binding
        await operatorClient.documents([{
            oneofKind: "updateDocument",
            updateDocument: Operation_UpdateDocument.create({
                collection: Collection.RoleBinding,
                primaryKey: Value.create({
                    value: {
                        oneofKind: "bytesValue",
                        bytesValue: roleBindingDocument.id,
                    },
                }),
                document: RoleBinding.toBinary(roleBindingDocument),
                fieldMask: { paths: ["subjects"] },
                mergeRepeated: false,
            }),
        }]);
    }).timeout(INCREASED_TEST_TIMEOUT);

    it("Creating bank accounts for 'XYZ' currency", async () => {
        // Normally we would find our instrument but its been created above
        const [,createdLedgerAccountId] = await operatorClient.createAccount({
            parentId: currencyLedgerAccountId.bytes,
        });

        /* eslint-disable no-console */
        console.log("Id of created ledger account", createdLedgerAccountId.hex);

        const createdLedgerAccountMetadataTxId = await cbdcClient.documents([{
            oneofKind: "insertDocument",
            insertDocument: Operation_InsertDocument.create({
                collection: Collection.AccountMetadata,
                document: AccountMetadata.toBinary(
                    AccountMetadata.create({
                        id: createdLedgerAccountId.bytes,
                        name: bankName,
                        publicName: bankName,
                        owner: cbdcSigner.getPublicKey().toUint8Array(),
                    }),
                ),
            }),
        }]);
        createdAccount = createdLedgerAccountId;

        /* eslint-disable no-console */
        console.log("Created account: ", createdAccount.hex, " with txID: ", createdLedgerAccountMetadataTxId);

        const cdbcRoleBindingDocument = RoleBinding.create({
            id: parse(SANDBOX_NOSTRO_ROLE),
            subjects: [bankAdminSigner.getPublicKey().toUint8Array()],
        });

        await cbdcClient.documents([{
            oneofKind: "updateDocument",
            updateDocument: Operation_UpdateDocument.create({
                collection: Collection.RoleBinding,
                primaryKey: Value.create({
                    value: {
                        oneofKind: "bytesValue",
                        bytesValue: cdbcRoleBindingDocument.id,
                    },
                }),
                document: RoleBinding.toBinary(cdbcRoleBindingDocument),
                fieldMask: { paths: ["subjects"] },
                mergeRepeated: false,
            }),
        }]);
    }).timeout(INCREASED_TEST_TIMEOUT);

    it("Create currency transfer from the holding account to an account with a memo", async () => {
        /* eslint-disable no-console */
        console.log("FROM ACCOUNT: ", currencyLedgerAccountId.hex);

        /* eslint-disable no-console */
        console.log("TO ACCOUNT: ", createdAccount.hex);

        /* eslint-disable no-console */
        console.log("VALUE: ", CREATE_ISSUANCE_AMOUNT);

        const txId = await operatorClient.transfer([{
            fromAccountId: currencyLedgerAccountId.bytes,
            toAccountId: createdAccount.bytes,
            amount: BigInt(CREATE_ISSUANCE_AMOUNT),
            metadata: [convertMemoToAny({ plaintext: "memo here" })],
        }]);

        /* eslint-disable no-console */
        console.log("TX ID: ", txId);
    }).timeout(INCREASED_TEST_TIMEOUT);

    it("Destroy currency through a transfer from the account to the holding account", async () => {
        /* eslint-disable no-console */
        console.log("FROM ACCOUNT: ", createdAccount.hex);

        /* eslint-disable no-console */
        console.log("TO ACCOUNT: ", currencyLedgerAccountId.hex);

        /* eslint-disable no-console */
        console.log("VALUE: ", CREATE_ISSUANCE_AMOUNT);

        const txId = await operatorClient.transfer([{
            fromAccountId: currencyLedgerAccountId.bytes,
            toAccountId: createdAccount.bytes,
            amount: BigInt(DESTROY_ISSUANCE_AMOUNT),
            metadata: [convertMemoToAny({ plaintext: "currency destruction example" })],
        }]);

        /* eslint-disable no-console */
        console.log("TX ID: ", txId);
    }).timeout(INCREASED_TEST_TIMEOUT);

    it("Create bank object from bank accounts", async () => {
        /* eslint-disable no-console */
        console.log("CREATING BANK");

        const bankCreationTxId = await operatorClient.documents([{
            oneofKind: "insertDocument",
            insertDocument: Operation_InsertDocument.create({
                collection: Collection.Bank,
                document: Bank.toBinary(
                    Bank.create({
                        id: ResourceId.generate().bytes,
                        owner: createdAccount.bytes,
                        shortName: "BNKX",
                        displayName: "XYZ Bank",
                        accounts: [
                            BankAccountRef.create({
                                accountId: createdAccount.bytes,
                                accountType: BankAccountRef_BankAccountType.CBDC,
                            }),
                        ],
                    }),
                ),
            }),
        }]);

        /* eslint-disable no-console */
        console.log("TX ID: ", bankCreationTxId);
    }).timeout(INCREASED_TEST_TIMEOUT);

});
