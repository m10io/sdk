import { assert } from "chai";

import { M10Client } from "../src";
import { Empty } from "../src/protobufs/google/protobuf/empty";


const LEDGER_URL = process.env.LEDGER_URL || "https://app.dev.m10.net";

describe("transaction", () => {

    describe("query", () => {

        it("blockHeight", async () => {

            const queryClient = M10Client.createQueryClient(LEDGER_URL);
            const response = await queryClient.getChainInfo(Empty.create());

            assert.isOk(response.response.blockHeight > 0);
        });
    });
});
