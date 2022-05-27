import { expect } from "chai";
import { beforeEach,describe, it } from "mocha";

import type { m10 } from "../../protobufs";
import { LedgerClient } from "../../src/client";
import { accounts, roleBindings } from "../../src/helpers";
import { convertPkcs8V2KeyToV1, CryptoSigner,getPrivateKey } from "../../src/utils";

const INCREASED_TEST_TIMEOUT: number = 6000;

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

    const BANK_ADMIN_KEY = getPrivateKey("MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK");
    const OPERATOR_KEY = getPrivateKey(
        convertPkcs8V2KeyToV1("MFMCAQEwBQYDK2VwBCIEIMA9e6xaIFATarh2S5yVEe7jene/36EOcM+3B/sOH0y/oSMDIQDA9/WOLKaVoXq92lOSjloExzLXCdRz8oW9TweVGAGqZg=="),
    );
    const CBDC_KEY = getPrivateKey(
        convertPkcs8V2KeyToV1("MFMCAQEwBQYDK2VwBCIEIPx4/t2ofbHbOzNFDHIppkxFmdcOwCrb2wihczFB23sUoSMDIQA5vchjxwfcRTl7c1OBpIoQ47ah5iMaebXWO8ASVZzrJQ=="),
    );

    // these are generated out of the box
    const ISSUER_ROLE_UUID = "77436769-8bf9-4581-b0cb-e62b66d5d9a9";
    const SANDBOX_NOSTRO_ROLE = "0c8a1d5f-ec36-4342-93ca-0893103e9b60";

    const LEDGER_URL = "develop.m10.net";

    beforeEach(async () => {
        ledgerClient = new LedgerClient(LEDGER_URL, true);
        bankAdminSigner = new CryptoSigner(BANK_ADMIN_KEY);
        cbdcSigner = new CryptoSigner(CBDC_KEY);
        operatorSigner = new CryptoSigner(OPERATOR_KEY);
        m1BankName = "Test M1 Bank";
        instrumentCode = "XYZ";
        instrumentDesc = "XYZ Currency";
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

    it("Creating m1 bank accounts for XYZ currency", async () => {
        // Normally we would find our instrument but its been created above
        const createdLedgerAccount = await accounts.createLedgerAccount(ledgerClient, operatorSigner, currencyLedgerAccountId);

        /* eslint-disable no-debugger, no-console */
        console.log("Id of created ledger account", createdLedgerAccount);

        const createdAccount = await accounts.createAccount(ledgerClient, cbdcSigner, createdLedgerAccount, m1BankName);

        /* eslint-disable no-debugger, no-console */
        console.log("Created account:", createdAccount);

        await roleBindings.updateRoleBinding(ledgerClient, cbdcSigner, SANDBOX_NOSTRO_ROLE, [ Uint8Array.from(bankAdminSigner.getPublicKey()) ]);

    }).timeout(INCREASED_TEST_TIMEOUT);



});
