const makeServerStreamRequest = function(client, originalFunction, options = {}) {
  return function(content = {}, { timeout } = {}) {
    const requestTimeout = options.timeout ?? timeout
    const deadline = requestTimeout != null ? Date.now() + requestTimeout : null
    return originalFunction.call(client, content, options.metadata, { deadline: deadline })
  }
}

module.exports = makeServerStreamRequest
