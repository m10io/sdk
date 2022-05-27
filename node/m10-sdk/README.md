# M10-SDK

The M10 (SDK) gives you access to the M10 APIs allowing you to build custom clients and banking integrations with the M10 Platform. Whether you are a central bank implementing custom applications, or a new Fintech who wants to build atop an M10 CBDC, the SDK is your first stop.

The M10-SDK is documented primary using TypeDoc (use `yarn run docs` to generate them) and examples in the tests.

## Installation & Requirements

Run `yarn install` for a node version >16.

## Using the SDK

Create an instance of the SDK and pass it the URL of your ledger Directory, specifying TLS:

```typescript
const sdk = require("m10-sdk");

const ledgerUrl = "develop.m10.net";
const ledgerClient = new sdk.client.LedgerClient(ledgerUrl, true);
```

Setup signer based on the key:

```typescript
const bankAdmin = "MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK";
const bankAdminSigner = new sdk.signer.CryptoSigner(getPrivateKey(bankAdmin));
```

Make a request:

- Query request example which shows how to list accounts with bankAdmin as an owner:

    ```typescript
    const request = { owner: bankAdminSigner.getPublicKey() };
    const response = await ledgerClient.listAccounts(bankAdminSigner, request);
    ```
    
- Transaction request example of creating an account:

    ```typescript
    const { m10 } = require("m10-sdk/protobufs");

    const account = new m10.sdk.model.Account({
        id: utils.getUint8ArrayFromAccountId(accountId),
        name: "Name",
        publicName: "Some Public Name",
        owner: bankAdminSigner.getPublicKey(),
    });
    const transactionData = sdk.collections.getCreateTransactionDataFromDocument(
        m10.sdk.model.Account.encode(account).finish(),
        sdk.collections.Collection.Account,
    );

    const request = ledgerClient.transactionRequest(transactionData);
    const response = await ledgerClient.createTransaction(signer, request);
    ```

We also have some `helpers`, but it's better not to rely on them and use them only as an example to create your own.

## Protobufs

Protobufs are generated using `yarn run proto` script, for this to work you need to have dependencies installed (`yarn install`) and have necessary proto files in `../../protobuf` folder.

## Tests 

Tests are split into two parts:
- `tests/integration` will give you some examples on how to implement certain functionality
- `tests/*` are used for testing all actions separate without context

All of them can be run using `yarn run test`, or `yarn run test:cover` which will generate coverage reports (with html one saved in `coverage` folder).

## Support

If you have any issues with using the SDK, please file an issue on Github
