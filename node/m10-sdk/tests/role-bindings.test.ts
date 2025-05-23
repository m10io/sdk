import { assert } from "chai";

import { M10Client, CryptoSigner } from "../src";
import type { ListRoleBindingsRequest } from "../src/protobufs/sdk/api";

const BANK_ADMIN = "MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK";
const LEDGER_URL = process.env.LEDGER_URL || "https://app.dev.m10.net";


describe("role-bindings", () => {

    describe("query", () => {

        it("listRoleBindings", async () => {

            const bankAdminSigner = await CryptoSigner.fromPkcs8Pem(BANK_ADMIN);
            const ledgerClient = new M10Client(LEDGER_URL, bankAdminSigner);

            const request: ListRoleBindingsRequest = {
                filter: {
                    name: "node-test-customer",
                    oneofKind: "name",
                },
            };

            const response = await ledgerClient.listRoleBindings(request);

            assert.isNotEmpty(response.roleBindings);
        });
    });
});
