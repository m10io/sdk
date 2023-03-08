import { assert } from "chai";
import fs from "fs";

import { DirectoryClient } from "../src/directory";
import { ImageClient } from "../src/image";
import { TokenProvider } from "../src/utils/auth";

const LEDGER_URL = process.env.LEDGER_URL || "develop.m10.net";
const INCREASED_TEST_TIMEOUT: number = 5000;

const TEST_USERNAME = "ops+e2etest@m10.io";
const TEST_PASSWORD = "n@R*88JxsccRpuw";

describe("images", () => {

    describe("query", () => {

        it("gets and puts an image", async () => {
            const TEST_IMAGE_BUFFER = fs.readFileSync("tests/assets/images/test-image.png");

            const tokenProvider = new TokenProvider(TEST_USERNAME, TEST_PASSWORD, LEDGER_URL);

            const directoryClient = new DirectoryClient(LEDGER_URL, tokenProvider, true);

            const createImageRequest = await directoryClient.createImageUrl("image/png");

            const imageClient = new ImageClient(createImageRequest.presignedUrl);

            await imageClient.putImage(TEST_IMAGE_BUFFER);

            const gotImage = await imageClient.getImage(createImageRequest.url);

            assert.deepEqual(TEST_IMAGE_BUFFER, gotImage);
        })
            .timeout(INCREASED_TEST_TIMEOUT);
    });
});
