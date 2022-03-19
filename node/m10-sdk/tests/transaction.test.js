/* eslint-disable no-unused-expressions */
/* eslint-disable no-undef */
const { expect } = require('chai')
const { Hex, Uuid } = require('../lib/utils/utils')

describe('Context', function() {
  const context = this.parent

  it('should list by context', async function() {
    const contextId = Uuid.newId()
    const ids = await submitContextTxs(context, contextId)
    const txs = await context.m10.listTransactions({ contextId: contextId, signer: context.bankAdmin })
    expect(txs.length).to.equal(3)
    expect(txs[0].txId).to.equal(ids[2])
    expect(txs[0].transfer).to.be.not.null
    expect(txs[1].txId).to.equal(ids[1])
    expect(txs[1].transfer).to.be.not.null
    expect(txs[2].txId).to.equal(ids[0])
    expect(txs[2].action).to.be.not.null
  })

  it('should group by context', async function() {
    const entries = await Promise.all([...Array(3).keys()].map(async() => {
      const contextId = Uuid.newId()
      const ids = await submitContextTxs(context, contextId)
      return { contextId: Hex.fromUint8Array(contextId), ids: ids }
    }))
    const firstId = entries[0].ids[0]
    const groups = entries.reduce((dict, { contextId, ids }) => ({ ...dict, [contextId]: ids }), {})

    const groupedTxs = await context.m10.groupTransactions({
        accountId: context.parentAccountId,
        limitGroups: 3,
        minTxId: firstId,
        signer: context.bankAdmin
      })
    expect(groupedTxs.length, 3)
    groupedTxs.forEach(groupTx => {
      expect(groupTx.length, 3)
      const contextId = Hex.fromUint8Array(groupTx[0].contextId)
      const ids = groups[contextId]
      expect(groupTx.length, 3)
      expect(groupTx[0].txId, ids[2])
      expect(groupTx[0].transfer != null, true)
      expect(groupTx[1].txId, ids[1])
      expect(groupTx[1].transfer != null, true)
      expect(groupTx[2].txId, ids[0])
      expect(groupTx[2].action != null, true)
      })
  })
})

const submitContextTxs = async(context, contextId) => {
    const first = await context.m10.invokeAction({
        name: 'test.context',
        fromAccountId: context.parentAccountId,
        targetAccountId: context.bobsAccountId,
        payload: 'context',
        contextId: contextId,
        signer: context.bankAdmin,
    })
    expect(first.error).to.be.null

    const secondId = await context.m10.createTransfer({
        fromAccountId: context.parentAccountId,
        toAccountId: context.bobsAccountId,
        amount: 100,
        memo: 'Funds',
        contextId: contextId,
        signer: context.bankAdmin,
    })
    expect(secondId > 0, true)

    const thirdId = await context.m10.createTransfer({
        fromAccountId: context.parentAccountId,
        toAccountId: context.bobsAccountId,
        amount: 100,
        memo: 'Funds',
        contextId: contextId,
        signer: context.bankAdmin,
    })
    expect(thirdId > 0, true)

    return [first.txId, secondId, thirdId]
  }
