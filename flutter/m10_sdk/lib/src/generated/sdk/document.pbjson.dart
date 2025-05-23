//
//  Generated code. Do not modify.
//  source: sdk/document.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use documentOperationsDescriptor instead')
const DocumentOperations$json = {
  '1': 'DocumentOperations',
  '2': [
    {'1': 'operations', '3': 2, '4': 3, '5': 11, '6': '.m10.sdk.Operation', '10': 'operations'},
  ],
};

/// Descriptor for `DocumentOperations`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List documentOperationsDescriptor = $convert.base64Decode(
    'ChJEb2N1bWVudE9wZXJhdGlvbnMSMgoKb3BlcmF0aW9ucxgCIAMoCzISLm0xMC5zZGsuT3Blcm'
    'F0aW9uUgpvcGVyYXRpb25z');

@$core.Deprecated('Use operationDescriptor instead')
const Operation$json = {
  '1': 'Operation',
  '2': [
    {'1': 'insert_document', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.Operation.InsertDocument', '9': 0, '10': 'insertDocument'},
    {'1': 'update_document', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.Operation.UpdateDocument', '9': 0, '10': 'updateDocument'},
    {'1': 'delete_document', '3': 3, '4': 1, '5': 11, '6': '.m10.sdk.Operation.DeleteDocument', '9': 0, '10': 'deleteDocument'},
    {'1': 'insert_collection', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.CollectionMetadata', '9': 0, '10': 'insertCollection'},
    {'1': 'insert_index', '3': 5, '4': 1, '5': 11, '6': '.m10.sdk.Operation.InsertIndex', '9': 0, '10': 'insertIndex'},
  ],
  '3': [Operation_InsertDocument$json, Operation_UpdateDocument$json, Operation_DeleteDocument$json, Operation_InsertIndex$json],
  '8': [
    {'1': 'operation'},
  ],
};

@$core.Deprecated('Use operationDescriptor instead')
const Operation_InsertDocument$json = {
  '1': 'InsertDocument',
  '2': [
    {'1': 'collection', '3': 1, '4': 1, '5': 9, '10': 'collection'},
    {'1': 'document', '3': 3, '4': 1, '5': 12, '10': 'document'},
  ],
};

@$core.Deprecated('Use operationDescriptor instead')
const Operation_UpdateDocument$json = {
  '1': 'UpdateDocument',
  '2': [
    {'1': 'collection', '3': 1, '4': 1, '5': 9, '10': 'collection'},
    {'1': 'primary_key', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.Value', '10': 'primaryKey'},
    {'1': 'document', '3': 3, '4': 1, '5': 12, '10': 'document'},
    {'1': 'field_mask', '3': 4, '4': 1, '5': 11, '6': '.google.protobuf.FieldMask', '10': 'fieldMask'},
    {'1': 'merge_repeated', '3': 5, '4': 1, '5': 8, '10': 'mergeRepeated'},
  ],
};

@$core.Deprecated('Use operationDescriptor instead')
const Operation_DeleteDocument$json = {
  '1': 'DeleteDocument',
  '2': [
    {'1': 'collection', '3': 1, '4': 1, '5': 9, '10': 'collection'},
    {'1': 'primary_key', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.Value', '10': 'primaryKey'},
  ],
};

@$core.Deprecated('Use operationDescriptor instead')
const Operation_InsertIndex$json = {
  '1': 'InsertIndex',
  '2': [
    {'1': 'collection', '3': 1, '4': 1, '5': 9, '10': 'collection'},
    {'1': 'path', '3': 2, '4': 1, '5': 9, '10': 'path'},
  ],
};

/// Descriptor for `Operation`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List operationDescriptor = $convert.base64Decode(
    'CglPcGVyYXRpb24STAoPaW5zZXJ0X2RvY3VtZW50GAEgASgLMiEubTEwLnNkay5PcGVyYXRpb2'
    '4uSW5zZXJ0RG9jdW1lbnRIAFIOaW5zZXJ0RG9jdW1lbnQSTAoPdXBkYXRlX2RvY3VtZW50GAIg'
    'ASgLMiEubTEwLnNkay5PcGVyYXRpb24uVXBkYXRlRG9jdW1lbnRIAFIOdXBkYXRlRG9jdW1lbn'
    'QSTAoPZGVsZXRlX2RvY3VtZW50GAMgASgLMiEubTEwLnNkay5PcGVyYXRpb24uRGVsZXRlRG9j'
    'dW1lbnRIAFIOZGVsZXRlRG9jdW1lbnQSSgoRaW5zZXJ0X2NvbGxlY3Rpb24YBCABKAsyGy5tMT'
    'Auc2RrLkNvbGxlY3Rpb25NZXRhZGF0YUgAUhBpbnNlcnRDb2xsZWN0aW9uEkMKDGluc2VydF9p'
    'bmRleBgFIAEoCzIeLm0xMC5zZGsuT3BlcmF0aW9uLkluc2VydEluZGV4SABSC2luc2VydEluZG'
    'V4GkwKDkluc2VydERvY3VtZW50Eh4KCmNvbGxlY3Rpb24YASABKAlSCmNvbGxlY3Rpb24SGgoI'
    'ZG9jdW1lbnQYAyABKAxSCGRvY3VtZW50Gt8BCg5VcGRhdGVEb2N1bWVudBIeCgpjb2xsZWN0aW'
    '9uGAEgASgJUgpjb2xsZWN0aW9uEi8KC3ByaW1hcnlfa2V5GAIgASgLMg4ubTEwLnNkay5WYWx1'
    'ZVIKcHJpbWFyeUtleRIaCghkb2N1bWVudBgDIAEoDFIIZG9jdW1lbnQSOQoKZmllbGRfbWFzax'
    'gEIAEoCzIaLmdvb2dsZS5wcm90b2J1Zi5GaWVsZE1hc2tSCWZpZWxkTWFzaxIlCg5tZXJnZV9y'
    'ZXBlYXRlZBgFIAEoCFINbWVyZ2VSZXBlYXRlZBphCg5EZWxldGVEb2N1bWVudBIeCgpjb2xsZW'
    'N0aW9uGAEgASgJUgpjb2xsZWN0aW9uEi8KC3ByaW1hcnlfa2V5GAIgASgLMg4ubTEwLnNkay5W'
    'YWx1ZVIKcHJpbWFyeUtleRpBCgtJbnNlcnRJbmRleBIeCgpjb2xsZWN0aW9uGAEgASgJUgpjb2'
    'xsZWN0aW9uEhIKBHBhdGgYAiABKAlSBHBhdGhCCwoJb3BlcmF0aW9u');

@$core.Deprecated('Use collectionMetadataDescriptor instead')
const CollectionMetadata$json = {
  '1': 'CollectionMetadata',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'descriptor_name', '3': 2, '4': 1, '5': 9, '10': 'descriptorName'},
    {'1': 'file_descriptor_set', '3': 3, '4': 1, '5': 11, '6': '.google.protobuf.FileDescriptorSet', '10': 'fileDescriptorSet'},
    {'1': 'index_metadata', '3': 4, '4': 3, '5': 11, '6': '.m10.sdk.IndexMetadata', '10': 'indexMetadata'},
    {'1': 'primary_key_path', '3': 5, '4': 1, '5': 9, '10': 'primaryKeyPath'},
  ],
};

/// Descriptor for `CollectionMetadata`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List collectionMetadataDescriptor = $convert.base64Decode(
    'ChJDb2xsZWN0aW9uTWV0YWRhdGESEgoEbmFtZRgBIAEoCVIEbmFtZRInCg9kZXNjcmlwdG9yX2'
    '5hbWUYAiABKAlSDmRlc2NyaXB0b3JOYW1lElIKE2ZpbGVfZGVzY3JpcHRvcl9zZXQYAyABKAsy'
    'Ii5nb29nbGUucHJvdG9idWYuRmlsZURlc2NyaXB0b3JTZXRSEWZpbGVEZXNjcmlwdG9yU2V0Ej'
    '0KDmluZGV4X21ldGFkYXRhGAQgAygLMhYubTEwLnNkay5JbmRleE1ldGFkYXRhUg1pbmRleE1l'
    'dGFkYXRhEigKEHByaW1hcnlfa2V5X3BhdGgYBSABKAlSDnByaW1hcnlLZXlQYXRo');

@$core.Deprecated('Use indexMetadataDescriptor instead')
const IndexMetadata$json = {
  '1': 'IndexMetadata',
  '2': [
    {'1': 'path', '3': 1, '4': 3, '5': 9, '10': 'path'},
  ],
};

/// Descriptor for `IndexMetadata`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List indexMetadataDescriptor = $convert.base64Decode(
    'Cg1JbmRleE1ldGFkYXRhEhIKBHBhdGgYASADKAlSBHBhdGg=');

