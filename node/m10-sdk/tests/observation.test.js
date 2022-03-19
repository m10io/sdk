/* eslint-disable no-unused-expressions */
/* eslint-disable no-undef */
const { expect } = require('chai')
const { Hex } = require('../lib/utils/utils')

describe('Observe', function() {
  const context = this.parent
  const unknownAccount = '80800001000000000100000000000003' // Undefined account [16,1;1]

  it('should observe a successful transfer', async function() {
    const stream = await context.m10.observeTransfers({
      involvesAccounts: [context.parentAccountId],
      signer: context.bankAdmin,
    })

    let timeout = false
    const cancelStreamID = setTimeout(() => {
      console.log('Stopping successful stream')
      stream.destroy()
      timeout = true
    }, 10000)

    await sleep(500)
    const transactionID = await context.m10.createTransfer({
      fromAccountId: context.parentAccountId,
      toAccountId: context.bobsAccountId,
      amount: '100',
      memo: 'Funds',
      signer: context.bankAdmin
    })

    let success = false
    for await (const transferResults of stream) {
      console.log('Received transfers', transferResults)
      if (transferResults.some(res => res.txId === transactionID)) {
        success = true
        break
      }
    }

    console.log('Successful observation stream has finished')
    clearTimeout(cancelStreamID)
    expect(success, 'No such transfer observed').to.be.true
    expect(timeout, 'Stream timed out').to.be.false
  })

  it('should observe an unsuccessful transfer', async function() {
    const stream = await context.m10.observeTransfers({
      involvesAccounts: [context.parentAccountId],
      signer: context.bankAdmin,
    })

    let timeout = false
    const cancelStreamID = setTimeout(() => {
      console.log('Stopping unsuccessful stream')
      stream.destroy()
      timeout = true
    }, 10000)

    // Sending to an unknown account
    await sleep(500)
    await context.m10.createTransfer({
      fromAccountId: context.parentAccountId,
      toAccountId: unknownAccount,
      amount: '1.00',
      memo: 'Funds',
      signer: context.bankAdmin
    })

    let success = false
    for await (const transferResults of stream) {
      console.log('Received transfers', transferResults)
      if (transferResults.some(res => res.error != null)) {
          success = true
          break
      }
    }

    console.log('Unsuccessful observation stream has finished')
    clearTimeout(cancelStreamID)
    expect(success, 'No such transfer observed').to.be.true
    expect(timeout, 'Stream timed out').to.be.false
  })

  it('should observe a resource change', async function() {
    const accountId = await context.m10.createAccount({ publicName: 'Dennis V1', signer: context.bankAdmin })
    const stream = await context.m10.observeResources({
      collection: 'accounts',
      filter: '|id, doc| id == account',
      variables: [{ name: 'account', value: Hex.toUint8Array(accountId) }],
      signer: context.bankAdmin,
    })

    let timeout = false
    const cancelStreamID = setTimeout(() => {
      console.log('Stopping successful stream')
      stream.destroy()
      timeout = true
    }, 10000)

    await sleep(500)
    const transactionID = await context.m10.updateAccount({
      id: accountId,
      publicName: 'Dennis V2',
      signer: context.bankAdmin
    })

    let success = false
    for await (const resourceResults of stream) {
      console.log('Received resource changes', resourceResults)
      const results = resourceResults.find(res => res.txId === transactionID)
      if (results != null) {
        expect(results.operations.length === 1, 'Expected to observe a single operation')
        success = true
        break
      }
    }

    console.log('Successful observation stream has finished')
    clearTimeout(cancelStreamID)
    expect(success, 'No such resource change observed').to.be.true
    expect(timeout, 'Stream timed out').to.be.false
  })
})

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}
