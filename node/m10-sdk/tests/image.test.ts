import { assert } from "chai";
import fs from "fs";

import {
    DirectoryClient,
    getBaseAccessToken,
    ImageClient,
} from "../src";
import { TEST_USERNAME, TEST_PASSWORD, LEDGER_URL } from "./config";

const INCREASED_TEST_TIMEOUT: number = 10000;

describe("images", () => {

    describe("query", () => {
        it("gets and puts an image", async () => {
            const TEST_IMAGE_BUFFER = fs.readFileSync("tests/assets/images/test-image.png");

            const accessToken = await getBaseAccessToken(LEDGER_URL.replace("grpc-", ""), TEST_USERNAME, TEST_PASSWORD);

            const directoryClient = new DirectoryClient(LEDGER_URL, accessToken);

            const createImageRequest = await directoryClient.createImageUrl("image/png");

            const imageClient = new ImageClient(createImageRequest.presignedUrl);

            await imageClient.putImage(TEST_IMAGE_BUFFER);

            const gotImage = await imageClient.getImage(createImageRequest.url);

            assert.deepEqual(TEST_IMAGE_BUFFER, gotImage);
        })
            .timeout(INCREASED_TEST_TIMEOUT);
    });
});