@$core.Deprecated('Use expDescriptor instead')
const Exp$json = {
  '1': 'Exp',
  '2': [
    {'1': 'exp', '3': 1, '4': 1, '5': 9, '10': 'exp'},
    {'1': 'vars', '3': 2, '4': 3, '5': 11, '6': '.m10.sdk.Exp.VarsEntry', '10': 'vars'},
  ],
  '3': [Exp_VarsEntry$json],
};

@$core.Deprecated('Use expDescriptor instead')
const Exp_VarsEntry$json = {
  '1': 'VarsEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {'1': 'value', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.Value', '10': 'value'},
  ],
  '7': {'7': true},
};

/// Descriptor for `Exp`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List expDescriptor = $convert.base64Decode(
    'CgNFeHASEAoDZXhwGAEgASgJUgNleHASKgoEdmFycxgCIAMoCzIWLm0xMC5zZGsuRXhwLlZhcn'
    'NFbnRyeVIEdmFycxpHCglWYXJzRW50cnkSEAoDa2V5GAEgASgJUgNrZXkSJAoFdmFsdWUYAiAB'
    'KAsyDi5tMTAuc2RrLlZhbHVlUgV2YWx1ZToCOAE=');

@$core.Deprecated('Use valueDescriptor instead')
const Value$json = {
  '1': 'Value',
  '2': [
    {'1': 'string_value', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'stringValue'},
    {'1': 'int8_value', '3': 2, '4': 1, '5': 5, '9': 0, '10': 'int8Value'},
    {'1': 'int16_value', '3': 3, '4': 1, '5': 5, '9': 0, '10': 'int16Value'},
    {'1': 'int32_value', '3': 4, '4': 1, '5': 5, '9': 0, '10': 'int32Value'},
    {'1': 'int64_value', '3': 5, '4': 1, '5': 3, '9': 0, '10': 'int64Value'},
    {'1': 'uint8_value', '3': 6, '4': 1, '5': 13, '9': 0, '10': 'uint8Value'},
    {'1': 'uint16_value', '3': 7, '4': 1, '5': 13, '9': 0, '10': 'uint16Value'},
    {'1': 'uint32_value', '3': 8, '4': 1, '5': 13, '9': 0, '10': 'uint32Value'},
    {'1': 'uint64_value', '3': 9, '4': 1, '5': 4, '9': 0, '10': 'uint64Value'},
    {'1': 'double_value', '3': 10, '4': 1, '5': 1, '9': 0, '10': 'doubleValue'},
    {'1': 'float_value', '3': 11, '4': 1, '5': 2, '9': 0, '10': 'floatValue'},
    {'1': 'bool_value', '3': 12, '4': 1, '5': 8, '9': 0, '10': 'boolValue'},
    {'1': 'bytes_value', '3': 13, '4': 1, '5': 12, '9': 0, '10': 'bytesValue'},
  ],
  '8': [
    {'1': 'value'},
  ],
};

