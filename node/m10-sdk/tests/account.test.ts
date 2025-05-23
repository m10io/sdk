import { assert } from "chai";

import type { TestCaseInstances } from "./config";
import { createCurrencyAccounts, initTestCaseInstances } from "./config";

describe("account", () => {
    let testCaseInstances: TestCaseInstances;

    it("init", async () => {
        testCaseInstances = await initTestCaseInstances();
    });

    it("create currency accounts", async () => {
        await createCurrencyAccounts(testCaseInstances);
    // eslint-disable-next-line @typescript-eslint/no-magic-numbers
    }).timeout(10_000);

    describe("query", () => {
        it("listAccounts", async () => {
            const accountMetadatas = await testCaseInstances.accountClient.listAccountMetadata(
                {
                    filter: {
                        oneofKind: "owner",
                        owner: testCaseInstances.accountSigner.getPublicKey().toUint8Array(),
                    },
                },
            );

            assert.isNotEmpty(accountMetadatas);
        });
    });
});
