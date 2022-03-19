/* eslint-disable no-unused-expressions */
/* eslint-disable no-undef */
const { expect } = require('chai')
const { Base64 } = require('../lib/utils/utils')
const { CryptoSigner } = require('../lib/utils/crypto')

describe('Registration', function() {
  const context = this.parent
  const signer = CryptoSigner.generateKeyPair()

  it('it should register a new user, create an account, activate the account and issue funds to the new account', async function() {
    const name = Math.random().toString(36)
    const userId = await context.m10.createUser({ name, signer })
    console.log(`new user id: ${userId}`)

    const accountOwner = await signer.getPublicKey()
    const accountOwnerBase64 = Base64.fromUint8Array(accountOwner)

    const newAccountId = await context.m10.createLedgerAccount({
      parentId: context.parentAccountId,
      signer: context.bankAdmin
    })

    console.log(`new account id: ${newAccountId}`)
    console.log('creating account')

    await  await context.m10.createAccount({
      id: newAccountId,
      instrument: context.instrument.toUpperCase(),
      owner: accountOwnerBase64,
      signer: context.bankAdmin
    })

    const user =  await context.m10.updateUser({
      id: userId,
      accountsList: [`${context.ledgerId}/${newAccountId}`],
      signer
    })
    expect(user.accountsList.length).to.equal(1)

    console.log('creating transfer')

    const transactionId =  await context.m10.createTransfer({
      fromAccountId: context.parentAccountId,
      toAccountId: newAccountId,
      amount: '1000',
      signer: context.bankAdmin
    })

    expect(transactionId).to.exist
  }).timeout(20000)
})
