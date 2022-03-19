const {
  Transaction,
  Queries: query,
  Observations: observe,
  Model: model,
  QueryServiceClient: QueryClient,
  TxServiceClient: TxClient,
  rbac,
  Queries,
} = require('../proto')
const { TxId } = require('../generated/sdk/api_pb')
const { DocumentUpdateBuilder } = require('../api/document_update/builder')
const { memoMetadata, contractMetadata, parseMetadata } = require('../api/metadata')
const {
  actionRequestFrom,
  createTransactionFrom,
  deleteTransactionFrom,
  updateTransactionFrom,
  createTransferFrom,
  createLedgerAccountRequestFrom,
  signedEnvelopFrom,
  createExpression,
} = require('./requests')
const {
  v4: uuidv4,
  stringify: uuidStringify,
  parse: uuidParse,
} = require('uuid')
const grpc = require('@grpc/grpc-js')
const { required, Hex, Base64, Uuid } = require('../utils/utils')
const promisifyAll = require('../utils/grpc/promisify')
const { Transform } = require('stream')

const { contractId } = require('./contract/utils')

class LedgerClient {
  constructor({ host, disableTls, instrument, decimals }) {
    const creds = !disableTls
      ? grpc.credentials.createSsl()
      : grpc.credentials.createInsecure()
    this.queryClient = promisifyAll(new QueryClient(host, creds))
    this.txClient = promisifyAll(new TxClient(host, creds))
    this.instrument = instrument
    this.decimals = decimals
  }

  // Account
  async createAccount({
    id,
    name,
    publicName,
    profileImageUrl,
    owner,
    contextId,
    signer,
  }) {
    if (id) {
      Hex.verify(id)
    } else {
      id = Uuid.newId()
      id = Hex.fromUint8Array(id)
    }

    const account = new model.Account()
    account.setId(Hex.toUint8Array(id))

    if (name != null) {
      account.setName(name)
    }
    if (publicName != null) {
      account.setPublicName(publicName)
    }
    if (profileImageUrl != null) {
      account.setProfileImageUrl(profileImageUrl)
    }
    if (owner) {
      Base64.verify(owner)
      account.setOwner(Base64.toUint8Array(owner))
    } else {
      account.setOwner(await signer.getPublicKey())
    }

    const request = createTransactionFrom(account)
    const envelop = await signedEnvelopFrom({ request, signer, contextId })
    await this.txClient.createTransaction(envelop)

    // NOTE: account ids are in hex rather than uuid format
    return id
  }

  async getAccount({ id = required('id'), signer }) {
    Hex.verify(id)

    const request = new query.GetAccountRequest()
    request.setId(Hex.toUint8Array(id))
    const envelop = await signedEnvelopFrom({ request, signer })
    const account = await this.queryClient.getAccount(envelop)

    return {
      id: id,
      owner: Base64.fromUint8Array(account.getOwner()),
      name: account.getName(),
      publicName: account.getPublicName(),
      profileImageUrl: account.getProfileImageUrl(),
    }
  }

  async getAccountInfo({ id = required('id'), signer }) {
    Hex.verify(id)

    const request = new query.GetAccountRequest()
    request.setId(Hex.toUint8Array(id))
    const envelop = await signedEnvelopFrom({ request, signer })
    const info = await this.queryClient.getAccountInfo(envelop)

    return {
      id: id,
      parentId: info.getParentAccountId() != null ? Hex.fromUint8Array(info.getParentAccountId()) : null,
      publicName: info.getPublicName(),
      profileImageUrl: info.getProfileImageUrl(),
    }
  }

  async findAccountByOwner({ owner = required('owner'), signer }) {
    Base64.verify(owner)

    const request = new query.ListAccountsRequest()
    request.setOwner(Base64.toUint8Array(owner))
    const envelop = await signedEnvelopFrom({ request, signer })
    const res = (await this.queryClient.listAccounts(envelop))
      .getAccountsList()
      .map(account => {
        return {
          id: Hex.fromUint8Array(account.getId()),
          owner: Base64.fromUint8Array(account.getOwner()),
          name: account.getName(),
          publicName: account.getPublicName(),
          profileImageUrl: account.getProfileImageUrl(),
        }
      })

      return res
  }

