import { assert, expect } from "chai";

import { m10 } from "../protobufs";
import { StepBuilder, TransferBuilder, TransferFilter } from "../src";
import { M10Error } from "../src/error";
import { TransferStatus } from "../src/types";
import { convertMemoToAny, isSome, unwrap } from "../src/utils";

import type { TestContext } from "./hooks";



describe("transfer", () => {

    let transactionId: Option<LongNumber>;

    describe("transaction", () => {

        it("should create a transfer", async function(this: TestContext) {

            const context = unwrap(this.context, M10Error.Other("TestSuiteContext is None"));

            // Create transfer

            const transferAmount = 1000;

            transactionId = await context.client.transfer(
                new TransferBuilder().step(
                    new StepBuilder(
                        context.parentAccountId,
                        context.bobsAccountId,
                        transferAmount,
                    ).metadata(convertMemoToAny(new m10.sdk.metadata.Memo({ plaintext: "Funds" }))),
                ),
            );

            assert.isOk(isSome(transactionId));

            // Get transfer

            const transfer = await context.client.getTransfer(transactionId);

            assert.isOk(transfer.success);
        });

        it("should initiate and commit a transfer", async function(this: TestContext) {

            const context = unwrap(this.context, M10Error.Other("TestSuiteContext is None"));

            // Initiate transfer


            const transferAmount = 1000;

            transactionId = await context.client.initiateTransfer(
                new TransferBuilder().step(
                    new StepBuilder(
                        context.parentAccountId,
                        context.bobsAccountId,
                        transferAmount,
                    ).metadata(convertMemoToAny(new m10.sdk.metadata.Memo({ plaintext: "Funds" }))),
                ),
            );

            // Commit transfer

            if (isSome(transactionId)) {

                const responseTxId = await context.client.commitTransfer(transactionId, true);

                assert.isOk(isSome(responseTxId), "should successfully commit a pending transfer");
            }
            else {
                assert.isOk(false, "transactionId is undefined");
            }

            // Get transfer

            const transfer = await context.client.getTransfer(transactionId);

            assert.isOk(transfer.success);
            assert.equal(transfer.status, TransferStatus.Accepted);
        });
    });

    describe("query", () => {

        it("should get an enhanced transfer by id", async function(this: TestContext) {

            const context = unwrap(this.context, M10Error.Other("TestSuiteContext is None"));

            // Get transfer
            const enhancedTransfer = await context.client.getEnhancedTransfer(
                unwrap(transactionId, M10Error.Other("transactionId is None")),
            );

            const enhancedTransferStep = enhancedTransfer.steps[0];

            expect(enhancedTransferStep.from.id).to.exist;
            expect(enhancedTransferStep.to.id).to.exist;
            expect(enhancedTransferStep.amount).to.exist;
        });

        it("should get a list of transfers", async function(this: TestContext) {

            const context = unwrap(this.context, M10Error.Other("TestSuiteContext is None"));

            // List transfers

            const limit = 10;
            const enhancedTransfers = await context.client.getEnhancedTransfers(
                TransferFilter.byAccount(context.bobsAccountId).limit(limit),
            );

            expect(enhancedTransfers).to.be.instanceOf(Array);
            expect(enhancedTransfers?.length).to.be.greaterThan(0);

            const enhancedTransfer = unwrap(enhancedTransfers[0], M10Error.Other("enhancedTransfers[0] is None"));

            expect(enhancedTransfer.steps[0].amount).to.exist;
            expect(enhancedTransfer.steps[0].from.id).to.exist;
            expect(enhancedTransfer.steps[0].to.id).to.exist;
        });
    });
});
