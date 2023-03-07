import { assert } from "chai";

import { LedgerClient } from "../src/ledger_client";


const LEDGER_URL = process.env.LEDGER_URL || "develop.m10.net";

describe("transaction", () => {

    describe("query", () => {

        it("blockHeight", async () => {

            const ledgerClient = new LedgerClient(LEDGER_URL, true);
            const response = await ledgerClient.blockHeight();

            assert.isOk(response > 0);
        });
    });
});
