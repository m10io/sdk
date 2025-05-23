//
//  Generated code. Do not modify.
//  source: directory/api.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Types of aliases.
class Alias_Type extends $pb.ProtobufEnum {
  static const Alias_Type HANDLE = Alias_Type._(0, _omitEnumNames ? '' : 'HANDLE');
  static const Alias_Type EMAIL = Alias_Type._(1, _omitEnumNames ? '' : 'EMAIL');
  static const Alias_Type PHONE = Alias_Type._(2, _omitEnumNames ? '' : 'PHONE');

  static const $core.List<Alias_Type> values = <Alias_Type> [
    HANDLE,
    EMAIL,
    PHONE,
  ];

  static final $core.Map<$core.int, Alias_Type> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Alias_Type? valueOf($core.int value) => _byValue[value];

  const Alias_Type._(super.v, super.n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
