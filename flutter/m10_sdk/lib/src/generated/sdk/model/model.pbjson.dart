//
//  Generated code. Do not modify.
//  source: sdk/model/model.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use accountMetadataDescriptor instead')
const AccountMetadata$json = {
  '1': 'AccountMetadata',
  '2': [
    {'1': 'owner', '3': 1, '4': 1, '5': 12, '10': 'owner'},
    {'1': 'profile_image_url', '3': 9, '4': 1, '5': 9, '10': 'profileImageUrl'},
    {'1': 'name', '3': 10, '4': 1, '5': 9, '10': 'name'},
    {'1': 'public_name', '3': 11, '4': 1, '5': 9, '10': 'publicName'},
    {'1': 'id', '3': 12, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `AccountMetadata`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountMetadataDescriptor = $convert.base64Decode(
    'Cg9BY2NvdW50TWV0YWRhdGESFAoFb3duZXIYASABKAxSBW93bmVyEioKEXByb2ZpbGVfaW1hZ2'
    'VfdXJsGAkgASgJUg9wcm9maWxlSW1hZ2VVcmwSEgoEbmFtZRgKIAEoCVIEbmFtZRIfCgtwdWJs'
    'aWNfbmFtZRgLIAEoCVIKcHVibGljTmFtZRIOCgJpZBgMIAEoDFICaWQ=');

@$core.Deprecated('Use accountSetDescriptor instead')
const AccountSet$json = {
  '1': 'AccountSet',
  '2': [
    {'1': 'owner', '3': 1, '4': 1, '5': 12, '10': 'owner'},
    {'1': 'accounts', '3': 2, '4': 3, '5': 12, '10': 'accounts'},
    {'1': 'id', '3': 7, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `AccountSet`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountSetDescriptor = $convert.base64Decode(
    'CgpBY2NvdW50U2V0EhQKBW93bmVyGAEgASgMUgVvd25lchIaCghhY2NvdW50cxgCIAMoDFIIYW'
    'Njb3VudHMSDgoCaWQYByABKAxSAmlk');

@$core.Deprecated('Use accountInfoDescriptor instead')
const AccountInfo$json = {
  '1': 'AccountInfo',
  '2': [
    {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    {'1': 'parent_account_id', '3': 2, '4': 1, '5': 12, '10': 'parentAccountId'},
    {'1': 'public_name', '3': 3, '4': 1, '5': 9, '10': 'publicName'},
    {'1': 'profile_image_url', '3': 4, '4': 1, '5': 9, '10': 'profileImageUrl'},
    {'1': 'code', '3': 5, '4': 1, '5': 9, '10': 'code'},
    {'1': 'decimal_places', '3': 6, '4': 1, '5': 13, '10': 'decimalPlaces'},
  ],
};

/// Descriptor for `AccountInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountInfoDescriptor = $convert.base64Decode(
    'CgtBY2NvdW50SW5mbxIdCgphY2NvdW50X2lkGAEgASgMUglhY2NvdW50SWQSKgoRcGFyZW50X2'
    'FjY291bnRfaWQYAiABKAxSD3BhcmVudEFjY291bnRJZBIfCgtwdWJsaWNfbmFtZRgDIAEoCVIK'
    'cHVibGljTmFtZRIqChFwcm9maWxlX2ltYWdlX3VybBgEIAEoCVIPcHJvZmlsZUltYWdlVXJsEh'
    'IKBGNvZGUYBSABKAlSBGNvZGUSJQoOZGVjaW1hbF9wbGFjZXMYBiABKA1SDWRlY2ltYWxQbGFj'
    'ZXM=');

@$core.Deprecated('Use bankAccountRefDescriptor instead')
const BankAccountRef$json = {
  '1': 'BankAccountRef',
  '2': [
    {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    {'1': 'account_type', '3': 2, '4': 1, '5': 14, '6': '.m10.sdk.model.BankAccountRef.BankAccountType', '10': 'accountType'},
  ],
  '4': [BankAccountRef_BankAccountType$json],
};

@$core.Deprecated('Use bankAccountRefDescriptor instead')
const BankAccountRef_BankAccountType$json = {
  '1': 'BankAccountType',
  '2': [
    {'1': 'CBDC', '2': 0},
    {'1': 'DRM', '2': 1},
  ],
};

/// Descriptor for `BankAccountRef`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bankAccountRefDescriptor = $convert.base64Decode(
    'Cg5CYW5rQWNjb3VudFJlZhIdCgphY2NvdW50X2lkGAEgASgMUglhY2NvdW50SWQSUAoMYWNjb3'
    'VudF90eXBlGAIgASgOMi0ubTEwLnNkay5tb2RlbC5CYW5rQWNjb3VudFJlZi5CYW5rQWNjb3Vu'
    'dFR5cGVSC2FjY291bnRUeXBlIiQKD0JhbmtBY2NvdW50VHlwZRIICgRDQkRDEAASBwoDRFJNEA'
    'E=');

@$core.Deprecated('Use bankDescriptor instead')
const Bank$json = {
  '1': 'Bank',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    {'1': 'owner', '3': 2, '4': 1, '5': 12, '10': 'owner'},
    {'1': 'short_name', '3': 3, '4': 1, '5': 9, '10': 'shortName'},
    {'1': 'display_name', '3': 4, '4': 1, '5': 9, '10': 'displayName'},
    {'1': 'accounts', '3': 5, '4': 3, '5': 11, '6': '.m10.sdk.model.BankAccountRef', '10': 'accounts'},
  ],
};

/// Descriptor for `Bank`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bankDescriptor = $convert.base64Decode(
    'CgRCYW5rEg4KAmlkGAEgASgMUgJpZBIUCgVvd25lchgCIAEoDFIFb3duZXISHQoKc2hvcnRfbm'
    'FtZRgDIAEoCVIJc2hvcnROYW1lEiEKDGRpc3BsYXlfbmFtZRgEIAEoCVILZGlzcGxheU5hbWUS'
    'OQoIYWNjb3VudHMYBSADKAsyHS5tMTAuc2RrLm1vZGVsLkJhbmtBY2NvdW50UmVmUghhY2NvdW'
    '50cw==');

