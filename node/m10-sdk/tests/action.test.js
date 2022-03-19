/* eslint-disable no-unused-expressions */
/* eslint-disable no-undef */
const { expect } = require('chai').use(require('chai-bytes'))

describe('Action', function() {
  const context = this.parent

  let transactionId

  it('should invoke an action', async function() {
    const response = await context.m10.invokeAction({
        name: 'm10.Request',
        fromAccountId: context.parentAccountId,
        targetAccountId: context.bobsAccountId,
        payload: 'this is a payload',
        signer: context.bankAdmin
    })
    expect(response.error, `Action invocation failed: ${response.error}`).to.be.null

    transactionId = response.txId
    expect(transactionId).to.be.not.null
    console.log('> transactionId', response.txId)
  })

  it('should get an action by id', async function() {
    expect(transactionId ?? 0, 'Transaction ID was not set during previous test').not.equals(0)
    const action = await context.m10.getAction({ id: transactionId, signer: context.bankAdmin })

    expect(action).to.have.all.keys(
      'txId',
      'contextId',
      'name',
      'fromAccountId',
      'target',
      'payload',
    )
  })

  it('should get a list of actions', async function() {
    const first = await context.m10.invokeAction({
        name: 'm10.Request',
        fromAccountId: context.parentAccountId,
        targetAccountId: context.bobsAccountId,
        payload: Uint8Array.from([1]),
        signer: context.bankAdmin
    })
    expect(first.error, `Action invocation failed: ${first.error}`).to.be.null
    const second = await context.m10.invokeAction({
        name: 'm10.Request',
        fromAccountId: context.parentAccountId,
        targetAccountId: context.bobsAccountId,
        payload: Uint8Array.from([2]),
        signer: context.bankAdmin
    })
    expect(second.error, `Action invocation failed: ${second.error}`).to.be.null

    const actions = await context.m10.listActions({
        name: 'm10.Request',
        accountId: context.bobsAccountId,
        minTxId: first.txId,
        maxTxId: second.txId,
        limit: 10,
        signer: context.bankAdmin
    })

    expect(actions).to.be.instanceOf(Array)
    expect(actions.length).to.equal(2)
    actions.forEach(action => {
        expect(action.fromAccountId).to.equal(context.parentAccountId)
        expect(action.target).to.equal(context.bobsAccountId)
    })
    expect(actions[0].payload).to.equalBytes([2])
    expect(actions[1].payload).to.equalBytes([1])
  })

  it('should filter actions by name', async function() {
    const first = await context.m10.invokeAction({
        name: 'm10.Request',
        fromAccountId: context.parentAccountId,
        targetAccountId: context.bobsAccountId,
        payload: Uint8Array.from([1]),
        signer: context.bankAdmin
    })
    expect(first.error, `Action invocation failed: ${first.error}`).to.be.null
    const second = await context.m10.invokeAction({
        name: 'm10.Request',
        fromAccountId: context.parentAccountId,
        targetAccountId: context.bobsAccountId,
        payload: Uint8Array.from([2]),
        signer: context.bankAdmin
    })
    expect(second.error, `Action invocation failed: ${second.error}`).to.be.null

    const actions = await context.m10.listActions({
        name: 'm10.NotARequest',
        accountId: context.bobsAccountId,
        minTxId: first.txId,
        maxTxId: second.txId,
        limit: 10,
        signer: context.bankAdmin
    })

    expect(actions).to.be.instanceOf(Array)
    expect(actions).to.be.empty
  })
})
