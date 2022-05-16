// Create a M1 bank example
const M10 = require('m10-sdk')

const { Base64, Hex } = require('m10-sdk/lib/utils/utils')
const config = require('./config/config.json')
const _ = require('lodash')

const instrument = process.env.CURRENCY || 'usd'
const ledgerId = `${instrument}.m10`

function convertKey(pkcs8v2Key) {
    const keyHeader = "-----BEGIN PRIVATE KEY-----\n"
    const keyFooter = "-----END PRIVATE KEY-----\n"

    let key = 'MC4CAQAw' + pkcs8v2Key.substr(8, 56);

    key = keyHeader + key + '\n' + keyFooter;

    return key;
}

// TODO: pass these as variables, or just configure them
const name = 'TestBank'
const publicName = 'Test Name for a Bank'
const profileImageUrl = ''

async function run() {
    const m10 = new M10({ address: 'qa.m10.net', secure: 'true' })
    await m10.refreshLedgers();
    const ledger = m10[`${ledgerId}`];

    // decoded keys
    const operatorKey = convertKey(config.operator_key);
    const cbdcKey = convertKey(config.central_bank_key);
    const bankKey = convertKey(config.banks_key);
       
    let cbdcSigningKey = new m10.crypto.CryptoSigner({ key: cbdcKey });
    let bankSigningKey = new m10.crypto.CryptoSigner({ key: bankKey });

    const base64CbdcKey = Base64.fromUint8Array(await cbdcSigningKey.getPublicKey());
    const base64BankKey = Base64.fromUint8Array(await bankSigningKey.getPublicKey());

    // this logic is copied from: https://github.com/m10io/sdk/blob/main/examples/cbdc-admin/src-tauri/src/context.rs#L143

    // find our instrument
    
    const bankAccounts = await ledger.findAccountByOwner({ owner: base64CbdcKey, signer: cbdcSigningKey })
    const cbdcCreatorInstrument = _.find(bankAccounts, { name: instrument.toUpperCase() });

    // create a ledger account for an m1 bank
    const createdLedgerAccount = await ledger.createLedgerAccount({ 
        parentId: cbdcCreatorInstrument.id, 
        // TODO: include these in SDK
        // issuance: true, 
        // frozen: false,
        signer: cbdcSigningKey,
        owner: base64BankKey,
    });

    console.log('Id of created ledger account', createdLedgerAccount)

    const createdAccount = await ledger.createAccount({
        id: createdLedgerAccount,
        name, 
        publicName, 
        profileImageUrl,
        instrument: instrument.toUpperCase(),
        owner: base64BankKey,
        signer: cbdcSigningKey,
    });

    console.log('Created account:', createdAccount)

    updateBinding =  await ledger.updateRoleBinding({
        id: config.sandbox_nostro_role,
        subjects: [ base64BankKey ], 
        signer: cbdcSigningKey,
    })
}

run()