  async updateAccount({
    id = required('id'),
    name,
    publicName,
    profileImageUrl,
    signer,
    contextId,
  }) {
    Hex.verify(id)

    const builder = new DocumentUpdateBuilder(new model.Account())
    builder.setId(Hex.toUint8Array(id))

    if (name != null) {
      builder.setName(name)
      builder.addMask('name')
    }
    if (publicName != null) {
      builder.setPublicName(publicName)
      builder.addMask('public_name')
    }
    if (profileImageUrl != null) {
      builder.setProfileImageUrl(profileImageUrl)
      builder.addMask('profile_image_url')
    }

    const request = updateTransactionFrom(builder)
    const envelop = await signedEnvelopFrom({ request, signer, contextId })
    const response = await this.txClient.createTransaction(envelop)
    if (response.hasError()) {
      throw response.getError()
    }
    return response.getTxId()
  }

  // Transfers

  async createTransfer({
    fromAccountId = required('fromAccountId'),
    toAccountId = required('toAccountId'),
    amount = required('amount'),
    memo,
    contract,
    signer,
    contextId,
  }) {
    Hex.verify(fromAccountId)
    Hex.verify(toAccountId)

    // TODO @sadroeck: Make multi-transfer aware
    const step = new Transaction.TransferStep()
    step.setFromAccountId(Hex.toUint8Array(fromAccountId))
    step.setToAccountId(Hex.toUint8Array(toAccountId))
    step.setAmount(amount)
    if (memo != null) {
      step.addMetadata(memoMetadata(memo))
    }
    if (contract != null) {
      contextId = contractId(contract)
      step.addMetadata(contractMetadata(contract))
    }

    const transfer = new Transaction.CreateTransfer()
    transfer.setTransferStepsList([step])
    const request = createTransferFrom(transfer)
    const envelop = await signedEnvelopFrom({ request, signer, contextId })
    const res = await this.txClient.createTransaction(envelop)
    return res.getTxId()
  }

  async getTransfer({ id = required('id'), signer }) {
    Hex.verify(id)

    const request = new Transaction.GetTransferRequest()
    request.setTxId(id)
    const envelop = await signedEnvelopFrom({ request, signer })
    const tx = await this.queryClient.getTransfer(envelop)
    return parseTransfer(tx)
  }

  async _enhanceTransfer({ transfer = required('transfer'), signer }) {
    const from = await this.getAccountInfo({ id: transfer.fromAccountId, signer })
    const fromBank = from.parentId
      ? await this.getAccountInfo({ id: from.parentId, signer })
      : null
    const to = await this.getAccountInfo({ id: transfer.toAccountId, signer })
    const toBank = to.parentId ? await this.getAccountInfo({ id: to.parentId, signer }) : null

    return {
      to,
      from,
      fromBank,
      toBank
    }
  }

  async getEnhancedTransfer({ id = required('id'), signer }) {
    const transfer = await this.getTransfer({ id, signer })
    const enhanced = await this._enhanceTransfer({ transfer, signer })
    return transferToObject({ transfer, ...enhanced })
  }

  async listTransfers({
    accountId,
    contextId,
    minTxId = 0,
    limit = 10,
    includeChildAccounts = false,
    signer,
  }) {
    const request = new Transaction.ListTransferRequest()

    const filterCount = (contextId ? 1 : 0) + (accountId ? 1 : 0)
    if (filterCount !== 1) {
      throw new Error('Invalid filter')
    }

    if (accountId != null) {
      Hex.verify(accountId)
      request.setAccountId(Hex.toUint8Array(accountId))
    }
    if (contextId != null) {
      Hex.verify(contextId)
      request.setContextId(Hex.toUint8Array(contextId))
    }

    request.setMinTxId(minTxId)
    request.setLimit(limit)
    request.setIncludeChildAccounts(includeChildAccounts)

    const envelop = await signedEnvelopFrom({ request, signer })
    const transfers = await this.queryClient.listTransfers(envelop)
    return transfers.getTransfersList().map(parseTransfer)
  }

