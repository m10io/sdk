import { assert } from "chai";

import { ContractBuilder } from "../src/contract/builder";
import { getUint8ArrayFromAccountId } from "../src/utils/account_id";


describe("transaction", () => {

    describe("contract", () => {

        it("builder", async () => {

            const FIRST_TRANSFER_AMOUNT: number = 4212;
            const SECOND_TRANSFER_AMOUNT: number = 1242;
            const AMOUNT_OF_TRANSFERS: number = 2;

            const contract = ContractBuilder.default()
                .transfer(
                    "usd.m10",
                    getUint8ArrayFromAccountId("04800003000000009400000000000001"),
                    getUint8ArrayFromAccountId("04800003000000009400000000000002"),
                    FIRST_TRANSFER_AMOUNT,
                    "my test transfer on USD",
                )
                .transfer(
                    "cad.m10",
                    getUint8ArrayFromAccountId("04800003000000009400000000000002"),
                    getUint8ArrayFromAccountId("04800003000000009400000000000003"),
                    SECOND_TRANSFER_AMOUNT,
                    "my test transfer on CAD",
                )
                .build();

            assert.isNotEmpty(contract.id(), "Contract ID is empty");
            assert.isOk(contract.transferInfo().length === AMOUNT_OF_TRANSFERS, "Should have 2 transfers");
        });
    });
});
