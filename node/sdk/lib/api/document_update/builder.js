// eslint-disable-next-line camelcase
const google_protobuf_field_mask_pb = require('google-protobuf/google/protobuf/field_mask_pb.js')

exports.DocumentUpdateBuilder = class DocumentUpdateBuilder {
  constructor(document) {
    this.document = document
    this.mask = new google_protobuf_field_mask_pb.FieldMask()

    return new Proxy(this, {
      get: function(obj, prop) {
        if (typeof obj[prop] === 'function') {
          return function(...args) {
            return obj[prop].apply(obj, args)
          }
        } else if (typeof obj.document[prop] === 'function') {
          return function(...args) {
            return obj.document[prop].apply(obj.document, args)
          }
        } else {
          return obj[prop]
        }
      },
    })
  }

  addMask(path) {
    this.mask.addPaths(path)
  }
}