  async listEnhancedTransfers({
    accountId,
    contextId,
    minTxId = 0,
    limit = 10,
    includeChildAccounts = false,
    signer,
  }) {
    const transfers = await this.listTransfers({ accountId, contextId, minTxId, limit, includeChildAccounts, signer })
    return await Promise.all(transfers.map(async transfer => {
      const enhanced = await this._enhanceTransfer({ transfer, signer })
      return transferToObject({ transfer, ...enhanced })
    }))
  }

  // Actions

  async invokeAction({
    name = required('name'),
    fromAccountId = required('fromAccountId'),
    payload = required('payload'),
    targetAccountId,
    contextId,
    signer,
  }) {
    Hex.verify(fromAccountId)
    if (targetAccountId != null) {
      Hex.verify(targetAccountId)
    }

    const action = new Transaction.InvokeAction()

    action.setName(name)
    action.setFromAccount(Hex.toUint8Array(fromAccountId))
    action.setPayload(payload)

    if (targetAccountId != null) {
      const target = new Transaction.Target()
      target.setAccountId(Hex.toUint8Array(targetAccountId))
      action.setTarget(target)
    } else {
      throw new Error('Missing action target')
    }

    const request = actionRequestFrom(action)
    const envelop = await signedEnvelopFrom({ request, signer, contextId })
    const response = await this.txClient.createTransaction(envelop)
    const error = response.getError()
    return {
      txId: response.getTxId(),
      error: error ? `${error.message} (code ${error.code})` : null
    }
  }

  async getAction({ id = required('id'), signer }) {
    Hex.verify(id)

    const request = new Transaction.GetActionRequest()
    request.setTxId(id)
    const envelop = await signedEnvelopFrom({ request, signer })
    const res = await this.queryClient.getAction(envelop)

    return parseAction(res)
  }

  async listActions({
    name = required('name'),
    accountId = required('accountId'),
    minTxId = 0,
    maxTxId = 0,
    limit = 10,
    signer,
  }) {
    Hex.verify(accountId)

    const request = new Transaction.ListActionsRequest()

    request.setName(name)
    request.setAccountId(Hex.toUint8Array(accountId))
    request.setMinTxId(minTxId)
    request.setMaxTxId(maxTxId)
    request.setLimit(limit)

    const envelop = await signedEnvelopFrom({ request, signer })
    const actions = await this.queryClient.listActions(envelop)
    return actions.getActionsList().map(parseAction)
  }

  // Context
  async listTransactions({
    contextId = required('contextId'),
    minTxId = 0,
    maxTxId = 0,
    limit = 10,
    signer,
  }) {
    const request = new Queries.ListTransactionsRequest()

    request.setContextId(contextId)
    request.setMinTxId(minTxId)
    request.setMaxTxId(maxTxId)
    request.setLimit(limit)

    const envelop = await signedEnvelopFrom({ request, signer })
    const transactions = await this.queryClient.listTransactions(envelop)
    return transactions.getTransactionsList().map(parseTransaction)
  }

  async groupTransactions({
    accountId = required('accountId'),
    minTxId = 0,
    maxTxId = 0,
    limitGroups = 10,
    signer,
  }) {
    Hex.verify(accountId)

    const request = new Queries.GroupTransactionsRequest()

    if (accountId != null) {
      request.setAccountId(Hex.toUint8Array(accountId))
    }
    request.setMinTxId(minTxId)
    request.setMaxTxId(maxTxId)
    request.setLimitGroups(limitGroups)

    const envelop = await signedEnvelopFrom({ request, signer })
    const response = await this.queryClient.groupTransactions(envelop)
    return response.getGroupsList().map(group => group.getTransactionsList().map(parseTransaction))
  }

  // Accounts

  async getLedgerAccount({ accountId = required('accountId'), signer }) {
    Hex.verify(accountId)

    const request = new Transaction.GetAccountRequest()
    request.setId(Hex.toUint8Array(accountId))

    const envelop = await signedEnvelopFrom({ request, signer })
    const account = await this.queryClient.getIndexedAccount(envelop)
    const ledgerAccount = {
      id: Hex.fromUint8Array(account.getId()),
      balance: account.getBalance(),
      frozen: account.getFrozen(),
    }
    if (account.hasIssuance()) {
      const issuance = account.getIssuance()
      ledgerAccount.issuance = {
        issuedBalance: issuance.getIssuedBalance(),
        leafChildren: issuance.getLeafChildren(),
        nonLeafChildren: issuance.getNonLeafChildren(),
      }
    }
    return ledgerAccount
  }

