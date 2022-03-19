/* eslint-disable no-unused-expressions */
/* eslint-disable no-undef */
const { expect } = require('chai')
const { Uuid, Hex } = require('../lib/utils/utils')

describe('Account', function() {
  const context = this.parent
  let accountId = Uuid.newId()
  accountId = Hex.fromUint8Array(accountId)

  it('should create an account', async function() {
    await context.m10.createAccount({
      id: accountId,
      name: 'New Account',
      signer: context.bob,
    })

    const account = await context.m10.getAccount({ id: accountId, signer: context.bob })

    console.log(`New account: ${accountId}`)

    expect(Hex.isValid(account.id)).to.be.true
    expect(account).to.have.all.keys(
      'id',
      'owner',
      'name',
      'publicName',
      'profileImageUrl'
    )
  })

  it('should get an existing account', async function() {
    const account = await context.m10.getAccount({ id: accountId, signer: context.bob })

    expect(Hex.isValid(account.id)).to.be.true
  })

  it('should update an existing account', async function() {
    await context.m10.updateAccount({
      id: accountId,
      name: 'Spending',
      publicName: 'Alice R.',
      profileImageUrl: 'https://fake.m10.net/images/alice',
      signer: context.bankAdmin
    })
    const account = await context.m10.getAccount({ id: accountId, signer: context.bob })

    expect(account.name).to.equal('Spending')
    expect(account.publicName).to.equal('Alice R.')
    expect(account.profileImageUrl).to.equal('https://fake.m10.net/images/alice')
  })
})
