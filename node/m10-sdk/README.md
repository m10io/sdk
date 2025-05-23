# M10-SDK

The M10 Software Development Kit (SDK) provides access to the M10 APIs, enabling you to build custom clients and banking integrations with the M10 Platform.

The M10-SDK is primarily documented using TypeDoc (you can generate the documentation by running `yarn run docs`) and through examples provided in the tests.

## Installing

```bash
npm install m10-sdk
```

## API

You can leverage the m10-sdk APIs to query details and metadata for Accounts and Resources, as well as Transactions. Additionally, you can use these APIs to initiate transfers, create documents, and invoke actions.

## Getting started

Let's start by defining `M10CLient`

You use `M10CLient` to query any data and send transactions in the M10 ecosystem.

```typescript
import { M10Client } from 'm10-sdk'

const ledgerUrl = "https://app.dev.m10.net";
const m10Client = new M10Client(ledgerUrl);
```

The preceding code defines a basic client that enables you to query public account details, transactions, and documents. Keep in mind that accessing certain data might necessitate specific permissions.

Within the M10 ecosystem, an `Account` represents an entry on the M10 ledger and is associated with an `Ed25519` public key.

To begin querying data or sending transactions, you need to define a `CryptoSigner`. This is a utility helper that works with `Ed25519` cryptography and the `pkcs8` PEM format.

```ts
import { CryptoSigner } from 'm10-sdk'

const pkcs8PemKey = "MC4CAQAwBQYDK2VwBCIEIIrikV/M3erX0lqmQgVXDRU1yFLStge7RyyvXv+kDesK"
const aliceSigner = await CryptoSigner.fromPkcs8Pem(pkcs8PemKey)
```

or with `Ed25519` private key

```ts
import { CryptoSigner } from 'm10-sdk'

const ed25519PrivateKeyHex = "0x......"
const aliceSigner = await CryptoSigner.fromSeed(
    Buffer.from(ed25519PrivateKeyHex, "hex")
)
```

With this you can perform a signed operations with `M10Client`

```ts
const aliceClient = new M10Client(ledgerUrl, aliceSigner);

const accountFromBytes = new Uint8Array();
const accountToBytes = new Uint8Array();

const amountToTransfer = 5n;

const decimalPlaces = 2n;

aliceClient.transfer([
    {
        fromAccountId: accountFromBytes,
        toAccountId: accountToBytes,
        amount: amountToTransfer * (10n ** decimalPlaces),
    },
])
```

To obtain an account, an entity with the necessary permissions must create one for you.

If you are developing an application with a sign-up process:

1) Generate the user's key pair:

```ts
const newUser = CryptoSigner.generateKeyPair()
```

2) Transmit the user's public key to your secure server. On this server, an `operator` or `admin` account (one with sufficient permissions) can then create a new account and associate it with the user's public key.

```ts
const submit = async () => {
    const response = await fetch("https://your-private-server.com/accouns", {
        method: "POST",
        body: JSON.stringify({
            publicKey: aliceSigner.getPublicKey().toString(), // base64
            name: "alice",
        })
    })
}
```

3) Submit the request from your private server:

