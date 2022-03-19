/* eslint-disable no-unused-vars */
const { Any, Transaction, metadata } = require('../proto')
const { Hex } = require('../utils/utils')

const ATTACHMENT_TYPE_URL = 'm10.sdk.metadata.Attachment'
const MEMO_TYPE_URL = 'm10.sdk.metadata.Memo'
const FEE_TYPE_URL = 'm10.sdk.metadata.Fee'
const WITHDRAW_TYPE_URL = 'm10.sdk.metadata.Withdraw'
const DEPOSIT_TYPE_URL = 'm10.sdk.metadata.Deposit'
const CONTRACT_TYPE_URL = 'm10.sdk.metadata.Contract'

function memoMetadata(plaintext) {
  const memo = new metadata.Memo()
  memo.setPlaintext(plaintext)
  const any = new Any()
  any.setTypeUrl(MEMO_TYPE_URL)
  any.setValue(memo.serializeBinary())
  return any
}

function contractMetadata(contract) {
  const any = new Any()
  any.setTypeUrl(CONTRACT_TYPE_URL)
  any.setValue(contract.serializeBinary())
  return any
}

function parseMetadata(anys) {
  const data = {}
  for (const any of anys) {
    const typeUrl = any.getTypeUrl()
    if (typeUrl === MEMO_TYPE_URL) {
      const memo = metadata.Memo.deserializeBinary(any.getValue())
      data.memo = memo.getPlaintext()
    } else if (typeUrl === CONTRACT_TYPE_URL) {
      const contract = metadata.Contract.deserializeBinary(any.getValue())
      data.contract = parseContract(contract)
    }
  }
  return data
}

function parseContract(contract) {
  const ledgerTransfers = Transaction.CreateLedgerTransfers.deserializeBinary(contract.getTransactions())
  return ledgerTransfers.getTransfersList().flatMap(ledgerTransfer => {
      const transfer = ledgerTransfer.getTransfer()
      return transfer.getTransferStepsList().map(step => ({
          ledgerId: ledgerTransfer.getLedgerId(),
          nonce: ledgerTransfer.getNonce(),
          fromAccountId: Hex.fromUint8Array(step.getFromAccountId()),
          toAccountId: Hex.fromUint8Array(step.getToAccountId()),
          amount: step.getAmount(),
          metadata: parseMetadata(step.getMetadataList()),
      }))
  })
}

module.exports = {
  memoMetadata,
  contractMetadata,
  parseMetadata
}
