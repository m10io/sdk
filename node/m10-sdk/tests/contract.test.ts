import { assert } from "chai";

import { AccountId, ContractBuilder } from "../src";


describe("contract", () => {

    describe("transaction", () => {

        it("builder", async () => {

            const FIRST_TRANSFER_AMOUNT: number = 4212;
            const SECOND_TRANSFER_AMOUNT: number = 1242;
            const AMOUNT_OF_TRANSFERS: number = 2;

            const FIRST_ACCOUNT_ID = AccountId.fromHex("04800003000000009400000000000001");
            const SECOND_ACCOUNT_ID = AccountId.fromHex("04800003000000009400000000000002");
            const THIRD_ACCOUNT_ID = AccountId.fromHex("04800003000000009400000000000003");

            const contract = ContractBuilder.default()
                .transfer(
                    "usd.m10",
                    FIRST_ACCOUNT_ID.bytes,
                    SECOND_ACCOUNT_ID.bytes,
                    FIRST_TRANSFER_AMOUNT,
                    "my test transfer on USD",
                )
                .transfer(
                    "cad.m10",
                    SECOND_ACCOUNT_ID.bytes,
                    THIRD_ACCOUNT_ID.bytes,
                    SECOND_TRANSFER_AMOUNT,
                    "my test transfer on CAD",
                )
                .build();

            assert.isNotEmpty(contract.id(), "Contract ID is empty");
            assert.isOk(contract.transferInfo().length === AMOUNT_OF_TRANSFERS, "Should have 2 transfers");
        });
    });
});
