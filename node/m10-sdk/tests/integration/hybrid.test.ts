import { expect } from "chai";
import { beforeEach, describe, it } from "mocha";

import { m10 } from "../../protobufs";
import { AccountBuilder, DocumentBuilder, StepBuilder, TransferBuilder } from "../../src";
import { M10Client } from "../../src/client";
import { Collection, DocumentUpdate } from "../../src/collections";
import type { AccountId } from "../../src/ids/account_id";
import { DocumentId } from "../../src/ids/document_id";
import { convertMemoToAny, CryptoSigner } from "../../src/utils";

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
    const OPERATOR_KEY = "MFMCAQEwBQYDK2VwBCIEIMA9e6xaIFATarh2S5yVEe7jene/36EOcM+3B/sOH0y/oSMDIQDA9/WOLKaVoXq92lOSjloExzLXCdRz8oW9TweVGAGqZg==";
    const CBDC_KEY = "MFMCAQEwBQYDK2VwBCIEIPx4/t2ofbHbOzNFDHIppkxFmdcOwCrb2wihczFB23sUoSMDIQA5vchjxwfcRTl7c1OBpIoQ47ah5iMaebXWO8ASVZzrJQ==";

    // these are generated out of the box
    const ISSUER_ROLE_UUID = "77436769-8bf9-4581-b0cb-e62b66d5d9a9";
    const SANDBOX_NOSTRO_ROLE = "0c8a1d5f-ec36-4342-93ca-0893103e9b60";

    const LEDGER_URL = process.env.LEDGER_URL || "develop.m10.net";

    beforeEach(async () => {
        bankAdminSigner = CryptoSigner.getSignerFromPkcs8V1(BANK_ADMIN_KEY);
        cbdcSigner = CryptoSigner.getSignerFromPkcs8V2(CBDC_KEY);
        operatorSigner = CryptoSigner.getSignerFromPkcs8V2(OPERATOR_KEY);

        cbdcClient = new M10Client(LEDGER_URL, cbdcSigner);
        operatorClient = new M10Client(LEDGER_URL, operatorSigner);

        bankName = "Test Bank";
        instrumentCode = "XYZ";
        instrumentDesc = "XYZ Currency";

        /* eslint-disable no-debugger, no-console */
        console.log("Operator key account", operatorSigner.getPublicKey().toString());
    });

    it("Create currency", async () => {
        const CURRENCY_DECIMALS = 2;
        const account = new AccountBuilder()
            .instrument(instrumentCode, CURRENCY_DECIMALS, instrumentDesc)
            .frozen(false)
            .issuance(true)
            .balanceLimit(0);

        // Create a ledger account for the instrument
        const [,accountId] = await operatorClient.createAccount(account);
        currencyLedgerAccountId = accountId;

        expect(currencyLedgerAccountId).to.exist;

        /* eslint-disable no-debugger, no-console */
        console.log(`Ledger currency ${instrumentCode} created with ID: ${currencyLedgerAccountId.toString()}`);

        // Create an associated account with more public information
        const currencyLedgerAccountMetadataTxId = await operatorClient.documents(
            new DocumentBuilder()
                .insert(
                    Collection.AccountMetadata,
                    new m10.sdk.model.AccountMetadata({
                        id: currencyLedgerAccountId.toUint8Array(),
                        name: "XYZ Account",
                        publicName: "XYZ Account",
                        owner: operatorSigner.getPublicKey().toUint8Array(),
                    }),
                ),
        );

        /* eslint-disable no-debugger, no-console */
        console.log(`Account for ${instrumentCode} created with ID: ${currencyLedgerAccountId.toString()}`);

        expect(currencyLedgerAccountMetadataTxId).to.exist;

        // Add the central bank key to the Issuer role binding
        await operatorClient.documents(
            new DocumentBuilder()
                .update(
                    new DocumentUpdate(
                        Collection.RoleBinding,
                        new m10.sdk.RoleBinding({
                            id: new DocumentId("role-binding", ISSUER_ROLE_UUID).toUint8Array(),
                            subjects: [cbdcSigner.getPublicKey().toUint8Array()],
                        }),
                        ["subjects"],
                    ),
                ),
        );
    }).timeout(INCREASED_TEST_TIMEOUT);

    it("Creating bank accounts for 'XYZ' currency", async () => {
        // Normally we would find our instrument but its been created above
        const [,createdLedgerAccountId] = await operatorClient.createAccount(new AccountBuilder(currencyLedgerAccountId));

        /* eslint-disable no-debugger, no-console */
        console.log("Id of created ledger account", createdLedgerAccountId.toString());

        const createdLedgerAccountMetadataTxId = await cbdcClient.documents(
            new DocumentBuilder()
                .insert(
                    Collection.AccountMetadata,
                    new m10.sdk.model.AccountMetadata({
                        id: createdLedgerAccountId.toUint8Array(),
                        name: bankName,
                        publicName: bankName,
                        owner: cbdcSigner.getPublicKey().toUint8Array(),
                    }),
                ),
        );
        createdAccount = createdLedgerAccountId;

        /* eslint-disable no-debugger, no-console */
        console.log("Created account: ", createdAccount.toString(), " with txID: ", createdLedgerAccountMetadataTxId);

        await cbdcClient.documents(
            new DocumentBuilder()
                .update(
                    new DocumentUpdate(
                        Collection.RoleBinding,
                        new m10.sdk.RoleBinding({
                            id: new DocumentId("role-binding", SANDBOX_NOSTRO_ROLE).toUint8Array(),
                            subjects: [bankAdminSigner.getPublicKey().toUint8Array()],
                        }),
                        ["subjects"],
                    ),
                ),
        );
    }).timeout(INCREASED_TEST_TIMEOUT);

    it("Create currency transfer from the holding account to an account with a memo", async () => {
        /* eslint-disable no-debugger, no-console */
        console.log("FROM ACCOUNT: ", currencyLedgerAccountId.toString());

        /* eslint-disable no-debugger, no-console */
        console.log("TO ACCOUNT: ", createdAccount.toString());

        /* eslint-disable no-debugger, no-console */
        console.log("VALUE: ", CREATE_ISSUANCE_AMOUNT);

        const txId = await operatorClient.transfer(
            new TransferBuilder()
                .step(
                    new StepBuilder(
                        currencyLedgerAccountId,
                        createdAccount,
                        CREATE_ISSUANCE_AMOUNT,
                    )
                        .metadata(convertMemoToAny({ plaintext: "memo here" })),
                ),
        );

        /* eslint-disable no-debugger, no-console */
        console.log("TX ID: ", txId);
    }).timeout(INCREASED_TEST_TIMEOUT);

    it("Destroy currency through a transfer from the account to the holding account", async () => {
        /* eslint-disable no-debugger, no-console */
        console.log("FROM ACCOUNT: ", createdAccount.toString());

        /* eslint-disable no-debugger, no-console */
        console.log("TO ACCOUNT: ", currencyLedgerAccountId.toString());

        /* eslint-disable no-debugger, no-console */
        console.log("VALUE: ", CREATE_ISSUANCE_AMOUNT);

        const txId = await operatorClient.transfer(
            new TransferBuilder()
                .step(
                    new StepBuilder(
                        currencyLedgerAccountId,
                        createdAccount,
                        DESTROY_ISSUANCE_AMOUNT,
                    )
                        .metadata(convertMemoToAny({ plaintext: "currency destruction example" })),
                ),
        );

        /* eslint-disable no-debugger, no-console */
        console.log("TX ID: ", txId);
    }).timeout(INCREASED_TEST_TIMEOUT);

    it("Create bank object from bank accounts", async () => {
        /* eslint-disable no-debugger, no-console */
        console.log("CREATING BANK");

        const bankCreationTxId = await operatorClient.documents(
            new DocumentBuilder()
                .insert(
                    Collection.Bank,
                    new m10.sdk.model.Bank({
                        id: new DocumentId("bank").toUint8Array(),
                        owner: createdAccount.toUint8Array(),
                        shortName: "BNKX",
                        displayName: "XYZ Bank",
                        accounts: [
                            new m10.sdk.model.BankAccountRef({
                                accountId: createdAccount.toUint8Array(),
                                accountType: m10.sdk.model.BankAccountRef.BankAccountType.CBDC,
                            }),
                        ],
                    }),
                ),
        );

        /* eslint-disable no-debugger, no-console */
        console.log("TX ID: ", bankCreationTxId);
    }).timeout(INCREASED_TEST_TIMEOUT);

});
