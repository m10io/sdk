# m10-sdk

The M10 Javascript SDK

## Using the SDK

Create an instance of the SDK and pass it your host settings:

```javascript
import M10Sdk from "../lib/sdk";

const signer = new KeyPair(KeyPair.Algorithm.ED25519);
signer.loadKeyPair("./tests/assets/root_key.pkcs8");

const sdk = new M10Sdk("develop.m10.net", true, signer);

const userId = await sdk.createUser("bob");
const user = await sdk.getUser(userId);
```

## Generating protobufs

The protobuf APIs can be generated using the following command (requires protobufV3):

```sh
yarn prepack
```

## Testing

Run the following command in order to run the unit test suite:

```sh
yarn test
```

## Debugging with VSCode

Add this to your `launch.json` file:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "env": {
        // Update env to point to correct host
        "LEDGER_API_HOST": "develop.m10.net"
      },
      "type": "node",
      "request": "launch",
      "name": "Mocha Tests",
      "program": "${workspaceFolder}/node_modules/mocha/bin/_mocha",
      "args": ["tests/*.js"],
      "internalConsoleOptions": "openOnSessionStart",
      "skipFiles": ["<node_internals>/**"]
    }
  ]
}
```

Then press `F5` to debug
