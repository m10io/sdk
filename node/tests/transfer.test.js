/* eslint-disable no-unused-expressions */
/* eslint-disable no-undef */
const { expect } = require('chai')
const { Hex } = require('../lib/utils/utils')
const { createContract } = require('./utils')

describe('Transfer', function() {
  const context = this.parent

  let transactionId

  it('should create a transfer', async function() {
    transactionId = await context.m10.createTransfer({
      fromAccountId: context.parentAccountId,
      toAccountId: context.bobsAccountId,
      amount: '1000',
      memo: 'Funds',
      signer: context.bankAdmin
    })

    console.log('> transactionId', transactionId)

    const transfer = await context.m10.getTransfer({ id: transactionId, signer: context.bankAdmin })

    expect(transactionId).to.exist
    expect(transfer).to.have.all.keys(
      'txId',
      'fromAccountId',
      'toAccountId',
      'amount',
      'failed',
      'contextId',
      'metadata',
      'timestamp'
    )
  })

  it('should get a transfer by id', async function() {
    const transfer = await context.m10.getTransfer({ id: transactionId, signer: context.bankAdmin })
    expect(transfer.amount).to.exist
    expect(transfer.fromAccountId).to.exist
    expect(transfer.toAccountId).to.exist
  })

  it('should get an enhanced transfer by id', async function() {
    const transfer = await context.m10.getEnhancedTransfer({ id: transactionId, signer: context.bankAdmin })
    expect(transfer.amount).to.exist
    expect(transfer.fromAccountId).to.exist
    expect(transfer.toAccountId).to.exist
    expect(transfer.senderName).to.exist
    expect(transfer.receiverName).to.exist
  })

  it('should get a list of transfers', async function() {
    const transfers = await context.m10.listTransfers({
      accountId: context.bobsAccountId,
      limit: 10,
      signer: context.bob
    })

    expect(transfers).to.be.instanceOf(Array)
    expect(transfers.length).to.be.greaterThan(0)
    expect(transfers[0].amount).to.exist
    expect(transfers[0].fromAccountId).to.exist
    expect(transfers[0].toAccountId).to.exist
  })

  it('should get child transfers', async function() {
    const newAccount = await context.m10.createLedgerAccount({
      parentId: context.parentAccountId,
      signer: context.bankAdmin
    })
    await context.m10.createAccount({
      id: newAccount,
      instrument: 'USD',
      name: 'new-child-account',
      publicName: 'new-child-account',
      signer: context.bankAdmin
    })
    transactionId = await context.m10.createTransfer({
      fromAccountId: context.bobsAccountId,
      toAccountId: newAccount,
      amount: '16',
      memo: 'child-account-transfer',
      signer: context.bob
    })

    const transfers = await context.m10.listTransfers({
      accountId: context.parentAccountId,
      minTxId: transactionId,
      signer: context.bankAdmin
    })
    if (transfers.length) {
      expect(transfers.some(t => t.txId === transactionId)).false
    }

    const childTransfers = await context.m10.listTransfers({
      accountId: context.parentAccountId,
      minTxId: transactionId,
      includeChildAccounts: true,
      signer: context.bankAdmin
    })
    expect(childTransfers.some(t => t.txId === transactionId)).true
  })

  it('should get a list of enhanced transfers', async function() {
    const transfers = await context.m10.listEnhancedTransfers({
      accountId: context.bobsAccountId,
      limit: 10,
      signer: context.bob
    })

    expect(transfers).to.be.instanceOf(Array)
    expect(transfers.length).to.be.greaterThan(0)
    expect(transfers[0]).to.have.all.keys(
      'timestamp',
      'senderName',
      'senderBankName',
      'senderProfileImageUrl',
      'receiverName',
      'receiverBankName',
      'receiverProfileImageUrl',
      'txId',
      'fromAccountId',
      'toAccountId',
      'amount',
      'failed',
      'contextId',
      'metadata'
    )
  })

  it('it should get the ledger account', async function() {
    const ledgerAccount = await context.m10.getLedgerAccount({
      accountId: context.parentAccountId,
      signer: context.bankAdmin
    })

    expect(Hex.isValid(ledgerAccount.id)).to.be.true
    expect(ledgerAccount).to.have.all.keys('id', 'balance', 'frozen', 'issuance')
    expect(ledgerAccount.issuance).to.have.all.keys(
      'issuedBalance',
      'leafChildren',
      'nonLeafChildren'
    )
  })

  it('should submit a transaction as part of a contract', async function() {
    const contract = createContract(context.parentAccountId, context.bobsAccountId, context.ledgerId, context.bankAdmin)

    const transactionId = await context.m10.createTransfer({
      fromAccountId: context.parentAccountId,
      toAccountId: context.bobsAccountId,
      amount: '1',
      memo: 'Contract test',
      contract: contract,
      signer: context.bankAdmin
    })

    const transfer = await context.m10.getTransfer({ id: transactionId, signer: context.bankAdmin })

    expect(transactionId).to.exist
    expect(transfer).to.have.all.keys(
      'txId',
      'fromAccountId',
      'toAccountId',
      'amount',
      'failed',
      'contextId',
      'metadata',
      'timestamp'
    )
    const txnContract = transfer.metadata.contract
    expect(txnContract.length).to.equal(2)
    txnContract.forEach(transfer => {
      expect(transfer.fromAccountId).to.be.not.null
      expect(transfer.toAccountId).to.be.not.null
    })
  })
})