  async createLedgerAccount({ parentId = required('parentId'), signer, contextId }) {
    Hex.verify(parentId)

    const txn = new Transaction.CreateLedgerAccount()
    txn.setParentId(Hex.toUint8Array(parentId))

    const request = createLedgerAccountRequestFrom(txn)
    const envelop = await signedEnvelopFrom({ request, signer, contextId })
    const response = await this.txClient.createTransaction(envelop)
    return Hex.fromUint8Array(response.getAccountCreated_asU8())
  }

  // User

  async createUser({ signer, contextId }) {
    const id = []
    uuidv4(null, id)

    const user = new model.AccountSet()
    user.setId(new Uint8Array(id))

    user.setOwner(new Uint8Array(await signer.getPublicKey()))

    const request = createTransactionFrom(user)
    const envelop = await signedEnvelopFrom({ request, signer, contextId })
    await this.txClient.createTransaction(envelop)

    return uuidStringify(id)
  }

  async getUser({ id = required('id'), signer }) {
    Uuid.verify(id)

    const request = new query.GetAccountSetRequest()
    request.setId(uuidParse(id))
    const envelop = await signedEnvelopFrom({ request, signer })
    const accountSet = await this.queryClient.getAccountSet(envelop)

    const accountsList = accountSet.getAccountsList().map(accountRef => {
      return (
        accountRef.getLedgerId() +
        '/' +
        Hex.fromUint8Array(accountRef.getAccountId())
      )
    })
    return {
      id: id,
      owner: Base64.fromUint8Array(accountSet.getOwner()),
      accountsList: accountsList,
    }
  }

  // accountsList is expected to be a list of fully qualified account ids:
  // ['usd.m10/00800001800003000000000100000005']
  async updateUser({ id = required('id'), accountsList, signer, contextId }) {
    Uuid.verify(id)

    const builder = new DocumentUpdateBuilder(new model.AccountSet())
    builder.setId(uuidParse(id))

    if (Array.isArray(accountsList) && accountsList.length > 0) {
      builder.setAccountsList(
        accountsList.map(accountId => {
          const split = accountId.split('/')
          const accountRef = new model.AccountRef()
          accountRef.setLedgerId(split[0])
          accountRef.setAccountId(Hex.toUint8Array(split[1]))
          return accountRef
        })
      )
      builder.addMask('accounts')
    }

    const request = updateTransactionFrom(builder)
    const envelop = await signedEnvelopFrom({ request, signer, contextId })
    await this.txClient.createTransaction(envelop)
    return this.getUser({ id, signer })
  }

  async deleteUser({ id = required('id'), signer, contextId }) {
    Uuid.verify(id)

    const accountSet = new model.AccountSet()
    accountSet.setId(uuidParse(id))
    const entry = await this.getUser({ id, signer })

    const request = deleteTransactionFrom(accountSet)
    const envelop = await signedEnvelopFrom({ request, signer, contextId })
    await this.txClient.createTransaction(envelop)
    return entry
  }

  async listRoleBinding({ name = required('name'), signer }) {
    const request = new query.ListRoleBindingsRequest()
    request.setName(name)
    const envelop = await signedEnvelopFrom({ request, signer })
    const response = await this.queryClient.listRoleBindings(envelop)

    const roleBindings = response.getRoleBindingsList().map(roleBinding => {
      return {
        id: Uuid.fromUint8Array(roleBinding.getId()),
        name: roleBinding.getName(),
        subjects: roleBinding.getSubjectsList(),
      }
    })
    return roleBindings
  }

  async updateRoleBinding({ id = required('id'), subjects, signer, contextId }) {
    Uuid.verify(id)

    const builder = new DocumentUpdateBuilder(new rbac.RoleBinding())
    builder.setId(uuidParse(id))

    if (Array.isArray(subjects) && subjects.length > 0) {
      builder.setSubjectsList(subjects.map(subject => new Uint8Array(subject)))
      builder.addMask('subjects')
    }

    const request = updateTransactionFrom(builder)
    const envelop = await signedEnvelopFrom({ request, signer, contextId })
    await this.txClient.createTransaction(envelop)
  }

