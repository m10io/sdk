///
//  Generated code. Do not modify.
//  source: sdk/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class Signature_Algorithm extends $pb.ProtobufEnum {
  static const Signature_Algorithm P256_SHA256_ASN1 = Signature_Algorithm._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'P256_SHA256_ASN1');
  static const Signature_Algorithm ED25519 = Signature_Algorithm._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ED25519');

  static const $core.List<Signature_Algorithm> values = <Signature_Algorithm> [
    P256_SHA256_ASN1,
    ED25519,
  ];

  static final $core.Map<$core.int, Signature_Algorithm> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Signature_Algorithm? valueOf($core.int value) => _byValue[value];

  const Signature_Algorithm._($core.int v, $core.String n) : super(v, n);
}

