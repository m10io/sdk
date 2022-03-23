/* eslint-disable no-unused-expressions */
/* eslint-disable no-undef */
const { ContractBuilder } = require('../lib/api/contract/builder')
const { endorse } = require('../lib/api/contract/utils')
const M10 = require('../lib')

const address = process.env.DIRECTORY_ADDRESS || 'develop.m10.net'
const disableTls = process.env.DISABLE_TLS === 'true'
const instrument = process.env.CURRENCY || 'ttt'

exports.ledgerId = `${instrument}.m10`
exports.m10 = new M10({ address: address, secure: !disableTls })

// TODO: replace with live contract creation using FX broker
exports.createContract = function createContract(fromAccountId, toAccountId, ledgerId, signer) {
    const contract = new ContractBuilder().transfer({
        ledgerId,
        fromAccountId,
        toAccountId,
        amount: '1.00',
        memo: 'Contract part A'
    }).transfer({
        ledgerId: 'noexist.m10',
        fromAccountId,
        toAccountId,
        amount: '1.00',
        memo: 'Contract part B'
    }).validFor(180).build()
    endorse({ ledgerId, signer, contract })
    return contract
}
