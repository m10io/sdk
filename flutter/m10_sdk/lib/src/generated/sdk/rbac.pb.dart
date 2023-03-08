///
//  Generated code. Do not modify.
//  source: sdk/rbac.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'document.pb.dart' as $7;

import 'rbac.pbenum.dart';

export 'rbac.pbenum.dart';

class RoleBinding extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RoleBinding', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'role', $pb.PbFieldType.OY)
    ..p<$core.List<$core.int>>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'subjects', $pb.PbFieldType.PY)
    ..pc<Expression>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'expressions', $pb.PbFieldType.PM, subBuilder: Expression.create)
    ..aOB(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'isUniversal')
    ..a<$core.List<$core.int>>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'owner', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  RoleBinding._() : super();
  factory RoleBinding({
    $core.List<$core.int>? id,
    $core.String? name,
    $core.List<$core.int>? role,
    $core.Iterable<$core.List<$core.int>>? subjects,
    $core.Iterable<Expression>? expressions,
    $core.bool? isUniversal,
    $core.List<$core.int>? owner,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (name != null) {
      _result.name = name;
    }
    if (role != null) {
      _result.role = role;
    }
    if (subjects != null) {
      _result.subjects.addAll(subjects);
    }
    if (expressions != null) {
      _result.expressions.addAll(expressions);
    }
    if (isUniversal != null) {
      _result.isUniversal = isUniversal;
    }
    if (owner != null) {
      _result.owner = owner;
    }
    return _result;
  }
  factory RoleBinding.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RoleBinding.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RoleBinding clone() => RoleBinding()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RoleBinding copyWith(void Function(RoleBinding) updates) => super.copyWith((message) => updates(message as RoleBinding)) as RoleBinding; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RoleBinding create() => RoleBinding._();
  RoleBinding createEmptyInstance() => create();
  static $pb.PbList<RoleBinding> createRepeated() => $pb.PbList<RoleBinding>();
  @$core.pragma('dart2js:noInline')
  static RoleBinding getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RoleBinding>(create);
  static RoleBinding? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get role => $_getN(2);
  @$pb.TagNumber(3)
  set role($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasRole() => $_has(2);
  @$pb.TagNumber(3)
  void clearRole() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.List<$core.int>> get subjects => $_getList(3);

  @$pb.TagNumber(5)
  $core.List<Expression> get expressions => $_getList(4);

  @$pb.TagNumber(6)
  $core.bool get isUniversal => $_getBF(5);
  @$pb.TagNumber(6)
  set isUniversal($core.bool v) { $_setBool(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasIsUniversal() => $_has(5);
  @$pb.TagNumber(6)
  void clearIsUniversal() => clearField(6);

  @$pb.TagNumber(7)
  $core.List<$core.int> get owner => $_getN(6);
  @$pb.TagNumber(7)
  set owner($core.List<$core.int> v) { $_setBytes(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasOwner() => $_has(6);
  @$pb.TagNumber(7)
  void clearOwner() => clearField(7);
}

class Expression extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Expression', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'collection')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'expression')
    ..hasRequiredFields = false
  ;

  Expression._() : super();
  factory Expression({
    $core.String? collection,
    $core.String? expression,
  }) {
    final _result = create();
    if (collection != null) {
      _result.collection = collection;
    }
    if (expression != null) {
      _result.expression = expression;
    }
    return _result;
  }
  factory Expression.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Expression.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Expression clone() => Expression()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Expression copyWith(void Function(Expression) updates) => super.copyWith((message) => updates(message as Expression)) as Expression; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Expression create() => Expression._();
  Expression createEmptyInstance() => create();
  static $pb.PbList<Expression> createRepeated() => $pb.PbList<Expression>();
  @$core.pragma('dart2js:noInline')
  static Expression getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Expression>(create);
  static Expression? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get collection => $_getSZ(0);
  @$pb.TagNumber(1)
  set collection($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCollection() => $_has(0);
  @$pb.TagNumber(1)
  void clearCollection() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get expression => $_getSZ(1);
  @$pb.TagNumber(2)
  set expression($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasExpression() => $_has(1);
  @$pb.TagNumber(2)
  void clearExpression() => clearField(2);
}

class Role extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Role', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'owner', $pb.PbFieldType.OY)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..pc<Rule>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'rules', $pb.PbFieldType.PM, subBuilder: Rule.create)
    ..hasRequiredFields = false
  ;

  Role._() : super();
  factory Role({
    $core.List<$core.int>? id,
    $core.List<$core.int>? owner,
    $core.String? name,
    $core.Iterable<Rule>? rules,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (owner != null) {
      _result.owner = owner;
    }
    if (name != null) {
      _result.name = name;
    }
    if (rules != null) {
      _result.rules.addAll(rules);
    }
    return _result;
  }
  factory Role.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Role.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Role clone() => Role()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Role copyWith(void Function(Role) updates) => super.copyWith((message) => updates(message as Role)) as Role; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Role create() => Role._();
  Role createEmptyInstance() => create();
  static $pb.PbList<Role> createRepeated() => $pb.PbList<Role>();
  @$core.pragma('dart2js:noInline')
  static Role getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Role>(create);
  static Role? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get owner => $_getN(1);
  @$pb.TagNumber(2)
  set owner($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasOwner() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwner() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<Rule> get rules => $_getList(3);
}

class Rule extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Rule', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'collection')
    ..pc<$7.Value>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'instanceKeys', $pb.PbFieldType.PM, subBuilder: $7.Value.create)
    ..pc<Rule_Verb>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'verbs', $pb.PbFieldType.KE, valueOf: Rule_Verb.valueOf, enumValues: Rule_Verb.values, defaultEnumValue: Rule_Verb.READ)
    ..hasRequiredFields = false
  ;

  Rule._() : super();
  factory Rule({
    $core.String? collection,
    $core.Iterable<$7.Value>? instanceKeys,
    $core.Iterable<Rule_Verb>? verbs,
  }) {
    final _result = create();
    if (collection != null) {
      _result.collection = collection;
    }
    if (instanceKeys != null) {
      _result.instanceKeys.addAll(instanceKeys);
    }
    if (verbs != null) {
      _result.verbs.addAll(verbs);
    }
    return _result;
  }
  factory Rule.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Rule.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Rule clone() => Rule()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Rule copyWith(void Function(Rule) updates) => super.copyWith((message) => updates(message as Rule)) as Rule; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Rule create() => Rule._();
  Rule createEmptyInstance() => create();
  static $pb.PbList<Rule> createRepeated() => $pb.PbList<Rule>();
  @$core.pragma('dart2js:noInline')
  static Rule getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Rule>(create);
  static Rule? _defaultInstance;

  @$pb.TagNumber(2)
  $core.String get collection => $_getSZ(0);
  @$pb.TagNumber(2)
  set collection($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(2)
  $core.bool hasCollection() => $_has(0);
  @$pb.TagNumber(2)
  void clearCollection() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$7.Value> get instanceKeys => $_getList(1);

  @$pb.TagNumber(4)
  $core.List<Rule_Verb> get verbs => $_getList(2);
}

