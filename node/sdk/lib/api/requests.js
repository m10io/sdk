const {
  Transaction,
  Model,
  ops,
  rbac,
  sdk,
} = require('../proto')
const { required } = require('../utils/utils')
Model.Account.prototype.collection = 'accounts'
Model.AccountSet.prototype.collection = 'account-sets'
rbac.RoleBinding.prototype.collection = 'role-bindings'
rbac.Role.prototype.collection = 'roles'

function createTransactionFrom(document) {
  const serializedDoc = document.serializeBinary()

  const setOp = new ops.Operation.InsertDocument()
  setOp.setCollection(document.collection)
  setOp.setDocument(serializedDoc)

  const op = new ops.Operation()
  op.setInsertDocument(setOp)

  const body = new ops.DocumentOperations()
  body.addOperations(op)

  const txRequest = new Transaction.TransactionRequestPayload()
  const data = new Transaction.TransactionData()
  data.setDocumentOperations(body)
  txRequest.setData(data)

  return txRequest
}

function updateTransactionFrom(builder) {
  const serializedDoc = builder.document.serializeBinary()

  const primaryKey = new ops.Value()
  primaryKey.setBytesValue(builder.document.getId())

  const updateOp = new ops.Operation.UpdateDocument()
  updateOp.setCollection(builder.document.collection)
  updateOp.setPrimaryKey(primaryKey)
  updateOp.setDocument(serializedDoc)
  updateOp.setFieldMask(builder.mask)

  const op = new ops.Operation()
  op.setUpdateDocument(updateOp)

  const body = new ops.DocumentOperations()
  body.addOperations(op)

  const txRequest = new Transaction.TransactionRequestPayload()
  const data = new Transaction.TransactionData()
  data.setDocumentOperations(body)
  txRequest.setData(data)

  return txRequest
}

function deleteTransactionFrom(document) {
  const primaryKey = new ops.Value()
  primaryKey.setBytesValue(document.getId())

  const deleteOp = new ops.Operation.DeleteDocument()
  deleteOp.setCollection(document.collection)
  deleteOp.setPrimaryKey(primaryKey)

  const op = new ops.Operation()
  op.setDeleteDocument(deleteOp)

  const body = new ops.DocumentOperations()
  body.addOperations(op)

  const txRequest = new Transaction.TransactionRequestPayload()
  const data = new Transaction.TransactionData()
  data.setDocumentOperations(body)
  txRequest.setData(data)

  return txRequest
}

function createTransferFrom(transfer) {
  const txRequest = new Transaction.TransactionRequestPayload()
  const data = new Transaction.TransactionData()
  data.setTransfer(transfer)
  txRequest.setData(data)
  return txRequest
}

function actionRequestFrom(action) {
  const txRequest = new Transaction.TransactionRequestPayload()
  const data = new Transaction.TransactionData()
  data.setInvokeAction(action)
  txRequest.setData(data)
  return txRequest
}

function createLedgerAccountRequestFrom(request) {
  const txRequest = new Transaction.TransactionRequestPayload()
  const data = new Transaction.TransactionData()
  data.setCreateLedgerAccount(request)
  txRequest.setData(data)
  return txRequest
}

async function signedEnvelopFrom({ request = required('request'), signer = required('signer'), contextId }) {
  if (request instanceof Transaction.TransactionRequestPayload) {
    request.setNonce(generateNonce())
    request.setTimestamp(Date.now() * 1000)
    if (contextId != null) {
      request.setContextId(contextId)
    }
  }
  const envelop = requestEnvelopFrom(request)

  await envelop.sign(signer)
  return envelop
}

function generateNonce() {
  return Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
}

function requestEnvelopFrom(request) {
  const payload = request.serializeBinary()
  const envelop = new sdk.RequestEnvelope()
  envelop.setPayload(payload)
  return envelop
}

sdk.RequestEnvelope.prototype.sign = async function(signer) {
  const signature = new sdk.Signature()
  signature.setAlgorithm(signer.getAlgorithm())
  signature.setPublicKey(new Uint8Array(await signer.getPublicKey()))
  signature.setSignature(
    new Uint8Array(await signer.sign(this.getPayload().buffer))
  )
  this.setSignature(signature)
}

function createExpression(query, variables) {
  const expr = new ops.Exp()
  expr.setExp(query)
  const varMap = expr.getVarsMap()
  if (variables) {
    variables.forEach(({ name, value }) => varMap.set(name, toProtoValue(value)))
  }
  return expr
}

function toProtoValue(val) {
  const protoValue = new ops.Value()
  switch (typeof val)  {
    case 'string':
      protoValue.setStringValue(val)
      break
    case 'number':
      if (val.isInteger()) {
        protoValue.setInt32(val)
      } else {
        protoValue.setDoubleValue(val)
      }
      break
    case 'bigint':
      protoValue.setUint64Value(val)
      break
    case 'boolean':
      protoValue.setBoolBlue(val)
      break
    case 'object':
      if (val instanceof Uint8Array) {
        protoValue.setBytesValue(val)
      }
      break
  }
  return protoValue
}

module.exports = {
  actionRequestFrom,
  createTransactionFrom,
  createExpression,
  deleteTransactionFrom,
  updateTransactionFrom,
  createTransferFrom,
  createLedgerAccountRequestFrom,
  requestEnvelopFrom,
  signedEnvelopFrom,
}
