// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var directory_api_pb = require('../directory/api_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');

function serialize_google_protobuf_Empty(arg) {
  if (!(arg instanceof google_protobuf_empty_pb.Empty)) {
    throw new Error('Expected argument of type google.protobuf.Empty');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_google_protobuf_Empty(buffer_arg) {
  return google_protobuf_empty_pb.Empty.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_directory_Alias(arg) {
  if (!(arg instanceof directory_api_pb.Alias)) {
    throw new Error('Expected argument of type m10.directory.Alias');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_directory_Alias(buffer_arg) {
  return directory_api_pb.Alias.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_directory_CheckAliasRequest(arg) {
  if (!(arg instanceof directory_api_pb.CheckAliasRequest)) {
    throw new Error('Expected argument of type m10.directory.CheckAliasRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_directory_CheckAliasRequest(buffer_arg) {
  return directory_api_pb.CheckAliasRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_directory_CreateImageUrlRequest(arg) {
  if (!(arg instanceof directory_api_pb.CreateImageUrlRequest)) {
    throw new Error('Expected argument of type m10.directory.CreateImageUrlRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_directory_CreateImageUrlRequest(buffer_arg) {
  return directory_api_pb.CreateImageUrlRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_directory_GetObjectUrlRequest(arg) {
  if (!(arg instanceof directory_api_pb.GetObjectUrlRequest)) {
    throw new Error('Expected argument of type m10.directory.GetObjectUrlRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_directory_GetObjectUrlRequest(buffer_arg) {
  return directory_api_pb.GetObjectUrlRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_directory_Ledger(arg) {
  if (!(arg instanceof directory_api_pb.Ledger)) {
    throw new Error('Expected argument of type m10.directory.Ledger');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_directory_Ledger(buffer_arg) {
  return directory_api_pb.Ledger.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_directory_ListLedgersResponse(arg) {
  if (!(arg instanceof directory_api_pb.ListLedgersResponse)) {
    throw new Error('Expected argument of type m10.directory.ListLedgersResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_directory_ListLedgersResponse(buffer_arg) {
  return directory_api_pb.ListLedgersResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_directory_ObjectUrlResponse(arg) {
  if (!(arg instanceof directory_api_pb.ObjectUrlResponse)) {
    throw new Error('Expected argument of type m10.directory.ObjectUrlResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_directory_ObjectUrlResponse(buffer_arg) {
  return directory_api_pb.ObjectUrlResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_directory_SearchAliasesRequest(arg) {
  if (!(arg instanceof directory_api_pb.SearchAliasesRequest)) {
    throw new Error('Expected argument of type m10.directory.SearchAliasesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_directory_SearchAliasesRequest(buffer_arg) {
  return directory_api_pb.SearchAliasesRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_directory_SearchAliasesResponse(arg) {
  if (!(arg instanceof directory_api_pb.SearchAliasesResponse)) {
    throw new Error('Expected argument of type m10.directory.SearchAliasesResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_directory_SearchAliasesResponse(buffer_arg) {
  return directory_api_pb.SearchAliasesResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


// Follows the Google Standard Methods API Guide: https://cloud.google.com/apis/design/standard_methods
// Method / Request  / Response
// List   / N/A      / Resource* list
// Get    / N/A      / Resource*
// Create / Resource / Resource*
// Update / Resource / Resource*
// Delete / N/A      / google.protobuf.Empty**
// *can be partial resource objects if using field masks in request
// **empty if removed immeadiately, otherwise return long-running job id or the remaining resource
//
var DirectoryServiceService = exports.DirectoryServiceService = {
  // Ledgers
createLedger: {
    path: '/m10.directory.DirectoryService/CreateLedger',
    requestStream: false,
    responseStream: false,
    requestType: directory_api_pb.Ledger,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_m10_directory_Ledger,
    requestDeserialize: deserialize_m10_directory_Ledger,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  listLedgers: {
    path: '/m10.directory.DirectoryService/ListLedgers',
    requestStream: false,
    responseStream: false,
    requestType: google_protobuf_empty_pb.Empty,
    responseType: directory_api_pb.ListLedgersResponse,
    requestSerialize: serialize_google_protobuf_Empty,
    requestDeserialize: deserialize_google_protobuf_Empty,
    responseSerialize: serialize_m10_directory_ListLedgersResponse,
    responseDeserialize: deserialize_m10_directory_ListLedgersResponse,
  },
  // Aliases
checkAlias: {
    path: '/m10.directory.DirectoryService/CheckAlias',
    requestStream: false,
    responseStream: false,
    requestType: directory_api_pb.CheckAliasRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_m10_directory_CheckAliasRequest,
    requestDeserialize: deserialize_m10_directory_CheckAliasRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  createAlias: {
    path: '/m10.directory.DirectoryService/CreateAlias',
    requestStream: false,
    responseStream: false,
    requestType: directory_api_pb.Alias,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_m10_directory_Alias,
    requestDeserialize: deserialize_m10_directory_Alias,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  searchAliases: {
    path: '/m10.directory.DirectoryService/SearchAliases',
    requestStream: false,
    responseStream: false,
    requestType: directory_api_pb.SearchAliasesRequest,
    responseType: directory_api_pb.SearchAliasesResponse,
    requestSerialize: serialize_m10_directory_SearchAliasesRequest,
    requestDeserialize: deserialize_m10_directory_SearchAliasesRequest,
    responseSerialize: serialize_m10_directory_SearchAliasesResponse,
    responseDeserialize: deserialize_m10_directory_SearchAliasesResponse,
  },
  //  Transaction Support Documents
createObjectUrl: {
    path: '/m10.directory.DirectoryService/CreateObjectUrl',
    requestStream: false,
    responseStream: false,
    requestType: google_protobuf_empty_pb.Empty,
    responseType: directory_api_pb.ObjectUrlResponse,
    requestSerialize: serialize_google_protobuf_Empty,
    requestDeserialize: deserialize_google_protobuf_Empty,
    responseSerialize: serialize_m10_directory_ObjectUrlResponse,
    responseDeserialize: deserialize_m10_directory_ObjectUrlResponse,
  },
  getObjectUrl: {
    path: '/m10.directory.DirectoryService/GetObjectUrl',
    requestStream: false,
    responseStream: false,
    requestType: directory_api_pb.GetObjectUrlRequest,
    responseType: directory_api_pb.ObjectUrlResponse,
    requestSerialize: serialize_m10_directory_GetObjectUrlRequest,
    requestDeserialize: deserialize_m10_directory_GetObjectUrlRequest,
    responseSerialize: serialize_m10_directory_ObjectUrlResponse,
    responseDeserialize: deserialize_m10_directory_ObjectUrlResponse,
  },
  createImageUrl: {
    path: '/m10.directory.DirectoryService/CreateImageUrl',
    requestStream: false,
    responseStream: false,
    requestType: directory_api_pb.CreateImageUrlRequest,
    responseType: directory_api_pb.ObjectUrlResponse,
    requestSerialize: serialize_m10_directory_CreateImageUrlRequest,
    requestDeserialize: deserialize_m10_directory_CreateImageUrlRequest,
    responseSerialize: serialize_m10_directory_ObjectUrlResponse,
    responseDeserialize: deserialize_m10_directory_ObjectUrlResponse,
  },
};

exports.DirectoryServiceClient = grpc.makeGenericClientConstructor(DirectoryServiceService);
