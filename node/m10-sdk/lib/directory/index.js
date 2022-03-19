const { credentials } = require('@grpc/grpc-js')
const {
  SearchAliasesRequest,
  CreateObjectUrlRequest,
  GetObjectUrlRequest
} = require('../generated/directory/api_pb')
const {
  DirectoryServiceClient,
} = require('../generated/directory/api_grpc_pb')
const promisifyAll = require('../utils/grpc/promisify')
const { required, Uuid } = require('../utils/utils')
const { Empty } = require('../proto')

exports.DirectoryClient = class DirectoryClient {
  constructor(address, secure) {
    const creds = secure
      ? credentials.createSsl()
      : credentials.createInsecure()
    this.client = promisifyAll(new DirectoryServiceClient(address, creds))
  }

  async searchAliases({
    handlePrefix = required('handlePrefix'),
    pageSize = 20,
    pageToken,
  }) {
    const request = new SearchAliasesRequest()
    request.setHandlePrefix(handlePrefix)
    request.setPageSize(pageSize)
    if (pageToken != null) {
      request.setPageToken(pageToken)
    }

    const response = await this.client.searchAliases(request)
    return response.toObject()
  }

  async listLedgers() {
    const request = new Empty()
    const response = await this.client.listLedgers(request)
    return response.toObject()
  }

  async createObjectUrl() {
    const request = new CreateObjectUrlRequest()
    const response = await this.client.createObjectUrl(request)
    return {
      url: response.getPresignedUrl(),
      objectId: Uuid.fromUint8Array(response.getObjectId())
    }
  }

  async getObjectUrl(objectId) {
    const request = new GetObjectUrlRequest()
    request.setObjectId(objectId)
    const response = await this.client.getObjectUrl(request)
    return {
      url: response.getPresignedUrl(),
      objectId: response.getObjectId()
    }
  }
}
