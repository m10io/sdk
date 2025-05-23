/* eslint-disable @typescript-eslint/no-magic-numbers */
/* eslint-disable no-console */
import "./hooks";

import type {
    AccountId } from "../src";
import { Target } from "../src/protobufs/sdk/transaction/transaction";
import { parseUnits } from "./hooks";
import type { TestCaseInstances } from "./config";
import { initTestCaseInstances, createCurrencyAccounts, USD_BANK_ID, EUR_BANK_ID } from "./config";

// Too many async calls to finish under default timeout
const INCREASED_TEST_TIMEOUT: number = 6000;

describe("observations", () => {
    let testCaseInstances: TestCaseInstances;

    it("init", async () => {
        testCaseInstances = await initTestCaseInstances();
    });

    let accountIds: Record<string, AccountId> = {};
    it("create USD/EUR/TOKEN currency accounts", async () => {
        const currencyAccounts = await createCurrencyAccounts(testCaseInstances);
        accountIds = currencyAccounts.accountIds;
    }).timeout(10_000);

    it("fund currency accounts", async () => {
        if (!Object.values(accountIds).length) throw new TypeError("Account IDs are not initialized");

        await Promise.all([USD_BANK_ID, EUR_BANK_ID].map(async (bankId) => {
            const bankInfo = await testCaseInstances.accountClient.getAccountInfo(bankId);

            await testCaseInstances.operatorClient.transfer([
                {
                    fromAccountId: bankId.bytes,
                    toAccountId: accountIds[bankId.hex].bytes,
                    amount: parseUnits(parseUnits(10, bankInfo.decimalPlaces), bankInfo.decimalPlaces),
                },
            ]);
        }));
    });

    describe("stream", () => {

        it("observe transfer", async function() {
            await Promise.all(Object.values(accountIds).map(async (accountId) => {
                const abortController = new AbortController();

                const startObserve = await testCaseInstances.accountClient.observeTransfers(
                    {
                        involvedAccounts: [accountId.bytes],
                    },
                    {
                        abort: abortController.signal,
                    },
                );

                const service = startObserve();

                return new Promise((resolve, reject) => {
                    (async ()=>{
                        service.responses.onNext((message, error) => {
                            if (error) {
                                abortController.abort();
                                reject(new Error("Error in transfer observation: " + error));
                                return;
                            }
                            if (message) {
                                console.log("Transfer observation: ", message);
                                abortController.abort();
                                resolve(true);
                            }
                        });

                        const to = Object.values(accountIds).find((id) => {
                            return id.hex !== accountId.hex;
                        });

                        if (!to) {
                            abortController.abort();
                            reject(new Error("No other account found"));
                            return;
                        }

                        const txId = await testCaseInstances.accountClient.transfer([
                            {
                                fromAccountId: accountId.bytes,
                                toAccountId: to.bytes,
                                amount: parseUnits(parseUnits(10, 2), 2),
                            },
                        ]);

                        console.log("Transfer sent, txId: ", txId);
                    })();
                });
            }));
        }).timeout(INCREASED_TEST_TIMEOUT);

        it("observe actions", async function() {
            await Promise.all(Object.values(accountIds).map(async (accountId) => {
                const abortController = new AbortController();

                const startObserve = await testCaseInstances.accountClient.observeActions(
                    {
                        involvesAccounts: [accountId.bytes],
                        name: "test.action",
                    },
                    {
                        abort: abortController.signal,
                    },
                );

                const service = startObserve();

                return new Promise((resolve, reject) => {
                    (async ()=>{
                        service.responses.onNext((message, error) => {
                            if (error) {
                                abortController.abort();
                                reject(new Error("Error in transfer observation: " + error));
                                return;
                            }
                            if (message) {
                                console.log("Action observation: ", message.transactions);
                                abortController.abort();
                                resolve(true);
                            }
                        });

                        const to = Object.values(accountIds).find((id) => id.hex !== accountId.hex);

                        if (!to) {
                            abortController.abort();
                            reject(new Error("No other account found"));
                            return;
                        }

                        const txId = await testCaseInstances.accountClient.action({
                            name: "test.action",
                            fromAccount: accountId.bytes,
                            target: Target.create({
                                target: {
                                    oneofKind: "accountId",
                                    accountId: to.bytes,
                                },
                            }),
                        });

                        console.log("Action sent, txId: ", txId);
                    })();
                });
            }));
        }).timeout(INCREASED_TEST_TIMEOUT);
    });
});

