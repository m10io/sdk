import { assert } from "chai";

import type { m10 } from "../protobufs";
import { LedgerClient } from "../src/client";
import { CryptoSigner, getPrivateKey } from "../src/utils";

const BANK_ADMIN = "MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK";
const LEDGER_URL = process.env.LEDGER_URL || "develop.m10.net";


describe("query", () => {

    describe("role-bindings", () => {

        it("listRoleBindings", async () => {

            const bankAdminSigner = new CryptoSigner(getPrivateKey(BANK_ADMIN));
            const ledgerClient = new LedgerClient(LEDGER_URL, true);

            const request: m10.sdk.IListRoleBindingsRequest = {
                name: "node-test-customer",
            };

            const response: m10.sdk.IListRoleBindingsResponse = await ledgerClient
                .listRoleBindings(bankAdminSigner, request);

            assert.isNotEmpty(response.roleBindings);
        });
    });
});
