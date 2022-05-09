///
//  Generated code. Do not modify.
//  source: sdk/rbac.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class Rule_Verb extends $pb.ProtobufEnum {
  static const Rule_Verb READ = Rule_Verb._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'READ');
  static const Rule_Verb CREATE = Rule_Verb._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'CREATE');
  static const Rule_Verb UPDATE = Rule_Verb._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'UPDATE');
  static const Rule_Verb DELETE = Rule_Verb._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'DELETE');
  static const Rule_Verb TRANSACT = Rule_Verb._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TRANSACT');
  static const Rule_Verb INITIATE = Rule_Verb._(5, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INITIATE');
  static const Rule_Verb COMMIT = Rule_Verb._(6, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'COMMIT');

  static const $core.List<Rule_Verb> values = <Rule_Verb> [
    READ,
    CREATE,
    UPDATE,
    DELETE,
    TRANSACT,
    INITIATE,
    COMMIT,
  ];

  static final $core.Map<$core.int, Rule_Verb> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Rule_Verb? valueOf($core.int value) => _byValue[value];

  const Rule_Verb._($core.int v, $core.String n) : super(v, n);
}

