import { assert } from "chai";

import type { m10 } from "../protobufs";
import { LedgerClient } from "../src/ledger_client";
import { CryptoSigner } from "../src/utils";

const BANK_ADMIN = "MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK";
const LEDGER_URL = process.env.LEDGER_URL || "develop.m10.net";


describe("role-bindings", () => {

    describe("query", () => {

        it("listRoleBindings", async () => {

            const bankAdminSigner = CryptoSigner.getSignerFromPkcs8V1(BANK_ADMIN);
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
