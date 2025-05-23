//
//  Generated code. Do not modify.
//  source: sdk/rbac.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Verb defines the actions a subject can perform on a resource.
class Rule_Verb extends $pb.ProtobufEnum {
  static const Rule_Verb READ = Rule_Verb._(0, _omitEnumNames ? '' : 'READ');
  static const Rule_Verb CREATE = Rule_Verb._(1, _omitEnumNames ? '' : 'CREATE');
  static const Rule_Verb UPDATE = Rule_Verb._(2, _omitEnumNames ? '' : 'UPDATE');
  static const Rule_Verb DELETE = Rule_Verb._(3, _omitEnumNames ? '' : 'DELETE');
  static const Rule_Verb TRANSACT = Rule_Verb._(4, _omitEnumNames ? '' : 'TRANSACT');
  static const Rule_Verb INITIATE = Rule_Verb._(5, _omitEnumNames ? '' : 'INITIATE');
  static const Rule_Verb COMMIT = Rule_Verb._(6, _omitEnumNames ? '' : 'COMMIT');

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

  const Rule_Verb._(super.v, super.n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