    // Observations

    async observeTransfers({
      involvesAccounts = required('involvesAccounts'),
      startingFrom,
      signer,
    }, options = {}) {
      const request = new observe.Transfers()
      request.setInvolvedAccountsList(involvesAccounts.map(Hex.toUint8Array))
      if (startingFrom != null) {
        const txId = new TxId()
        txId.setTxId(startingFrom)
        request.setStartingFrom(txId)
      }
      const envelop = await signedEnvelopFrom({ request, signer })
      const stream = await this.queryClient.observeTransfers(envelop, options)

      stream.on('error', err => {
        // Ignore connection CANCELED (1) events
        if (err.code !== 1) {
          console.error(err)
        }
      })
      const resultStream = stream.pipe(transferResultStream())
      resultStream.on('close', () => {
        setImmediate(() => stream.cancel())
        stream.destroy()
      })
      return resultStream
    }

    async observeResources({
      filter = required('filter'),
      collection = required('collection'),
      variables,
      startingFrom,
      signer,
    }, options = {}) {
      const request = new observe.Resources()
      request.setExpression(createExpression(filter, variables))
      request.setCollection(collection)

      if (startingFrom != null) {
        const txId = new TxId()
        txId.setTxId(startingFrom)
        request.setStartingFrom(txId)
      }
      const envelop = await signedEnvelopFrom({ request, signer })
      const stream = await this.queryClient.observeResources(envelop, options)

      stream.on('error', err => {
        // Ignore connection CANCELED (1) events
        if (err.code !== 1) {
          console.error(err)
        }
      })
      const resultStream = stream.pipe(resourceResultStream())
      resultStream.on('close', () => {
        setImmediate(() => stream.cancel())
        stream.destroy()
      })
      return resultStream
    }

    async observeActions({
      name = required('name'),
      involvesAccounts = required('involvesAccounts'),
      startingFrom,
      signer,
    }, options = {}) {
      const request = new observe.Actions()
      request.setName(name)
      request.setInvolvedAccountsList(involvesAccounts.map(Hex.toUint8Array))
      if (startingFrom != null) {
        const txId = new TxId()
        txId.setTxId(startingFrom)
        request.setStartingFrom(txId)
      }
      const envelop = await signedEnvelopFrom({ request, signer })
      const stream = await this.queryClient.observeActions(envelop, options)

      stream.on('error', err => {
        // Ignore connection CANCELED (1) events
        if (err.code !== 1) {
          console.error(err)
        }
      })
      const resultStream = stream.pipe(actionStream())
      resultStream.on('close', () => {
        setImmediate(() => stream.cancel())
        stream.destroy()
      })
      return resultStream
    }
}

const transferToObject = ({ transfer, from, to, fromBank, toBank }) => (
  {
    senderName: from.publicName,
    senderBankName: fromBank?.publicName,
    senderProfileImageUrl: from.profileImageUrl,
    receiverName: to.publicName,
    receiverBankName: toBank?.publicName,
    receiverProfileImageUrl: to.profileImageUrl,
    ...transfer,
  })

const transferResultStream = () => new Transform({
  objectMode: true,
  transform: (message, _, callback) => {
    const transactions = message.getTransactionsList().map(transaction => {
      const request = transaction.getRequest()
      const transfer = request.getData().getTransfer()
      // TODO: Make Multi-Transfer aware
      const transferStep = transfer.getTransferStepsList()[0]
      const response = transaction.getResponse()
      const error = response.getError()
      return {
        txId: response.getTxId(),
        fromAccountId: Hex.fromUint8Array(transferStep.getFromAccountId()),
        toAccountId: Hex.fromUint8Array(transferStep.getToAccountId()),
        amount: transferStep.getAmount(),
        contextId: request.getContextId(),
        metadata: parseMetadata(transferStep.getMetadataList()),
        error: error != null ? `${error.message} (code ${error.code})` : null,
      }
    })
    callback(null, transactions)
  },
  flush: callback => callback(),
})