```ts
import { Operation_InsertDocument } from "m10-sdk/protobufs/sdk/document";
import { AccountMetadata } from "m10-sdk/protobufs/sdk/model/model";
import { Role, Rule_Verb, RoleBinding } from "m10-sdk/protobufs/sdk/rbac";

import {
    AccountId,
    Collection,
    ResourceId,
    CryptoSigner,
    DirectoryClient,
    getBaseAccessToken,
    M10Client,
} from "m10-sdk";

// assume this in env variables
const operatorKey = 'MC4CA...'

const handleSignUp = async (req, res) => {
    const { publicKey, name } = req.json<{ publicKey: string, name: string }>()

    const operatorClient = new M10Client(
        'https://...',
        await CryptoSigner.fromPkcs8Pem(operatorKey)
    )

    /* Creates account on the ledger */
    const [, accountId] = await operatorClient.createAccount({
        parentId: bankId.bytes,
        frozen: false,
        issuance: true,
    });

    /* Bind public key to newly created account */
    await operatorClient.documents([
        {
            oneofKind: "insertDocument",
            insertDocument: Operation_InsertDocument.create({
                collection: Collection.AccountMetadata,
                document: AccountMetadata.toBinary(
                    AccountMetadata.create({
                        name: name,
                        owner: publicKey,
                        publicName: name,
                        id: accountId.bytes,
                    }),
                ),
            }),
        },
    ]);

    /* Define role and rules for account (permissions) */
    const bankCustomerRole = Role.create({
        id: ResourceId.generate().bytes,
        name: `${name}`,
        rules: [
            {
                collection: Collection.AccountMetadata,
                verbs: [Rule_Verb.CREATE],
            },
            {
                collection: Collection.LedgerAccount,
                verbs: [
                    Rule_Verb.CREATE,
                    ...(additionalLedgerAccRuleVerbs ?? []),
                ],
            },
        ],
    });

    /* send transaction to create document with account's permissions */
    await operatorClient.documents([{
        oneofKind: "insertDocument",
        insertDocument: Operation_InsertDocument.create({
            collection: Collection.Role,
            document: Role.toBinary(bankCustomerRole),
        }),
    }]);

    /* bind those permissions to created account */
    const bankCustomerRoleBinding = RoleBinding.create({
        id: ResourceId.generate().bytes,
        name: `${name}-role-binding`,
        isUniversal: false,
        subjects: [publicKey],
        expressions: [],
        owner: publicKey,
        role: bankCustomerRole.id,
    });

    /* send transaction to create document with role binding */
    await operatorClient.documents([{
        oneofKind: "insertDocument",
        insertDocument: Operation_InsertDocument.create({
            collection: Collection.RoleBinding,
            document: RoleBinding.toBinary(bankCustomerRoleBinding),
        }),
    }]);

    res.ok(204);
}
```

Upon submission, the user's account will be created on the ledger. In the application you are developing, you can retrieve the details of this newly created account as follows:

```ts
import { IndexedAccount } from 'm10-sdk/protobufs/sdk/transaction/transaction'
import { AccountInfo, AccountMetadata } from 'm10-sdk/protobufs/sdk/model/model'

type UserAccount = {
  indexedDetails: IndexedAccount
  account: AccountInfo
  bank: AccountInfo
}

const loadAccounts = async () => {
    const accountListAccounts = await userClient.listAccounts({
        filter: {
            oneofKind: 'owner',
            owner: userSigner.getPublicKey().toUint8Array(),
        },
    })

    return Promise.all(
        accountListAccounts.map(async account => {
            const accountId = AccountId.fromBytes(account.id)

            const [indexedDetails, userAccountInfo] = await Promise.all([
                userClient.getAccount(accountId),
                userClient.getAccountInfo(accountId),
            ])

            const bankInfo = await userClient.getAccountInfo(
                AccountId.fromBytes(userAccountInfo.parentAccountId),
            )

            return {
                indexedDetails,
                account: userAccountInfo,
                bank: bankInfo,
            }
        }),
    )
}
```

Or you can create a transfer:

```ts
const transfer = async (from: AccountId, to: AccountId, amount: bigint) => {
    const txId = await userClient.transfer(
      [
        {
          fromAccountId: from.bytes,
          toAccountId: to.bytes,
          amount: amount,
        },
      ],
    )

    return userClient.getTransfer(txId)
}
```

## Contribution

### Protobufs

The M10 SDK communicates with the M10 platform using generated M10 Protocol Buffers (protobufs). To generate the required models, TypeScript types, and API client code from the protobuf definitions, execute the following command:

```bash
yarn protoc:ts
```

## Tests 

The tests are divided into two categories:
- `tests/integration`: This directory provides examples of how to implement specific functionalities.
- `tests/*`: These tests are designed to evaluate individual actions in isolation, without any external context.

You can execute all tests using the command `yarn test`. To generate coverage reports, including an HTML report saved in the `coverage` folder, use the command `yarn test:cover`.

## Support

If you have any issues with using the SDK, please file an issue on Github.
