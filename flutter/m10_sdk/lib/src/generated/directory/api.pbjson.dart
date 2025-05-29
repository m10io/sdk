//
//  Generated code. Do not modify.
//  source: directory/api.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use ledgerDescriptor instead')
const Ledger$json = {
  '1': 'Ledger',
  '2': [
    {'1': 'operator', '3': 1, '4': 1, '5': 9, '10': 'operator'},
    {'1': 'url', '3': 2, '4': 1, '5': 9, '10': 'url'},
  ],
  '9': [
    {'1': 4, '2': 5},
    {'1': 5, '2': 6},
  ],
};

/// Descriptor for `Ledger`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List ledgerDescriptor = $convert.base64Decode(
    'CgZMZWRnZXISGgoIb3BlcmF0b3IYASABKAlSCG9wZXJhdG9yEhAKA3VybBgCIAEoCVIDdXJsSg'
    'QIBBAFSgQIBRAG');

@$core.Deprecated('Use listLedgersResponseDescriptor instead')
const ListLedgersResponse$json = {
  '1': 'ListLedgersResponse',
  '2': [
    {'1': 'ledgers', '3': 1, '4': 3, '5': 11, '6': '.m10.directory.Ledger', '10': 'ledgers'},
  ],
};

/// Descriptor for `ListLedgersResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listLedgersResponseDescriptor = $convert.base64Decode(
    'ChNMaXN0TGVkZ2Vyc1Jlc3BvbnNlEi8KB2xlZGdlcnMYASADKAsyFS5tMTAuZGlyZWN0b3J5Lk'
    'xlZGdlclIHbGVkZ2Vycw==');

@$core.Deprecated('Use aliasDescriptor instead')
const Alias$json = {
  '1': 'Alias',
  '2': [
    {'1': 'handle', '3': 1, '4': 1, '5': 9, '10': 'handle'},
    {'1': 'display_name', '3': 2, '4': 1, '5': 9, '10': 'displayName'},
    {'1': 'account_set_id', '3': 5, '4': 1, '5': 12, '10': 'accountSetId'},
    {'1': 'operator', '3': 8, '4': 1, '5': 9, '10': 'operator'},
    {'1': 'alias_type', '3': 10, '4': 1, '5': 14, '6': '.m10.directory.Alias.Type', '10': 'aliasType'},
  ],
  '4': [Alias_Type$json],
  '9': [
    {'1': 9, '2': 10},
  ],
};

@$core.Deprecated('Use aliasDescriptor instead')
const Alias_Type$json = {
  '1': 'Type',
  '2': [
    {'1': 'HANDLE', '2': 0},
    {'1': 'EMAIL', '2': 1},
    {'1': 'PHONE', '2': 2},
  ],
};

/// Descriptor for `Alias`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List aliasDescriptor = $convert.base64Decode(
    'CgVBbGlhcxIWCgZoYW5kbGUYASABKAlSBmhhbmRsZRIhCgxkaXNwbGF5X25hbWUYAiABKAlSC2'
    'Rpc3BsYXlOYW1lEiQKDmFjY291bnRfc2V0X2lkGAUgASgMUgxhY2NvdW50U2V0SWQSGgoIb3Bl'
    'cmF0b3IYCCABKAlSCG9wZXJhdG9yEjgKCmFsaWFzX3R5cGUYCiABKA4yGS5tMTAuZGlyZWN0b3'
    'J5LkFsaWFzLlR5cGVSCWFsaWFzVHlwZSIoCgRUeXBlEgoKBkhBTkRMRRAAEgkKBUVNQUlMEAES'
    'CQoFUEhPTkUQAkoECAkQCg==');

@$core.Deprecated('Use checkAliasRequestDescriptor instead')
const CheckAliasRequest$json = {
  '1': 'CheckAliasRequest',
  '2': [
    {'1': 'handle', '3': 1, '4': 1, '5': 9, '10': 'handle'},
  ],
};

/// Descriptor for `CheckAliasRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List checkAliasRequestDescriptor = $convert.base64Decode(
    'ChFDaGVja0FsaWFzUmVxdWVzdBIWCgZoYW5kbGUYASABKAlSBmhhbmRsZQ==');

