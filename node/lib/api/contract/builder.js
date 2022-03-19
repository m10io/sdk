const { Transaction, metadata } = require('../../proto')
const { Hex } = require('../../utils/utils')
const { memoMetadata } = require('../metadata')
exports.ContractBuilder = class ContractBuilder {
  constructor() {
    this.transfers = []
    this.validUntil = null
  }

  transfer({ ledgerId, fromAccountId, toAccountId, amount, memo = '' }) {
    const step = new Transaction.TransferStep()
    step.setFromAccountId(Hex.toUint8Array(fromAccountId))
    step.setToAccountId(Hex.toUint8Array(toAccountId))
    step.setAmount(amount)
    step.addMetadata(memoMetadata(memo))

    const transfer = new Transaction.CreateTransfer()
    transfer.setTransferStepsList([step])

    const transferRequest = new Transaction.CreateLedgerTransfer()
    transferRequest.setLedgerId(ledgerId)
    transferRequest.setNonce(
      Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
    )
    transferRequest.setTransfer(transfer)

    this.transfers.push(transferRequest)
    return this
  }

  validFor(durationInSeconds) {
    const currentTimeInMicros = new Date().getTime() * 1000
    this.validUntil = currentTimeInMicros + durationInSeconds * 1e6
    return this
  }

  build() {
    const ledgerTransferRequests = new Transaction.CreateLedgerTransfers()
    ledgerTransferRequests.setTransfersList(this.transfers)
    if (this.validUntil == null) {
      this.validFor(300)
    }
    ledgerTransferRequests.setValidUntil(this.validUntil)

    const contract = new metadata.Contract()
    contract.setTransactions(ledgerTransferRequests.serializeBinary())
    return contract
  }
}
