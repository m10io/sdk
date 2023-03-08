import { assert } from "chai";

import { M10Client } from "../src";
import { PageBuilder } from "../src/builders/page";
import { CryptoSigner } from "../src/utils";

const BANK_ADMIN = "MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK";
const LEDGER_URL = process.env.LEDGER_URL || "develop.m10.net";


describe("account", () => {

    describe("query", () => {

        it("listAccounts", async () => {

            const bankAdminSigner = CryptoSigner.getSignerFromPkcs8V1(BANK_ADMIN);
            const client = new M10Client(LEDGER_URL, bankAdminSigner);

            const accountMetadatas = await client.listAccountMetadatas(
                PageBuilder.byOwner(bankAdminSigner.getPublicKey()),
            );

            assert.isNotEmpty(accountMetadatas);
        });
    });
});
