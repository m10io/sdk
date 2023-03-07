///
//  Generated code. Do not modify.
//  source: sdk/model/model.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class BankAccountRef_BankAccountType extends $pb.ProtobufEnum {
  static const BankAccountRef_BankAccountType CBDC = BankAccountRef_BankAccountType._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'CBDC');
  static const BankAccountRef_BankAccountType DRM = BankAccountRef_BankAccountType._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'DRM');

  static const $core.List<BankAccountRef_BankAccountType> values = <BankAccountRef_BankAccountType> [
    CBDC,
    DRM,
  ];

  static final $core.Map<$core.int, BankAccountRef_BankAccountType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static BankAccountRef_BankAccountType? valueOf($core.int value) => _byValue[value];

  const BankAccountRef_BankAccountType._($core.int v, $core.String n) : super(v, n);
}