const resourceResultStream = () => new Transform({
  objectMode: true,
  transform: (message, _, callback) => {
    const transactions = message.getTransactionsList().map(transaction => {
      const request = transaction.getRequest()
      const operations = request.getData().getDocumentOperations()
      const response = transaction.getResponse()
      const error = response.getError()
      return {
        txId: response.getTxId(),
        // TODO: Convert me to a usable model??
        operations: operations.getOperationsList(),
        contextId: request.getContextId(),
        error: error != null ? `${error.message} (code ${error.code})` : null,
      }
    })
    callback(null, transactions)
  },
  flush: callback => callback(),
})

const actionStream = () => new Transform({
  objectMode: true,
  transform: (message, _, callback) => {
    const transactions = message.getTransactionsList().map(transaction => {
      const request = transaction.getRequest()
      const action = request.getData().getInvokeAction()
      const target = action.getTarget()
      const response = transaction.getResponse()
      return {
        txId: response.getTxId(),
        name: action.getName(),
        fromAccountId: Hex.fromUint8Array(action.getFromAccount()),
        target: target?.hasAccountId() ? Hex.fromUint8Array(target.getAccountId()) : null,
        payload: action.getPayload(),
        contextId: request.getContextId()
      }
    })
    callback(null, transactions)
  },
  flush: callback => callback(),
})

// Tx transformations

const parseTransfer = tx => {
  // TODO: Make Multi-Transfer aware
  const transferStep = tx.getTransferStepsList()[0]
  return {
    txId: tx.getTxId(),
    fromAccountId: Hex.fromUint8Array(transferStep.getFromAccountId()),
    toAccountId: Hex.fromUint8Array(transferStep.getToAccountId()),
    amount: transferStep.getAmount(),
    failed: tx.hasError(),
    contextId: tx.getContextId(),
    metadata: parseMetadata(transferStep.getMetadataList()),
    timestamp: new Date(tx.getTimestamp() / 1000),
  }
}

const parseCreateTransfer = ({ transfer, contextId }) => {
  // TODO: Make Multi-Transfer aware
  const transferStep = transfer.getTransferStepsList()[0]
  return {
    fromAccountId: Hex.fromUint8Array(transferStep.getFromAccountId()),
    toAccountId: Hex.fromUint8Array(transferStep.getToAccountId()),
    amount: transferStep.getAmount(),
    metadata: parseMetadata(transferStep.getMetadataList()),
  }
}

const parseAction = tx => {
  const target = tx.getTarget()
  return {
    txId: tx.getTxId(),
    name: tx.getName(),
    fromAccountId: Hex.fromUint8Array(tx.getFromAccount()),
    target: target?.hasAccountId() ? Hex.fromUint8Array(target.getAccountId()) : null,
    payload: tx.getPayload(),
    contextId: tx.getContextId()
  }
}

const parseInvokeAction = tx => {
  const target = tx.getTarget()
  return {
    name: tx.getName(),
    fromAccountId: Hex.fromUint8Array(tx.getFromAccount()),
    target: target?.hasAccountId() ? Hex.fromUint8Array(target.getAccountId()) : null,
    payload: tx.getPayload(),
  }
}

const parseTransaction = transaction => {
  const request = transaction.getRequest()
  const data = request.getData()
  const response = transaction.getResponse()
  const error = response.getError()
  return {
    txId: response.getTxId(),
    timestamp: new Date(response.getTimestamp() / 1000),
    nonce: request.getNonce(),
    contextId: request.getContextId(),
    error: (error != null) ? `${error.message} (code ${error.code})` : null,
    transfer: (data.getTransfer() != null)
      ? parseCreateTransfer({ transfer: data.getTransfer(), contextId: request.getContextId() })
      : null,
    action: (data.getInvokeAction() != null) ? parseInvokeAction(data.getInvokeAction()) : null,
    accountCreated: (response.getAccountCreated() != null) ? Hex.fromUint8Array(response.getAccountCreated()) : null,
    accountFrozen: (data.getSetFreezeState() != null) ? parseSetFrozen(data.getSetFreezeState()) : null,
    documentOperations: data.getDocumentOperations(),
  }
}

const parseSetFrozen = tx => ({
  accountId: Hex.fromUint8Array(tx.getAccountId()),
  isFrozen: tx.getFrozen(),
})

module.exports = LedgerClient
