///
//  Generated code. Do not modify.
//  source: directory/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class Alias_Type extends $pb.ProtobufEnum {
  static const Alias_Type HANDLE = Alias_Type._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'HANDLE');
  static const Alias_Type EMAIL = Alias_Type._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EMAIL');
  static const Alias_Type PHONE = Alias_Type._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'PHONE');

  static const $core.List<Alias_Type> values = <Alias_Type> [
    HANDLE,
    EMAIL,
    PHONE,
  ];

  static final $core.Map<$core.int, Alias_Type> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Alias_Type? valueOf($core.int value) => _byValue[value];

  const Alias_Type._($core.int v, $core.String n) : super(v, n);
}

