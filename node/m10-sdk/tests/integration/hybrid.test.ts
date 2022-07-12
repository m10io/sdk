import { expect } from "chai";
import { beforeEach,describe, it } from "mocha";

import { m10 } from "../../protobufs";
import { LedgerClient } from "../../src/client";
import { accounts, roleBindings, transfers } from "../../src/helpers";
import { CryptoSigner, getPublicKeyFromUint8Array, getUint8ArrayFromAccountId } from "../../src/utils";

const INCREASED_TEST_TIMEOUT: number = 12000;

describe("Hybrid CBDC Bootstrap", () => {
    let operatorSigner: CryptoSigner;
    let bankAdminSigner: CryptoSigner;
    let cbdcSigner: CryptoSigner;

    let ledgerClient: LedgerClient;
    let instrument: m10.sdk.transaction.ISetInstrument;
    let currencyLedgerAccountId: string;

    let m1BankName: string;
    let instrumentCode: string;
    let instrumentDesc: string;
    let createdM1Account: string = "";

    const CREATE_M1_AMOUNT: number = 10000;

    const BANK_ADMIN_KEY = "MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK";
    const OPERATOR_KEY = "MFMCAQEwBQYDK2VwBCIEIMA9e6xaIFATarh2S5yVEe7jene/36EOcM+3B/sOH0y/oSMDIQDA9/WOLKaVoXq92lOSjloExzLXCdRz8oW9TweVGAGqZg==";
    const CBDC_KEY = "MFMCAQEwBQYDK2VwBCIEIPx4/t2ofbHbOzNFDHIppkxFmdcOwCrb2wihczFB23sUoSMDIQA5vchjxwfcRTl7c1OBpIoQ47ah5iMaebXWO8ASVZzrJQ==";

    // these are generated out of the box
    const ISSUER_ROLE_UUID = "77436769-8bf9-4581-b0cb-e62b66d5d9a9";
    const SANDBOX_NOSTRO_ROLE = "0c8a1d5f-ec36-4342-93ca-0893103e9b60";

    const LEDGER_URL = process.env.LEDGER_URL || "develop.m10.net";

    beforeEach(async () => {
        ledgerClient = new LedgerClient(LEDGER_URL, true);
        bankAdminSigner = CryptoSigner.getSignerFromPkcs8V1(BANK_ADMIN_KEY);
        cbdcSigner = CryptoSigner.getSignerFromPkcs8V2(CBDC_KEY);
        operatorSigner = CryptoSigner.getSignerFromPkcs8V2(OPERATOR_KEY);
        m1BankName = "Test M1 Bank";
        instrumentCode = "XYZ";
        instrumentDesc = "XYZ Currency";

        /* eslint-disable no-debugger, no-console */
        console.log("Operator key account", getPublicKeyFromUint8Array(operatorSigner.getPublicKey()));
    });

    it("Create m0 currency", async () => {
        instrument = {
            code: instrumentCode,
            decimalPlaces: 2,
            description: instrumentDesc,
        };

        // Create a ledger account for the instrument
        currencyLedgerAccountId = await accounts.createLedgerAccount(ledgerClient, operatorSigner, undefined, instrument, false, true);

        expect(currencyLedgerAccountId).to.exist;

        /* eslint-disable no-debugger, no-console */
        console.log(`Ledger currency ${instrumentCode} created with ID: ${currencyLedgerAccountId}`);

        // Create an associated account with more public information
        const currencyAccountId = await accounts.createAccount(ledgerClient, operatorSigner, currencyLedgerAccountId, "XYZ Account");

        /* eslint-disable no-debugger, no-console */
        console.log(`Account for ${instrumentCode} created with ID: ${currencyAccountId}`);

        expect(currencyAccountId).to.exist;

        // Add the central bank key to the Issuer role binding
        await roleBindings.updateRoleBinding(ledgerClient, operatorSigner, ISSUER_ROLE_UUID, [ Uint8Array.from(cbdcSigner.getPublicKey()) ]);
    }).timeout(INCREASED_TEST_TIMEOUT);

    it("Creating m1 bank accounts for 'XYZ' currency", async () => {
        // Normally we would find our instrument but its been created above
        const createdLedgerAccount = await accounts.createLedgerAccount(ledgerClient, operatorSigner, currencyLedgerAccountId);

        /* eslint-disable no-debugger, no-console */
        console.log("Id of created ledger account", createdLedgerAccount);

        createdM1Account = await accounts.createAccount(ledgerClient, cbdcSigner, createdLedgerAccount, m1BankName);

        /* eslint-disable no-debugger, no-console */
        console.log("Created account:", createdM1Account);

        await roleBindings.updateRoleBinding(ledgerClient, cbdcSigner, SANDBOX_NOSTRO_ROLE, [ Uint8Array.from(bankAdminSigner.getPublicKey()) ]);

    }).timeout(INCREASED_TEST_TIMEOUT);

    it("Create m0 currency transfer from the m0 holding account to an m1 account with a memo", async () => {
        /* eslint-disable no-debugger, no-console */
        console.log("FROM ACCOUNT: ", currencyLedgerAccountId);

        /* eslint-disable no-debugger, no-console */
        console.log("TO ACCOUNT: ", createdM1Account);

        /* eslint-disable no-debugger, no-console */
        console.log("VALUE: ", CREATE_M1_AMOUNT);

        const tx = await transfers.createTransfer(ledgerClient, currencyLedgerAccountId, createdM1Account, CREATE_M1_AMOUNT, "memo here", operatorSigner);

        /* eslint-disable no-debugger, no-console */
        console.log("TX ID: ", tx.txId);
    }).timeout(INCREASED_TEST_TIMEOUT);

    it("Create bank object from m1 bank accounts", async () => {
        /* eslint-disable no-debugger, no-console */
        console.log("CREATING BANK");

        const bankAccounts: m10.sdk.model.IBankAccountRef[] = [ {
            accountId: getUint8ArrayFromAccountId(createdM1Account),
            accountType: m10.sdk.model.BankAccountRef.BankAccountType.CBDC,
        } ];

        const bank = await accounts.createBank(ledgerClient, operatorSigner, createdM1Account, "BNKX", "XYZ Bank", bankAccounts);

        /* eslint-disable no-debugger, no-console */
        console.log("TX ID: ", bank);
    }).timeout(INCREASED_TEST_TIMEOUT);

});
