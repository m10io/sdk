const { Transaction, RequestEnvelope } = require('../../proto')
const { CryptoHasher } = require('../../utils/crypto')

// Adds a signature to the contract, thereby approving the contents
const endorse = async({ ledgerId, signer, contract }) => {
    const signature = new RequestEnvelope.Signature()
    signature.setAlgorithm(signer.getAlgorithm())
    signature.setPublicKey(await signer.getPublicKey())
    signature.setSignature(await signer.sign(contract.getTransactions()))

    const endorsement = RequestEnvelope.Endorsement()
    endorsement.setLedgerId(ledgerId)
    endorsement.setSignature(signature)
    contract.addEndorsements(endorsement)
}

// Calculates the contract ID
const contractId = contract => {
    return new CryptoHasher().hash(contract.getTransactions())
}

// Checks if the contract has at least one signature for every ledger involved
const isFullyEndorsed = contract => {
    const transferRequests = Transaction.CreateLedgerTransfers.deserializeBinary(contract.getTransactions())
    const transfers = transferRequests.getTransfersList()
    return transfers.every(transfer => contract.getEndorsementsList().some(signature => signature.getLedgerId() === transfer.getLedgerId()))
}

module.exports = {
    endorse,
    contractId,
    isFullyEndorsed
}
