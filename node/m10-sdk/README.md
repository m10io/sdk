# m10-sdk

The M10 Javascript SDK    
</br>

## Using the SDK

> Create an instance of the SDK and pass it the URL of your ledger Directory, specifying TLS:

```javascript
const M10 = require('m10-sdk')
const m10 = new M10({ address: 'develop.m10.net', secure: 'true' })
```

> Initialize your SDK by requesting the latest information on ledgers managed by the specified Directory:
```javascript
async function run() {
    await m10.refreshLedgers()
}
run()
```
**Note**: this will set a timeout to refresh ledgers on a recurring basis, you can pass skipRefresh=true to disable    
</br>   


### A robust example of a basic SDK workflow

```javascript
// IMPORT THE M10 SDK
const M10 = require('m10-sdk')

// BANK INTEGRATION CONFIGURATION INFO, NORMALLY CREATED WITH THE M10 CLI
const BANK_PRIVATE_KEY = //{BANK ISSAUNCE ACCOUNT KEY}
const BANK_ISSUANCE_ACCOUNT = //{BANK ISSAUNCE ACCOUNT ID}
const ledgerId = //{LEDGER ID OF YOUR ASSET LISTED ON THE DIRECTORY}

// CREATE INSTANCE OF M10 SDK
const m10 = new M10({ address: 'develop.m10.net', secure: 'true' })

async function run() {
    // INITIAL THE SDK, FETCHING LEDGER REGISRATION INFO
    await m10.refreshLedgers(true)
    console.log(m10.ledgers)

    // LOAD THE BANK PRIVATE KEY FOR USE, CHECK ISSUANCE ACCOUNT BALANCE
    const bankKey = new m10.crypto.CryptoSigner(BANK_PRIVATE_KEY)
    let issuanceAccount = await m10[ledgerId].getLedgerAccount({
        accountId: BANK_ISSUANCE_ACCOUNT,
        signer: bankKey
    })
    console.log(issuanceAccount)

    // CREATE A NEW USER & KEY FOR ALICE & BOB
    const alicesKey = m10.crypto.CryptoSigner.generateKeyPair()
    const alicesUserId = await m10[ledgerId].createUser({ name: 'Alice', signer: alicesKey })

    const bobsKey = m10.crypto.CryptoSigner.generateKeyPair()
    const bobsUserId = await m10[ledgerId].createUser({ name: 'Bob', signer: bobsKey })

    // AS THE BANK, GRANT ALICE & BOB THE ROLE OF CUSTOMER
    const bankRoleBindings = await m10[ledgerId].listRoleBinding({
        name: 'node-test-customer',
        signer: bankKey
    })
    const customerRoleBinding = bankRoleBindings[0]

    await m10[ledgerId].updateRoleBinding({
        id: customerRoleBinding.id,
        signer: bankKey,
        subjects: [alicesUserId, bobsUserId]
    })

    // AS THE BANK, OPEN NEW ACCOUNTS FOR ALICE & BOB
    const alicesAccountId = await m10[ledgerId].createLedgerAccount({
        parentId: BANK_ISSUANCE_ACCOUNT,
        publicName: "Alice's Holding Account",
        owner: await alicesKey.getPublicKey(),
        signer: bankKey
    })
    const bobsAccountId = await m10[ledgerId].createLedgerAccount({
        parentId: BANK_ISSUANCE_ACCOUNT,
        publicName: "Bob's Holding Account",
        owner: await bobsKey.getPublicKey(),
        signer: bankKey
    })

    // AS THE BANK, DEPOSIT FUNDS INTO ALICE'S ACCOUNT
    const alicesDepositId = await m10[ledgerId].createTransfer({
        fromAccountId: BANK_ISSUANCE_ACCOUNT,
        toAccountId: alicesAccountId,
        amount: '100',
        memo: 'Free Money for Opening An Account!',
        signer: bankKey
    })
    console.log(alicesDepositId)

    // AS ALICE, SEE THE DEPOSIT TRANSFER TO ALICE'S ACCOUNT
    const alicesDeposit = await m10[ledgerId].getTransfer({
        id: alicesDepositId,
        signer: alicesKey
    })
    console.log(alicesDeposit)

    // AS ALICE, TRANSFER FUNDS FROM ALICE'S ACCOUNT TO BOB'S ACCOUNT
    const alicesPaysBobId = await m10[ledgerId].createTransfer({
        fromAccountId: alicesAccountId,
        toAccountId: bobsAccountId,
        amount: '50',
        memo: 'Thanks for Breakfast!',
        signer: alicesKey
    })
    console.log(alicesPaysBobId)

    const alicesPaysBob = await m10[ledgerId].getTransfer({
        id: alicesPaysBobId,
        signer: alicesKey
    })
    console.log(alicesPaysBob)

    // AS ALICE, CHECK ALICE'S ACCOUNT BALANCE
    const alicesAccount = await m10[ledgerId].getLedgerAccount({
        accountId: alicesAccountId,
        signer: alicesKey
    })
    console.log(alicesAccount)

    // AS BOB, CHECK BOB'S ACCOUNT BALANCE
    const bobsAccount = await m10[ledgerId].getLedgerAccount({
        accountId: bobsAccountId,
        signer: bobsKey
    })
    console.log(bobsAccount)

    // AS THE BANK, CHECK ISSUANCE ACCOUNT BALANCE HAS UPDATED
    issuanceAccount = await m10[ledgerId].getLedgerAccount({
        accountId: BANK_ISSUANCE_ACCOUNT,
        signer: bankKey
    })
    console.log(issuanceAccount)
}

run()
```

## Developing the m10-sdk
</br>

### Generating protobufs

> The protobuf APIs can be generated using the following command (requires protobufV3):

```sh
yarn prepack
```

### Testing

> Run the following command in order to run the unit test suite:

```sh
yarn test
```