@$core.Deprecated('Use searchAliasesRequestDescriptor instead')
const SearchAliasesRequest$json = {
  '1': 'SearchAliasesRequest',
  '2': [
    {'1': 'handle_prefix', '3': 1, '4': 1, '5': 9, '10': 'handlePrefix'},
    {'1': 'page_size', '3': 2, '4': 1, '5': 5, '10': 'pageSize'},
    {'1': 'page_token', '3': 3, '4': 1, '5': 9, '10': 'pageToken'},
    {'1': 'subject', '3': 5, '4': 1, '5': 9, '10': 'subject'},
  ],
};

/// Descriptor for `SearchAliasesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List searchAliasesRequestDescriptor = $convert.base64Decode(
    'ChRTZWFyY2hBbGlhc2VzUmVxdWVzdBIjCg1oYW5kbGVfcHJlZml4GAEgASgJUgxoYW5kbGVQcm'
    'VmaXgSGwoJcGFnZV9zaXplGAIgASgFUghwYWdlU2l6ZRIdCgpwYWdlX3Rva2VuGAMgASgJUglw'
    'YWdlVG9rZW4SGAoHc3ViamVjdBgFIAEoCVIHc3ViamVjdA==');

@$core.Deprecated('Use searchAliasesResponseDescriptor instead')
const SearchAliasesResponse$json = {
  '1': 'SearchAliasesResponse',
  '2': [
    {'1': 'aliases', '3': 1, '4': 3, '5': 11, '6': '.m10.directory.Alias', '10': 'aliases'},
    {'1': 'next_page_token', '3': 2, '4': 1, '5': 9, '10': 'nextPageToken'},
  ],
};

/// Descriptor for `SearchAliasesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List searchAliasesResponseDescriptor = $convert.base64Decode(
    'ChVTZWFyY2hBbGlhc2VzUmVzcG9uc2USLgoHYWxpYXNlcxgBIAMoCzIULm0xMC5kaXJlY3Rvcn'
    'kuQWxpYXNSB2FsaWFzZXMSJgoPbmV4dF9wYWdlX3Rva2VuGAIgASgJUg1uZXh0UGFnZVRva2Vu');

@$core.Deprecated('Use getObjectUrlRequestDescriptor instead')
const GetObjectUrlRequest$json = {
  '1': 'GetObjectUrlRequest',
  '2': [
    {'1': 'object_id', '3': 1, '4': 1, '5': 9, '10': 'objectId'},
  ],
};

/// Descriptor for `GetObjectUrlRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getObjectUrlRequestDescriptor = $convert.base64Decode(
    'ChNHZXRPYmplY3RVcmxSZXF1ZXN0EhsKCW9iamVjdF9pZBgBIAEoCVIIb2JqZWN0SWQ=');

@$core.Deprecated('Use objectUrlResponseDescriptor instead')
const ObjectUrlResponse$json = {
  '1': 'ObjectUrlResponse',
  '2': [
    {'1': 'presigned_url', '3': 1, '4': 1, '5': 9, '10': 'presignedUrl'},
    {'1': 'object_id', '3': 2, '4': 1, '5': 9, '10': 'objectId'},
    {'1': 'url', '3': 3, '4': 1, '5': 9, '10': 'url'},
  ],
};

/// Descriptor for `ObjectUrlResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List objectUrlResponseDescriptor = $convert.base64Decode(
    'ChFPYmplY3RVcmxSZXNwb25zZRIjCg1wcmVzaWduZWRfdXJsGAEgASgJUgxwcmVzaWduZWRVcm'
    'wSGwoJb2JqZWN0X2lkGAIgASgJUghvYmplY3RJZBIQCgN1cmwYAyABKAlSA3VybA==');

@$core.Deprecated('Use createImageUrlRequestDescriptor instead')
const CreateImageUrlRequest$json = {
  '1': 'CreateImageUrlRequest',
  '2': [
    {'1': 'mime_type', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'mimeType', '17': true},
  ],
  '8': [
    {'1': '_mime_type'},
  ],
};

/// Descriptor for `CreateImageUrlRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createImageUrlRequestDescriptor = $convert.base64Decode(
    'ChVDcmVhdGVJbWFnZVVybFJlcXVlc3QSIAoJbWltZV90eXBlGAEgASgJSABSCG1pbWVUeXBliA'
    'EBQgwKCl9taW1lX3R5cGU=');

