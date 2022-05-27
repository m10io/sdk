import { assert } from "chai";

import { m10 } from "../protobufs";
import { getUint8ArrayFromAccountId, unwrap } from "../src/utils";

import type { TestContext, TestSuiteContext } from "./hooks";


// Too many async calls to finish under default timeout
const INCREASED_TEST_TIMEOUT: number = 6000;

describe("stream", () => {

    describe("observations", () => {

        it("observe transfer", function (this: TestContext, done: Mocha.Done) {

            const context = unwrap<TestSuiteContext>(this.context, "TestSuiteContext");

            const TRANSFER_CREATION_INTERVAL_MS: number = 1000;
            const REQUESTS_SENT_COUNT: number = 2;

            let transfersSent: number = 0;
            let transfersReceived: number = 0;

            const [ service, startObserver ] = context.ledgerClient.getObserveTransfers(context.bankAdminSigner, {
                involvedAccounts: [ getUint8ArrayFromAccountId(context.parentAccountId) ],
            });

            service.on("data", (response: m10.sdk.IFinalizedTransactions) => {
                assert.isAbove((response.transactions || []).length, 0);
                transfersReceived++;
            });

            startObserver();

            // -----------------------------------------------------------------

            const intervalId = setInterval(async () => {

                // Clear interval after 2 requests are sent
                if (transfersSent === REQUESTS_SENT_COUNT) {
                    clearInterval(intervalId);

                    service.end(false);

                    // Validate that all requests were observed
                    assert.equal(transfersReceived, REQUESTS_SENT_COUNT);
                    done();

                    return;
                }

                const transferStep: m10.sdk.transaction.ITransferStep = {
                    fromAccountId: getUint8ArrayFromAccountId(context.parentAccountId),
                    toAccountId: getUint8ArrayFromAccountId(context.bobsAccountId),
                    amount: 1,
                };

                const transfer = new m10.sdk.transaction.CreateTransfer({
                    transferSteps: [ transferStep ],
                });

                const transactionData = new m10.sdk.transaction.TransactionData({ transfer });
                const transactionRequestPayload = context.ledgerClient.transactionRequest(transactionData);

                await context.ledgerClient.createTransaction(context.bankAdminSigner, transactionRequestPayload);

                // Increment number of requests sent
                transfersSent++;
            }, TRANSFER_CREATION_INTERVAL_MS);
        })
            .timeout(INCREASED_TEST_TIMEOUT);
    });
});

