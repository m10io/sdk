/* eslint-disable @typescript-eslint/no-magic-numbers */
/* eslint-disable no-console */
import { assert, expect } from "chai";

import type { AccountId } from "../src";
import {
    convertMemoToAny,
} from "../src";

import { parseUnits, sleep } from "./hooks";

import { Memo } from "../src/protobufs/sdk/metadata";
import type { TestCaseInstances } from "./config";
import { createCurrencyAccounts, initTestCaseInstances, USD_BANK_ID } from "./config";
import { Rule_Verb } from "../src/protobufs/sdk/rbac";

describe("transfer", () => {
    const transactionIds: bigint[] = [];

    let testCaseInstances: TestCaseInstances;

    it("init", async () => {
        testCaseInstances = await initTestCaseInstances();
    });

    let accountIds: AccountId[];
    it("create USD currency accounts", async () => {
        const currencyAccounts = await createCurrencyAccounts(
            testCaseInstances,
            [USD_BANK_ID, USD_BANK_ID],
            [
                Rule_Verb.COMMIT,
                Rule_Verb.INITIATE,
                Rule_Verb.TRANSACT,
            ],
        );
        accountIds = Object.values(currencyAccounts.accountIds);
    }).timeout(10_000);

    it("fund currency accounts", async () => {
        if (!accountIds.length) throw new TypeError("Account IDs are not initialized");

        await Promise.all([USD_BANK_ID, USD_BANK_ID].map(async (bankId, idx) => {
            const bankInfo = await testCaseInstances.accountClient.getAccountInfo(bankId);

            await testCaseInstances.operatorClient.transfer([
                {
                    fromAccountId: bankId.bytes,
                    toAccountId: accountIds[idx].bytes,
                    // FIXME: parse it twice, cuz of backend divides bn amount by decimals
                    amount: parseUnits(parseUnits(10, bankInfo.decimalPlaces), bankInfo.decimalPlaces),
                },
            ]);
        }));
    });

    describe("transaction", () => {

        it("should create a transfer", async function() {
            const txIds = await Promise.all(accountIds.map(async (accountId) => {
                const to = accountIds.find((id) => {
                    return id.hex !== accountId.hex;
                });

                if (!to) {
                    throw new TypeError("to is undefined");
                }

                return testCaseInstances.accountClient.transfer([
                    {
                        fromAccountId: accountId.bytes,
                        toAccountId: to.bytes,
                        amount: parseUnits(parseUnits(10, 2), 2),
                    },
                ]);
            }));

            transactionIds.push(...txIds);
            expect(txIds.length).to.be.greaterThan(0);
        });

        it("should initiate and commit a transfer", async function() {
            await Promise.all(accountIds.map(async (accountId) => {
                const to = accountIds.find((id) => {
                    return id.hex !== accountId.hex;
                });

                if (!to) {
                    throw new TypeError("to is undefined");
                }

                const txId = await testCaseInstances.accountClient.initiateTransfer([
                    {
                        fromAccountId: accountId.bytes,
                        toAccountId: to.bytes,
                        amount: parseUnits(parseUnits(10, 2), 2),
                        metadata: [convertMemoToAny(Memo.create({ plaintext: "Funds" }))],
                    },
                ]);

                console.log("Initiated transfer with txId:", txId);

                if (!txId) {
                    throw new TypeError("transactionId is undefined");
                }

                try {
                    const responseTxId = await testCaseInstances.accountClient.commitTransfer(txId, true);

                    await sleep(1_000);

                    transactionIds.push(responseTxId);

                    console.log("responseTxId", responseTxId);

                    const transferTx = await testCaseInstances.accountClient.getTransaction({ txId: responseTxId });

                    assert.isOk(Boolean(transferTx.response?.transferCommitted?.transferSteps.length));
                } catch (error) {
                    console.error("Error while waiting for transfer to be accepted", error);
                }
            }));
        }).timeout(10_000);
    });

    describe("query", () => {
        it("should get a list of transfers", async function() {
            const limit = 10;
            await Promise.all(accountIds.map(async (accountId) => {
                const enhancedTransfers = await testCaseInstances.accountClient.getEnhancedTransfers(
                    {
                        filter: {
                            oneofKind: "accountId",
                            accountId: accountId.bytes,
                        },
                        limit: BigInt(limit),
                    },
                );

                expect(enhancedTransfers).to.be.instanceOf(Array);
                expect(enhancedTransfers?.length).to.be.greaterThan(0);

                const enhancedTransfer = enhancedTransfers[0];

                expect(enhancedTransfer.enhanced_steps[0].from?.accountId).to.exist;
                expect(enhancedTransfer.enhanced_steps[0].to?.accountId).to.exist;
            }));
        });
    });
});
