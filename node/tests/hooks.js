const { assert } = require('chai')
const { Base64 } = require('../lib/utils/utils')
const { CryptoSigner } = require('../lib/utils/crypto')
const M10 = require('../lib')
const KEYS = require('./assets/testKeys')

const address = process.env.DIRECTORY_ADDRESS || 'develop.m10.net'
const disableTls = process.env.DISABLE_TLS === 'true'
const instrument = process.env.CURRENCY || 'ttt'
const ledgerId = `${instrument}.m10`

const bankName = 'NodeTB'

exports.mochaHooks = {
    // global setup for all tests
    beforeAll: async function() {
      const m10Client = new M10({ address: address, secure: !disableTls })

      // Init Api
      await m10Client.refreshLedgers(true)
      this.test.parent.m10 = m10Client[`${ledgerId}`]
      this.test.parent.ledgerId = `${instrument}.m10`
      this.test.parent.instrument = instrument

      // Setup bank admin
      this.test.parent.bankAdmin = new CryptoSigner(KEYS.BANK_ADMIN)
      const bankAccountOwner = await this.test.parent.bankAdmin.getPublicKey()
      const bankAccountOwnerBase64 = Base64.fromUint8Array(bankAccountOwner)
      const bankAccounts = await this.test.parent.m10.findAccountByOwner({
        owner: bankAccountOwnerBase64, signer: this.test.parent.bankAdmin
      })
      const bankIssuanceAccount = bankAccounts.find(account => {
        return account.publicName === bankName
      })

      if (!bankIssuanceAccount) {
        throw new Error(`Cannot find bank account for: ${bankName}`)
      }

      this.test.parent.parentAccountId = bankIssuanceAccount.id
      console.log(`bank issuance account id: ${this.test.parent.parentAccountId}`)

      // Setup test user "alice"
      this.test.parent.alice = CryptoSigner.generateKeyPair()
      let name = `Alice ${Math.random().toString(5)}`
      this.test.parent.aliceId = await this.test.parent.m10.createUser({ name, signer: this.test.parent.alice })
      console.log(`alice: ${this.test.parent.aliceId}`)

      /*
          final roleBinding = await bankAdmin.getRoleBinding(
        name: "customer", instrument: instrument);
    final userKey = await sdk.signer.publicKey();
    if (!roleBinding.subjects.contains(userKey)) {
      final subject = base64.encode(userKey);
      await bankAdmin.updateRoleBinding(
          id: roleBinding.id, instrument: instrument, subjects: [subject]);
    }
    */

      const roleBindings = await this.test.parent.m10.listRoleBinding({
        name: 'node-test-customer',
        signer: this.test.parent.bankAdmin
      })
      assert.notEmpty(roleBindings)
      const roleBinding = roleBindings[0]
      const alicePubKey = await this.test.parent.alice.getPublicKey()
      const subjects = []
      if (!roleBinding.subjects.includes(alicePubKey)) {
        console.log('adding Alice to subjects')
        subjects.push(alicePubKey)
      }

      // Setup test user "bob"
      this.test.parent.bob = CryptoSigner.generateKeyPair()
      name = `Bob ${Math.random().toString(5)}`
      this.test.parent.bobId = await this.test.parent.m10.createUser({ name, signer: this.test.parent.bob })
      console.log(`bob: ${this.test.parent.bobId}`)
      // Both user needs to be added to cutomer role binding before they
      // can create accounts
      const bobPubKey = await this.test.parent.bob.getPublicKey()
      if (!roleBinding.subjects.includes(bobPubKey)) {
        console.log('adding Bob to subjects')
        subjects.push(bobPubKey)
      }
      if (subjects.length > 0) {
        console.log(`Updating role binding ${roleBinding.id}`)
        await this.test.parent.m10.updateRoleBinding({ id: roleBinding.id, signer: this.test.parent.bankAdmin, subjects })
      }

      this.test.parent.bobsAccountId = await this.test.parent.m10.createLedgerAccount(
          { parentId: this.test.parent.parentAccountId, publicName: name, signer: this.test.parent.bob })
      console.log(`bob's account: ${this.test.parent.bobsAccountId}`)
      await this.test.parent.m10.createAccount({
          id: this.test.parent.bobsAccountId,
          name,
          publicName: name,
          signer: this.test.parent.bob
      })
      this.test.parent.bobsAccountRef = `${this.test.parent.ledgerId}/${this.test.parent.bobsAccountId}`
    }
}
