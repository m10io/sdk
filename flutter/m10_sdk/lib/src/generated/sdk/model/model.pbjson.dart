///
//  Generated code. Do not modify.
//  source: sdk/model/model.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use accountMetadataDescriptor instead')
const AccountMetadata$json = const {
  '1': 'AccountMetadata',
  '2': const [
    const {'1': 'owner', '3': 1, '4': 1, '5': 12, '10': 'owner'},
    const {'1': 'profile_image_url', '3': 9, '4': 1, '5': 9, '10': 'profileImageUrl'},
    const {'1': 'name', '3': 10, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'public_name', '3': 11, '4': 1, '5': 9, '10': 'publicName'},
    const {'1': 'id', '3': 12, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `AccountMetadata`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountMetadataDescriptor = $convert.base64Decode('Cg9BY2NvdW50TWV0YWRhdGESFAoFb3duZXIYASABKAxSBW93bmVyEioKEXByb2ZpbGVfaW1hZ2VfdXJsGAkgASgJUg9wcm9maWxlSW1hZ2VVcmwSEgoEbmFtZRgKIAEoCVIEbmFtZRIfCgtwdWJsaWNfbmFtZRgLIAEoCVIKcHVibGljTmFtZRIOCgJpZBgMIAEoDFICaWQ=');
@$core.Deprecated('Use accountRefDescriptor instead')
const AccountRef$json = const {
  '1': 'AccountRef',
  '2': const [
    const {'1': 'ledger_id', '3': 1, '4': 1, '5': 9, '10': 'ledgerId'},
    const {'1': 'account_id', '3': 2, '4': 1, '5': 12, '10': 'accountId'},
  ],
};

/// Descriptor for `AccountRef`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountRefDescriptor = $convert.base64Decode('CgpBY2NvdW50UmVmEhsKCWxlZGdlcl9pZBgBIAEoCVIIbGVkZ2VySWQSHQoKYWNjb3VudF9pZBgCIAEoDFIJYWNjb3VudElk');
@$core.Deprecated('Use accountSetDescriptor instead')
const AccountSet$json = const {
  '1': 'AccountSet',
  '2': const [
    const {'1': 'owner', '3': 1, '4': 1, '5': 12, '10': 'owner'},
    const {'1': 'accounts', '3': 2, '4': 3, '5': 11, '6': '.m10.sdk.model.AccountRef', '10': 'accounts'},
    const {'1': 'id', '3': 7, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `AccountSet`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountSetDescriptor = $convert.base64Decode('CgpBY2NvdW50U2V0EhQKBW93bmVyGAEgASgMUgVvd25lchI1CghhY2NvdW50cxgCIAMoCzIZLm0xMC5zZGsubW9kZWwuQWNjb3VudFJlZlIIYWNjb3VudHMSDgoCaWQYByABKAxSAmlk');
@$core.Deprecated('Use accountInfoDescriptor instead')
const AccountInfo$json = const {
  '1': 'AccountInfo',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    const {'1': 'parent_account_id', '3': 2, '4': 1, '5': 12, '10': 'parentAccountId'},
    const {'1': 'public_name', '3': 3, '4': 1, '5': 9, '10': 'publicName'},
    const {'1': 'profile_image_url', '3': 4, '4': 1, '5': 9, '10': 'profileImageUrl'},
    const {'1': 'code', '3': 5, '4': 1, '5': 9, '10': 'code'},
    const {'1': 'decimal_places', '3': 6, '4': 1, '5': 13, '10': 'decimalPlaces'},
  ],
};

/// Descriptor for `AccountInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountInfoDescriptor = $convert.base64Decode('CgtBY2NvdW50SW5mbxIdCgphY2NvdW50X2lkGAEgASgMUglhY2NvdW50SWQSKgoRcGFyZW50X2FjY291bnRfaWQYAiABKAxSD3BhcmVudEFjY291bnRJZBIfCgtwdWJsaWNfbmFtZRgDIAEoCVIKcHVibGljTmFtZRIqChFwcm9maWxlX2ltYWdlX3VybBgEIAEoCVIPcHJvZmlsZUltYWdlVXJsEhIKBGNvZGUYBSABKAlSBGNvZGUSJQoOZGVjaW1hbF9wbGFjZXMYBiABKA1SDWRlY2ltYWxQbGFjZXM=');
@$core.Deprecated('Use bankAccountRefDescriptor instead')
const BankAccountRef$json = const {
  '1': 'BankAccountRef',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    const {'1': 'account_type', '3': 2, '4': 1, '5': 14, '6': '.m10.sdk.model.BankAccountRef.BankAccountType', '10': 'accountType'},
  ],
  '4': const [BankAccountRef_BankAccountType$json],
};

@$core.Deprecated('Use bankAccountRefDescriptor instead')
const BankAccountRef_BankAccountType$json = const {
  '1': 'BankAccountType',
  '2': const [
    const {'1': 'CBDC', '2': 0},
    const {'1': 'DRM', '2': 1},
  ],
};

/// Descriptor for `BankAccountRef`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bankAccountRefDescriptor = $convert.base64Decode('Cg5CYW5rQWNjb3VudFJlZhIdCgphY2NvdW50X2lkGAEgASgMUglhY2NvdW50SWQSUAoMYWNjb3VudF90eXBlGAIgASgOMi0ubTEwLnNkay5tb2RlbC5CYW5rQWNjb3VudFJlZi5CYW5rQWNjb3VudFR5cGVSC2FjY291bnRUeXBlIiQKD0JhbmtBY2NvdW50VHlwZRIICgRDQkRDEAASBwoDRFJNEAE=');
@$core.Deprecated('Use bankDescriptor instead')
const Bank$json = const {
  '1': 'Bank',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'owner', '3': 2, '4': 1, '5': 12, '10': 'owner'},
    const {'1': 'short_name', '3': 3, '4': 1, '5': 9, '10': 'shortName'},
    const {'1': 'display_name', '3': 4, '4': 1, '5': 9, '10': 'displayName'},
    const {'1': 'accounts', '3': 5, '4': 3, '5': 11, '6': '.m10.sdk.model.BankAccountRef', '10': 'accounts'},
  ],
};

/// Descriptor for `Bank`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bankDescriptor = $convert.base64Decode('CgRCYW5rEg4KAmlkGAEgASgMUgJpZBIUCgVvd25lchgCIAEoDFIFb3duZXISHQoKc2hvcnRfbmFtZRgDIAEoCVIJc2hvcnROYW1lEiEKDGRpc3BsYXlfbmFtZRgEIAEoCVILZGlzcGxheU5hbWUSOQoIYWNjb3VudHMYBSADKAsyHS5tMTAuc2RrLm1vZGVsLkJhbmtBY2NvdW50UmVmUghhY2NvdW50cw==');
