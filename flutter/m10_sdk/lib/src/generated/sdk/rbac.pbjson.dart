///
//  Generated code. Do not modify.
//  source: sdk/rbac.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use roleBindingDescriptor instead')
const RoleBinding$json = const {
  '1': 'RoleBinding',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'role', '3': 3, '4': 1, '5': 12, '10': 'role'},
    const {'1': 'subjects', '3': 4, '4': 3, '5': 12, '10': 'subjects'},
    const {'1': 'expressions', '3': 5, '4': 3, '5': 11, '6': '.m10.sdk.Expression', '10': 'expressions'},
    const {'1': 'is_universal', '3': 6, '4': 1, '5': 8, '10': 'isUniversal'},
    const {'1': 'owner', '3': 7, '4': 1, '5': 12, '10': 'owner'},
  ],
};

/// Descriptor for `RoleBinding`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List roleBindingDescriptor = $convert.base64Decode('CgtSb2xlQmluZGluZxIOCgJpZBgBIAEoDFICaWQSEgoEbmFtZRgCIAEoCVIEbmFtZRISCgRyb2xlGAMgASgMUgRyb2xlEhoKCHN1YmplY3RzGAQgAygMUghzdWJqZWN0cxI1CgtleHByZXNzaW9ucxgFIAMoCzITLm0xMC5zZGsuRXhwcmVzc2lvblILZXhwcmVzc2lvbnMSIQoMaXNfdW5pdmVyc2FsGAYgASgIUgtpc1VuaXZlcnNhbBIUCgVvd25lchgHIAEoDFIFb3duZXI=');
@$core.Deprecated('Use expressionDescriptor instead')
const Expression$json = const {
  '1': 'Expression',
  '2': const [
    const {'1': 'collection', '3': 1, '4': 1, '5': 9, '10': 'collection'},
    const {'1': 'expression', '3': 2, '4': 1, '5': 9, '10': 'expression'},
  ],
};

/// Descriptor for `Expression`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List expressionDescriptor = $convert.base64Decode('CgpFeHByZXNzaW9uEh4KCmNvbGxlY3Rpb24YASABKAlSCmNvbGxlY3Rpb24SHgoKZXhwcmVzc2lvbhgCIAEoCVIKZXhwcmVzc2lvbg==');
@$core.Deprecated('Use roleDescriptor instead')
const Role$json = const {
  '1': 'Role',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'owner', '3': 2, '4': 1, '5': 12, '10': 'owner'},
    const {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'rules', '3': 4, '4': 3, '5': 11, '6': '.m10.sdk.Rule', '10': 'rules'},
  ],
};

/// Descriptor for `Role`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List roleDescriptor = $convert.base64Decode('CgRSb2xlEg4KAmlkGAEgASgMUgJpZBIUCgVvd25lchgCIAEoDFIFb3duZXISEgoEbmFtZRgDIAEoCVIEbmFtZRIjCgVydWxlcxgEIAMoCzINLm0xMC5zZGsuUnVsZVIFcnVsZXM=');
@$core.Deprecated('Use ruleDescriptor instead')
const Rule$json = const {
  '1': 'Rule',
  '2': const [
    const {'1': 'collection', '3': 2, '4': 1, '5': 9, '10': 'collection'},
    const {'1': 'instance_keys', '3': 3, '4': 3, '5': 11, '6': '.m10.sdk.Value', '10': 'instanceKeys'},
    const {'1': 'verbs', '3': 4, '4': 3, '5': 14, '6': '.m10.sdk.Rule.Verb', '10': 'verbs'},
  ],
  '4': const [Rule_Verb$json],
};

@$core.Deprecated('Use ruleDescriptor instead')
const Rule_Verb$json = const {
  '1': 'Verb',
  '2': const [
    const {'1': 'READ', '2': 0},
    const {'1': 'CREATE', '2': 1},
    const {'1': 'UPDATE', '2': 2},
    const {'1': 'DELETE', '2': 3},
    const {'1': 'TRANSACT', '2': 4},
    const {'1': 'INITIATE', '2': 5},
    const {'1': 'COMMIT', '2': 6},
  ],
};

/// Descriptor for `Rule`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List ruleDescriptor = $convert.base64Decode('CgRSdWxlEh4KCmNvbGxlY3Rpb24YAiABKAlSCmNvbGxlY3Rpb24SMwoNaW5zdGFuY2Vfa2V5cxgDIAMoCzIOLm0xMC5zZGsuVmFsdWVSDGluc3RhbmNlS2V5cxIoCgV2ZXJicxgEIAMoDjISLm0xMC5zZGsuUnVsZS5WZXJiUgV2ZXJicyJcCgRWZXJiEggKBFJFQUQQABIKCgZDUkVBVEUQARIKCgZVUERBVEUQAhIKCgZERUxFVEUQAxIMCghUUkFOU0FDVBAEEgwKCElOSVRJQVRFEAUSCgoGQ09NTUlUEAY=');
