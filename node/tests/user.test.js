/* eslint-disable no-unused-expressions */
/* eslint-disable no-undef */
const { expect } = require('chai')
const { Uuid } = require('../lib/utils/utils')
const { CryptoSigner } = require('../lib/utils/crypto')

describe('User', function() {
  const context = this.parent
  const signer = CryptoSigner.generateKeyPair()

  let userId

  it('should create a user', async function() {
    userId = await context.m10.createUser({ signer })
    const user = await context.m10.getUser({ id: userId, signer })
    console.log(`New user: ${userId}`)

    expect(userId).to.exist
    expect(Uuid.isValid(userId)).to.be.true
    expect(user).to.have.all.keys(
      'id',
      'owner',
      'accountsList'
    )
  })

  it('should get an existing user', async function() {
    const user = await context.m10.getUser({ id: userId, signer })

    expect(Uuid.isValid(user.id)).to.be.true
  })

  it('should update an existing user', async function() {
    await context.m10.updateUser({
      id: userId,
      accountsList: ['ttt.m10/05800002000000003d00000000000003'],
      signer,
    })
    const user = await context.m10.getUser({ id: userId, signer })

    expect(user.accountsList.length).to.equal(1)
    expect(user.accountsList[0]).to.equal('ttt.m10/05800002000000003d00000000000003')
  })

  it('should delete an existing user', async function() {
    const user = await context.m10.deleteUser({ id: userId, signer })

    expect(Uuid.isValid(user.id)).to.be.true
  })
})
