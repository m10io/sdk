import { assert } from "chai";

import type { m10 } from "../protobufs";
import { AccountFilter } from "../src";
import { ActionBuilder,StepBuilder, TransferBuilder } from "../src/builders";
import { M10Error } from "../src/error";
import { unwrap } from "../src/utils";

import type { TestContext } from "./hooks";


// Too many async calls to finish under default timeout
const INCREASED_TEST_TIMEOUT: number = 6000;

describe("observations", () => {

    describe("stream", () => {

        it("observe transfer", function(this: TestContext, done: Mocha.Done) {

            const context = unwrap(this.context, M10Error.Other("TestSuiteContext is None"));

            const TRANSFER_CREATION_INTERVAL_MS: number = 1000;
            const REQUESTS_SENT_COUNT: number = 2;

            let transfersSent: number = 0;
            let transfersReceived: number = 0;

            const [service, startObserver] = context.client.observeTransfers(
                new AccountFilter().involves(context.parentAccountId),
            );

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

                const TRANSFER_AMOUNT = 1;

                await context.client.transfer(
                    new TransferBuilder()
                        .step(
                            new StepBuilder(
                                context.parentAccountId,
                                context.bobsAccountId,
                                TRANSFER_AMOUNT,
                            ),
                        ),
                );

                // Increment number of requests sent
                transfersSent++;
            }, TRANSFER_CREATION_INTERVAL_MS);
        })
            .timeout(INCREASED_TEST_TIMEOUT);

        it("observe actions", function(this: TestContext, done: Mocha.Done) {
            const context = unwrap(this.context, M10Error.Other("TestSuiteContext is None"));

            const ACTIONS_CREATION_INTERVAL_MS: number = 1000;
            const REQUESTS_SENT_COUNT: number = 2;

            let actionsSent: number = 0;
            let actionsReceived: number = 0;

            const [service, startObserver] = context.client.observeActions(
                new AccountFilter().involves(context.bobsAccountId).name("test.action"),
            );

            service.on("data", (response: m10.sdk.IFinalizedTransactions) => {
                assert.isAbove((response.transactions || []).length, 0);
                actionsReceived++;
            });

            startObserver();
            // -----------------------------------------------------------------
            const intervalId = setInterval(async () => {
                // Clear interval after 2 requests are sent
                if (actionsSent === REQUESTS_SENT_COUNT) {
                    clearInterval(intervalId);
                    service.end(false);
                    // Validate that all requests were observed
                    assert.equal(actionsReceived, REQUESTS_SENT_COUNT);
                    done();
                    return;
                }

                await context.client.action(
                    ActionBuilder.forAccount(
                        "test.action",
                        context.parentAccountId,
                        context.bobsAccountId),
                );
                // Increment number of requests sent
                actionsSent++;
            }, ACTIONS_CREATION_INTERVAL_MS);
        }).timeout(INCREASED_TEST_TIMEOUT);
    });
});

