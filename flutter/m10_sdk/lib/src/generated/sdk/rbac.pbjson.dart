//
//  Generated code. Do not modify.
//  source: sdk/rbac.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use roleBindingDescriptor instead')
const RoleBinding$json = {
  '1': 'RoleBinding',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'role', '3': 3, '4': 1, '5': 12, '10': 'role'},
    {'1': 'subjects', '3': 4, '4': 3, '5': 12, '10': 'subjects'},
    {'1': 'expressions', '3': 5, '4': 3, '5': 11, '6': '.m10.sdk.Expression', '10': 'expressions'},
    {'1': 'is_universal', '3': 6, '4': 1, '5': 8, '10': 'isUniversal'},
    {'1': 'owner', '3': 7, '4': 1, '5': 12, '10': 'owner'},
  ],
};

/// Descriptor for `RoleBinding`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List roleBindingDescriptor = $convert.base64Decode(
    'CgtSb2xlQmluZGluZxIOCgJpZBgBIAEoDFICaWQSEgoEbmFtZRgCIAEoCVIEbmFtZRISCgRyb2'
    'xlGAMgASgMUgRyb2xlEhoKCHN1YmplY3RzGAQgAygMUghzdWJqZWN0cxI1CgtleHByZXNzaW9u'
    'cxgFIAMoCzITLm0xMC5zZGsuRXhwcmVzc2lvblILZXhwcmVzc2lvbnMSIQoMaXNfdW5pdmVyc2'
    'FsGAYgASgIUgtpc1VuaXZlcnNhbBIUCgVvd25lchgHIAEoDFIFb3duZXI=');

@$core.Deprecated('Use expressionDescriptor instead')
const Expression$json = {
  '1': 'Expression',
  '2': [
    {'1': 'collection', '3': 1, '4': 1, '5': 9, '10': 'collection'},
    {'1': 'expression', '3': 2, '4': 1, '5': 9, '10': 'expression'},
  ],
};

/// Descriptor for `Expression`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List expressionDescriptor = $convert.base64Decode(
    'CgpFeHByZXNzaW9uEh4KCmNvbGxlY3Rpb24YASABKAlSCmNvbGxlY3Rpb24SHgoKZXhwcmVzc2'
    'lvbhgCIAEoCVIKZXhwcmVzc2lvbg==');

@$core.Deprecated('Use roleDescriptor instead')
const Role$json = {
  '1': 'Role',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    {'1': 'owner', '3': 2, '4': 1, '5': 12, '10': 'owner'},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
    {'1': 'rules', '3': 4, '4': 3, '5': 11, '6': '.m10.sdk.Rule', '10': 'rules'},
  ],
};

/// Descriptor for `Role`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List roleDescriptor = $convert.base64Decode(
    'CgRSb2xlEg4KAmlkGAEgASgMUgJpZBIUCgVvd25lchgCIAEoDFIFb3duZXISEgoEbmFtZRgDIA'
    'EoCVIEbmFtZRIjCgVydWxlcxgEIAMoCzINLm0xMC5zZGsuUnVsZVIFcnVsZXM=');

@$core.Deprecated('Use ruleDescriptor instead')
const Rule$json = {
  '1': 'Rule',
  '2': [
    {'1': 'collection', '3': 2, '4': 1, '5': 9, '10': 'collection'},
    {'1': 'instance_keys', '3': 3, '4': 3, '5': 11, '6': '.m10.sdk.Value', '10': 'instanceKeys'},
    {'1': 'verbs', '3': 4, '4': 3, '5': 14, '6': '.m10.sdk.Rule.Verb', '10': 'verbs'},
    {'1': 'excluded_instance_keys', '3': 5, '4': 3, '5': 11, '6': '.m10.sdk.Value', '10': 'excludedInstanceKeys'},
  ],
  '4': [Rule_Verb$json],
};

@$core.Deprecated('Use ruleDescriptor instead')
const Rule_Verb$json = {
  '1': 'Verb',
  '2': [
    {'1': 'READ', '2': 0},
    {'1': 'CREATE', '2': 1},
    {'1': 'UPDATE', '2': 2},
    {'1': 'DELETE', '2': 3},
    {'1': 'TRANSACT', '2': 4},
    {'1': 'INITIATE', '2': 5},
    {'1': 'COMMIT', '2': 6},
  ],
};

/// Descriptor for `Rule`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List ruleDescriptor = $convert.base64Decode(
    'CgRSdWxlEh4KCmNvbGxlY3Rpb24YAiABKAlSCmNvbGxlY3Rpb24SMwoNaW5zdGFuY2Vfa2V5cx'
    'gDIAMoCzIOLm0xMC5zZGsuVmFsdWVSDGluc3RhbmNlS2V5cxIoCgV2ZXJicxgEIAMoDjISLm0x'
    'MC5zZGsuUnVsZS5WZXJiUgV2ZXJicxJEChZleGNsdWRlZF9pbnN0YW5jZV9rZXlzGAUgAygLMg'
    '4ubTEwLnNkay5WYWx1ZVIUZXhjbHVkZWRJbnN0YW5jZUtleXMiXAoEVmVyYhIICgRSRUFEEAAS'
    'CgoGQ1JFQVRFEAESCgoGVVBEQVRFEAISCgoGREVMRVRFEAMSDAoIVFJBTlNBQ1QQBBIMCghJTk'
    'lUSUFURRAFEgoKBkNPTU1JVBAG');

