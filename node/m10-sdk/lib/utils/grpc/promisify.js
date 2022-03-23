const makeClientStreamRequest = require('./client-stream-request')
const makeServerStreamRequest = require('./server-stream-request')
const makeBidirectionalStreamRequest = require('./bidirectional-stream-request')
const { promisify } = require('util')

const promisifyAll = function(client, options) {
  Object.keys(Object.getPrototypeOf(client)).forEach(function(functionName) {
    const originalFunction = client[functionName]
    if (originalFunction.requestStream === undefined && originalFunction.responseStream === undefined) {
      return
    }

    const genericFunctionSelector =
      (originalFunction.requestStream ? 2 : 0) | (originalFunction.responseStream ? 1 : 0)
    let clientFunction
    switch (genericFunctionSelector) {
    case 0:
      clientFunction = (client, originalFunction) => promisify(originalFunction.bind(client))
      break
    case 1:
      clientFunction = makeServerStreamRequest
      break
    case 2:
      clientFunction = makeClientStreamRequest
      break
    case 3:
      clientFunction = makeBidirectionalStreamRequest
      break
    }

    client[functionName] = clientFunction(client, originalFunction, options)
  })
  return client
}

module.exports = promisifyAll