/// Descriptor for `Value`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List valueDescriptor = $convert.base64Decode(
    'CgVWYWx1ZRIjCgxzdHJpbmdfdmFsdWUYASABKAlIAFILc3RyaW5nVmFsdWUSHwoKaW50OF92YW'
    'x1ZRgCIAEoBUgAUglpbnQ4VmFsdWUSIQoLaW50MTZfdmFsdWUYAyABKAVIAFIKaW50MTZWYWx1'
    'ZRIhCgtpbnQzMl92YWx1ZRgEIAEoBUgAUgppbnQzMlZhbHVlEiEKC2ludDY0X3ZhbHVlGAUgAS'
    'gDSABSCmludDY0VmFsdWUSIQoLdWludDhfdmFsdWUYBiABKA1IAFIKdWludDhWYWx1ZRIjCgx1'
    'aW50MTZfdmFsdWUYByABKA1IAFILdWludDE2VmFsdWUSIwoMdWludDMyX3ZhbHVlGAggASgNSA'
    'BSC3VpbnQzMlZhbHVlEiMKDHVpbnQ2NF92YWx1ZRgJIAEoBEgAUgt1aW50NjRWYWx1ZRIjCgxk'
    'b3VibGVfdmFsdWUYCiABKAFIAFILZG91YmxlVmFsdWUSIQoLZmxvYXRfdmFsdWUYCyABKAJIAF'
    'IKZmxvYXRWYWx1ZRIfCgpib29sX3ZhbHVlGAwgASgISABSCWJvb2xWYWx1ZRIhCgtieXRlc192'
    'YWx1ZRgNIAEoDEgAUgpieXRlc1ZhbHVlQgcKBXZhbHVl');

@$core.Deprecated('Use queryRequestDescriptor instead')
const QueryRequest$json = {
  '1': 'QueryRequest',
  '2': [
    {'1': 'collection', '3': 1, '4': 1, '5': 9, '10': 'collection'},
    {'1': 'expression', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.Exp', '10': 'expression'},
    {'1': 'public_key', '3': 3, '4': 1, '5': 12, '9': 0, '10': 'publicKey', '17': true},
  ],
  '8': [
    {'1': '_public_key'},
  ],
};

/// Descriptor for `QueryRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryRequestDescriptor = $convert.base64Decode(
    'CgxRdWVyeVJlcXVlc3QSHgoKY29sbGVjdGlvbhgBIAEoCVIKY29sbGVjdGlvbhIsCgpleHByZX'
    'NzaW9uGAIgASgLMgwubTEwLnNkay5FeHBSCmV4cHJlc3Npb24SIgoKcHVibGljX2tleRgDIAEo'
    'DEgAUglwdWJsaWNLZXmIAQFCDQoLX3B1YmxpY19rZXk=');

