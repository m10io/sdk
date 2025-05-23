//
//  Generated code. Do not modify.
//  source: sdk/model/model.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// / Bank account type.
class BankAccountRef_BankAccountType extends $pb.ProtobufEnum {
  /// / Central Bank Digital Currency.
  static const BankAccountRef_BankAccountType CBDC = BankAccountRef_BankAccountType._(0, _omitEnumNames ? '' : 'CBDC');
  /// / Digital Regulated Money.
  static const BankAccountRef_BankAccountType DRM = BankAccountRef_BankAccountType._(1, _omitEnumNames ? '' : 'DRM');

  static const $core.List<BankAccountRef_BankAccountType> values = <BankAccountRef_BankAccountType> [
    CBDC,
    DRM,
  ];

  static final $core.Map<$core.int, BankAccountRef_BankAccountType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static BankAccountRef_BankAccountType? valueOf($core.int value) => _byValue[value];

  const BankAccountRef_BankAccountType._(super.v, super.n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
