///
//  Generated code. Do not modify.
//  source: sdk/model/model.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use accountDescriptor instead')
const Account$json = const {
  '1': 'Account',
  '2': const [
    const {'1': 'owner', '3': 1, '4': 1, '5': 12, '10': 'owner'},
    const {'1': 'profile_image_url', '3': 9, '4': 1, '5': 9, '10': 'profileImageUrl'},
    const {'1': 'name', '3': 10, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'public_name', '3': 11, '4': 1, '5': 9, '10': 'publicName'},
    const {'1': 'id', '3': 12, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `Account`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountDescriptor = $convert.base64Decode('CgdBY2NvdW50EhQKBW93bmVyGAEgASgMUgVvd25lchIqChFwcm9maWxlX2ltYWdlX3VybBgJIAEoCVIPcHJvZmlsZUltYWdlVXJsEhIKBG5hbWUYCiABKAlSBG5hbWUSHwoLcHVibGljX25hbWUYCyABKAlSCnB1YmxpY05hbWUSDgoCaWQYDCABKAxSAmlk');
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
