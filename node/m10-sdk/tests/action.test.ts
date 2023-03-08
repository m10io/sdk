/* eslint-disable @typescript-eslint/no-magic-numbers */
import { assert, expect } from "chai";
import { TextDecoder, TextEncoder } from "util";

import { ActionBuilder, ActionsFilter } from "../src";
import { M10Error } from "../src/error";
import { isSome, unwrap } from "../src/utils";

import type { TestContext } from "./hooks";

describe("action", () => {

    let targetedTxId: Option<LongNumber>;
    let broadcastTxId: Option<LongNumber>;

    const actionName = "my.action";

    describe("transaction", () => {

        it("should send an action to a specific account", async function(this: TestContext) {
            const context = unwrap(this.context, M10Error.Other("TestSuiteContext is None"));

            targetedTxId = await context.client.action(
                ActionBuilder
                    .forAccount(actionName, context.parentAccountId, context.bobsAccountId)
                    .payload(new TextEncoder().encode("my_data")),
            );

            assert.isOk(isSome(targetedTxId));
        });

        it("should broadcast an action to all accounts", async function(this: TestContext) {
            const context = unwrap(this.context, M10Error.Other("TestSuiteContext is None"));

            broadcastTxId = await context.client.action(
                ActionBuilder
                    .forAll(actionName, context.parentAccountId)
                    .payload(new TextEncoder().encode("my_data")),
            );

            assert.isOk(isSome(broadcastTxId));
        });
    });

    describe("query", () => {

        it("should get a targeted action by id", async function(this: TestContext) {
            const context = unwrap(this.context, M10Error.Other("TestSuiteContext is None"));

            const action = await context.client.getAction(unwrap(targetedTxId, M10Error.Other("targetedTxId is None")));

            expect(action.target.accountId).deep.equal(context.bobsAccountId);
            expect(action.name).to.equal(actionName);
            expect(action.txId).deep.equal(targetedTxId);
            const payload = new TextDecoder().decode(action.payload);
            expect(payload).to.equal("my_data");
        });

        it("should get a broadcast action by id", async function(this: TestContext) {
            const context = unwrap(this.context, M10Error.Other("TestSuiteContext is None"));

            const action = await context.client.getAction(unwrap(broadcastTxId, M10Error.Other("targetedTxId is None")));

            expect(action.target.accountId).to.not.exist;
            expect(action.name).to.equal(actionName);
            expect(action.txId).deep.equal(broadcastTxId);
            const payload = new TextDecoder().decode(action.payload);
            expect(payload).to.equal("my_data");
        });

        it("should get a list of actions", async function(this: TestContext) {
            const context = unwrap(this.context, M10Error.Other("TestSuiteContext is None"));
            const limit = 2;

            const actions = await context.client.listActions(
                ActionsFilter.byAccount(actionName, context.parentAccountId)
                    .limit(limit)
                    .min(1),
            );

            expect(actions).to.be.instanceOf(Array);
            expect(actions?.length).to.equal(2);
            expect(actions[1]?.txId).deep.equal(targetedTxId);
            expect(actions[0]?.txId).deep.equal(broadcastTxId);
        });
    });
});
