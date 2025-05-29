/* eslint-disable @typescript-eslint/no-magic-numbers */
import { assert, expect } from "chai";
import { TextDecoder, TextEncoder } from "util";

import { AccountId } from "../src";

import { Target } from "../src/protobufs/sdk/transaction/transaction";
import type { TestCaseInstances } from "./config";
import { initTestCaseInstances, createCurrencyAccounts } from "./config";

describe("action", () => {
    let testCaseInstances: TestCaseInstances;

    it("init", async () => {
        testCaseInstances = await initTestCaseInstances();
    });

    let accountIds: Record<string, AccountId> = {};
    it("create USD/EUR/TOKEN currency accounts", async () => {
        const currencyAccounts = await createCurrencyAccounts(testCaseInstances);
        accountIds = currencyAccounts.accountIds;
    }).timeout(10_000);

    const targetedTxIds: Record<string, bigint> = {};
    const broadcastTxIds: Record<string, bigint> = {};

    const actionName = "my.action";

    describe("transaction", () => {

        it("should send an action to a specific account", async function() {
            await Promise.all(Object.values(accountIds).map(async (accountId) => {
                const to = Object.values(accountIds).find((id) => id.hex !== accountId.hex);
                if (!to) {
                    throw new Error("No other account found");
                }

                targetedTxIds[accountId.hex] = await testCaseInstances.accountClient.action({
                    name: actionName,
                    payload: new TextEncoder().encode("my_data"),
                    fromAccount: accountId.bytes,
                    target: Target.create({
                        target: {
                            oneofKind: "accountId",
                            accountId: to.bytes,
                        },
                    }),
                });

                assert.isOk(targetedTxIds[accountId.hex]);
            }));
        });

        it("should broadcast an action to all accounts", async function() {
            await Promise.all(Object.values(accountIds).map(async (accountId) => {
                broadcastTxIds[accountId.hex] = await testCaseInstances.accountClient.action({
                    name: actionName,
                    fromAccount: accountId.bytes,
                    target: Target.create(),
                    payload: new TextEncoder().encode("my_data"),
                });

                assert.isOk(broadcastTxIds[accountId.hex]);
            }));
        });
    });

    describe("query", () => {
        it("should get a targeted action by id", async function() {
            await Promise.all(Object.values(accountIds).map(async (accountId) => {
                const targetedAccountId = Object.values(accountIds).find((id) => id.hex !== accountId.hex);

                if (!targetedAccountId) {
                    throw new Error("No other account found");
                }

                const action = await testCaseInstances.accountClient.getAction(targetedTxIds[targetedAccountId.hex]);

                const responseTargetedAccountId = action.target?.target?.oneofKind === "accountId" ? action.target?.target?.accountId : undefined;

                if (!responseTargetedAccountId) {
                    throw new Error("Targeted account ID is not present");
                }

                expect(AccountId.fromBytes(responseTargetedAccountId).hex).deep.equal(accountId.hex);
                expect(action.name).to.equal(actionName);
                expect(action.txId).deep.equal(targetedTxIds[targetedAccountId.hex]);
                const payload = new TextDecoder().decode(action.payload);
                expect(payload).to.equal("my_data");
            }));
        });

        it("should get a broadcast action by id", async function() {
            await Promise.all(Object.values(accountIds).map(async (accountId) => {
                const action = await testCaseInstances.accountClient.getAction(broadcastTxIds[accountId.hex]);

                const targetedAccountId = action.target?.target?.oneofKind === "accountId" ? action.target?.target?.accountId : undefined;

                expect(targetedAccountId).to.not.exist;
                expect(action.name).to.equal(actionName);
                expect(action.txId).deep.equal(broadcastTxIds[accountId.hex]);
                const payload = new TextDecoder().decode(action.payload);
                expect(payload).to.equal("my_data");
            }));
        });

        it("should get a list of actions", async function() {
            const limit = 2;

            await Promise.all(Object.values(accountIds).map(async (accountId) => {
                const actionsResponse = await testCaseInstances.accountClient.listActions({
                    name: actionName,
                    filter: {
                        oneofKind: "accountId",
                        accountId: accountId.bytes,
                    },
                    limit: BigInt(limit),
                    minTxId: 1n,
                });
                const actions = actionsResponse.actions;
                const targetedAccountId = Object.values(accountIds).find((id) => id.hex !== accountId.hex);
                if (!targetedAccountId) {
                    throw new Error("No other account found");
                }

                const allActionIds = [
                    ...Object.values(targetedTxIds),
                    ...Object.values(broadcastTxIds),
                ];

                expect(actions).to.be.instanceOf(Array);
                expect(actions?.length).to.equal(2);
                expect(actions.some(el => allActionIds.includes(el.txId))).to.be.true;
            }));
        });
    });
});
