import { assert, expect } from "chai";

import { m10 } from "../protobufs";
import { convertMemoToAny, getUint8ArrayFromAccountId, isSome, unwrap } from "../src/utils";

import type { TestContext, TestSuiteContext } from "./hooks";



describe("transaction", () => {

    describe("transfer", () => {

        let transactionId: Option<number | Long.Long>;

        it("should create a transfer", async function (this: TestContext) {

            const context = unwrap<TestSuiteContext>(this.context, "TestSuiteContext");

            // Create transfer

            const transferStep: m10.sdk.transaction.ITransferStep = {
                fromAccountId: getUint8ArrayFromAccountId(context.parentAccountId),
                toAccountId: getUint8ArrayFromAccountId(context.bobsAccountId),
                amount: 1000,
                metadata: [ convertMemoToAny(new m10.sdk.metadata.Memo({ plaintext: "Funds" })) ],
            };
            const transfer = new m10.sdk.transaction.CreateTransfer({ transferSteps: [ transferStep ] });

            const transactionData = new m10.sdk.transaction.TransactionData({ transfer });
            const transactionRequestPayload = context.ledgerClient.transactionRequest(transactionData);

            const transactionResponse = await context.ledgerClient
                .createTransaction(context.bankAdminSigner, transactionRequestPayload);

            transactionId = transactionResponse.txId;
            assert.isOk(isSome(transactionId));

            // Get transfer

            const transferPayload = new m10.sdk.transaction.GetTransferRequest({ txId: transactionId });

            const transferResponse = await context.ledgerClient.getTransfer(context.bankAdminSigner, transferPayload);
            assert.isOk(!isSome(transferResponse.error));
        });

        it("should get an enhanced transfer by id", async function (this: TestContext) {

            const context = unwrap<TestSuiteContext>(this.context, "TestSuiteContext");

            // Get transfer

            const transferPayload = new m10.sdk.transaction.GetTransferRequest({ txId: transactionId });

            const transferResponse = await context.ledgerClient.getTransfer(context.bankAdminSigner, transferPayload);
            const enhancedTransfer = await context.ledgerClient.enhanceTransfer(context.bankAdminSigner, transferResponse);

            const enhancedTransferStep = enhancedTransfer.enhanced_steps[0];

            expect(enhancedTransferStep.from).to.exist;
            expect(enhancedTransferStep.to).to.exist;
            expect(enhancedTransferStep.from_bank).to.exist;
            expect(enhancedTransferStep.to_bank).to.exist;
        });


        it("should get a list of transfers", async function (this: TestContext) {

            const context = unwrap<TestSuiteContext>(this.context, "TestSuiteContext");

            // List transfers

            const listTransferPayload = new m10.sdk.transaction.ListTransferRequest({
                accountId: getUint8ArrayFromAccountId(context.bobsAccountId),
                limit: 10,
            });

            const listTransferResponse = await context.ledgerClient
                .listTransfers(context.bankAdminSigner, listTransferPayload);

            const transfers = unwrap(listTransferResponse.transfers, "listTransferResponse.transfers");

            const enhancedTransfers = await context.ledgerClient.enhanceTransfers(context.bankAdminSigner, transfers);

            expect(enhancedTransfers).to.be.instanceOf(Array);
            expect(enhancedTransfers?.length).to.be.greaterThan(0);

            const enhancedTransfer = unwrap(enhancedTransfers[0], "enhancedTransfers[0]");

            expect(enhancedTransfer.transfer.transferSteps[0].amount).to.exist;
            expect(enhancedTransfer.transfer.transferSteps[0].fromAccountId).to.exist;
            expect(enhancedTransfer.transfer.transferSteps[0].toAccountId).to.exist;

            expect(enhancedTransfer.enhanced_steps[0].from).to.exist;
            expect(enhancedTransfer.enhanced_steps[0].to).to.exist;
            expect(enhancedTransfer.enhanced_steps[0].from_bank).to.exist;
            expect(enhancedTransfer.enhanced_steps[0].to_bank).to.exist;
        });
    });
});
