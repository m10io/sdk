// Example to convert an ED25519 key generated via the m10 CLI to something useable in JS.

const M10 = require('m10-sdk');

const keyHeader = "-----BEGIN PRIVATE KEY-----\n"
const keyFooter = "-----END PRIVATE KEY-----\n"

// key exported out of m10 cli
const sample = "MFMCAQEwBQYDK2VwBCIEIIEOT5vlgRx5qlNpavYfE63mXNAEGYkb5CZHMMD10ko9oSMDIQB10whMJ96apEvFigfqocGnL8TN2OXf641KsnXBHgd9rQ=="

async function run() {
    const instrument = process.env.CURRENCY || 'usd'
    const m10 = new M10({ address: 'qa.m10.net', secure: 'true' })

    await m10.refreshLedgers();

    const ledgerId = m10[`${instrument}.m10`]

    // peek the asn.1 here: https://lapo.it/asn1js 
    // we're going from a PKCS#8 v2 key => PKSC#8 v1 and discarding the public key information
    let key = 'MC4CAQAw' + sample.substr(8, 56);

    key = keyHeader + key + '\n' + keyFooter;

    const signer = new m10.crypto.CryptoSigner({ key });

    console.dir(signer);
}

run()