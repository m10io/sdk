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

import 'document.pb.dart' as $7;
import 'rbac.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'rbac.pbenum.dart';

/// RoleBinding represents the binding of a Role to a set of subjects.
class RoleBinding extends $pb.GeneratedMessage {
  factory RoleBinding({
    $core.List<$core.int>? id,
    $core.String? name,
    $core.List<$core.int>? role,
    $core.Iterable<$core.List<$core.int>>? subjects,
    $core.Iterable<Expression>? expressions,
    $core.bool? isUniversal,
    $core.List<$core.int>? owner,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    if (name != null) {
      $result.name = name;
    }
    if (role != null) {
      $result.role = role;
    }
    if (subjects != null) {
      $result.subjects.addAll(subjects);
    }
    if (expressions != null) {
      $result.expressions.addAll(expressions);
    }
    if (isUniversal != null) {
      $result.isUniversal = isUniversal;
    }
    if (owner != null) {
      $result.owner = owner;
    }
    return $result;
  }
  RoleBinding._() : super();
  factory RoleBinding.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RoleBinding.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RoleBinding', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'role', $pb.PbFieldType.OY)
    ..p<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'subjects', $pb.PbFieldType.PY)
    ..pc<Expression>(5, _omitFieldNames ? '' : 'expressions', $pb.PbFieldType.PM, subBuilder: Expression.create)
    ..aOB(6, _omitFieldNames ? '' : 'isUniversal')
    ..a<$core.List<$core.int>>(7, _omitFieldNames ? '' : 'owner', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RoleBinding clone() => RoleBinding()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RoleBinding copyWith(void Function(RoleBinding) updates) => super.copyWith((message) => updates(message as RoleBinding)) as RoleBinding;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoleBinding create() => RoleBinding._();
  RoleBinding createEmptyInstance() => create();
  static $pb.PbList<RoleBinding> createRepeated() => $pb.PbList<RoleBinding>();
  @$core.pragma('dart2js:noInline')
  static RoleBinding getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RoleBinding>(create);
  static RoleBinding? _defaultInstance;

  /// Unique identifier (uuid) for the RoleBinding.
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  /// Human-readable name for the RoleBinding.
  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  /// Reference to the Role being bound. This is expected to be the Role's ID.
  @$pb.TagNumber(3)
  $core.List<$core.int> get role => $_getN(2);
  @$pb.TagNumber(3)
  set role($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasRole() => $_has(2);
  @$pb.TagNumber(3)
  void clearRole() => $_clearField(3);

  /// List of subjects (public keys) this role binding applies to.
  @$pb.TagNumber(4)
  $pb.PbList<$core.List<$core.int>> get subjects => $_getList(3);

  /// List of expressions that further refine the scope of the role binding.
  @$pb.TagNumber(5)
  $pb.PbList<Expression> get expressions => $_getList(4);

  /// If true, this RoleBinding applies universally, regardless of expressions or subjects.
  @$pb.TagNumber(6)
  $core.bool get isUniversal => $_getBF(5);
  @$pb.TagNumber(6)
  set isUniversal($core.bool v) { $_setBool(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasIsUniversal() => $_has(5);
  @$pb.TagNumber(6)
  void clearIsUniversal() => $_clearField(6);

  /// ID (public key) of the owner of this RoleBinding.
  @$pb.TagNumber(7)
  $core.List<$core.int> get owner => $_getN(6);
  @$pb.TagNumber(7)
  set owner($core.List<$core.int> v) { $_setBytes(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasOwner() => $_has(6);
  @$pb.TagNumber(7)
  void clearOwner() => $_clearField(7);
}

/// Expression represents a conditional expression that refines the scope of a RoleBinding.
class Expression extends $pb.GeneratedMessage {
  factory Expression({
    $core.String? collection,
    $core.String? expression,
  }) {
    final $result = create();
    if (collection != null) {
      $result.collection = collection;
    }
    if (expression != null) {
      $result.expression = expression;
    }
    return $result;
  }
  Expression._() : super();
  factory Expression.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Expression.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Expression', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'collection')
    ..aOS(2, _omitFieldNames ? '' : 'expression')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Expression clone() => Expression()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Expression copyWith(void Function(Expression) updates) => super.copyWith((message) => updates(message as Expression)) as Expression;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Expression create() => Expression._();
  Expression createEmptyInstance() => create();
  static $pb.PbList<Expression> createRepeated() => $pb.PbList<Expression>();
  @$core.pragma('dart2js:noInline')
  static Expression getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Expression>(create);
  static Expression? _defaultInstance;

  /// The collection the expression applies to.
  @$pb.TagNumber(1)
  $core.String get collection => $_getSZ(0);
  @$pb.TagNumber(1)
  set collection($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCollection() => $_has(0);
  @$pb.TagNumber(1)
  void clearCollection() => $_clearField(1);

  /// The actual expression string. The syntax and semantics of this string are context-dependent.
  @$pb.TagNumber(2)
  $core.String get expression => $_getSZ(1);
  @$pb.TagNumber(2)
  set expression($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasExpression() => $_has(1);
  @$pb.TagNumber(2)
  void clearExpression() => $_clearField(2);
}

/// Role defines a set of permissions.
class Role extends $pb.GeneratedMessage {
  factory Role({
    $core.List<$core.int>? id,
    $core.List<$core.int>? owner,
    $core.String? name,
    $core.Iterable<Rule>? rules,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    if (owner != null) {
      $result.owner = owner;
    }
    if (name != null) {
      $result.name = name;
    }
    if (rules != null) {
      $result.rules.addAll(rules);
    }
    return $result;
  }
  Role._() : super();
  factory Role.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Role.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Role', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'owner', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..pc<Rule>(4, _omitFieldNames ? '' : 'rules', $pb.PbFieldType.PM, subBuilder: Rule.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Role clone() => Role()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Role copyWith(void Function(Role) updates) => super.copyWith((message) => updates(message as Role)) as Role;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Role create() => Role._();
  Role createEmptyInstance() => create();
  static $pb.PbList<Role> createRepeated() => $pb.PbList<Role>();
  @$core.pragma('dart2js:noInline')
  static Role getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Role>(create);
  static Role? _defaultInstance;

  /// Unique identifier for the Role.
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  /// ID (public key) of the owner of this Role.
  @$pb.TagNumber(2)
  $core.List<$core.int> get owner => $_getN(1);
  @$pb.TagNumber(2)
  set owner($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasOwner() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwner() => $_clearField(2);

  /// Human-readable name for the Role.
  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => $_clearField(3);

  /// List of rules that define the permissions granted by this role.
  @$pb.TagNumber(4)
  $pb.PbList<Rule> get rules => $_getList(3);
}

/// Rule specifies permissions on a particular collection.
class Rule extends $pb.GeneratedMessage {
  factory Rule({
    $core.String? collection,
    $core.Iterable<$7.Value>? instanceKeys,
    $core.Iterable<Rule_Verb>? verbs,
    $core.Iterable<$7.Value>? excludedInstanceKeys,
  }) {
    final $result = create();
    if (collection != null) {
      $result.collection = collection;
    }
    if (instanceKeys != null) {
      $result.instanceKeys.addAll(instanceKeys);
    }
    if (verbs != null) {
      $result.verbs.addAll(verbs);
    }
    if (excludedInstanceKeys != null) {
      $result.excludedInstanceKeys.addAll(excludedInstanceKeys);
    }
    return $result;
  }
  Rule._() : super();
  factory Rule.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Rule.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Rule', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOS(2, _omitFieldNames ? '' : 'collection')
    ..pc<$7.Value>(3, _omitFieldNames ? '' : 'instanceKeys', $pb.PbFieldType.PM, subBuilder: $7.Value.create)
    ..pc<Rule_Verb>(4, _omitFieldNames ? '' : 'verbs', $pb.PbFieldType.KE, valueOf: Rule_Verb.valueOf, enumValues: Rule_Verb.values, defaultEnumValue: Rule_Verb.READ)
    ..pc<$7.Value>(5, _omitFieldNames ? '' : 'excludedInstanceKeys', $pb.PbFieldType.PM, subBuilder: $7.Value.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Rule clone() => Rule()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Rule copyWith(void Function(Rule) updates) => super.copyWith((message) => updates(message as Rule)) as Rule;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Rule create() => Rule._();
  Rule createEmptyInstance() => create();
  static $pb.PbList<Rule> createRepeated() => $pb.PbList<Rule>();
  @$core.pragma('dart2js:noInline')
  static Rule getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Rule>(create);
  static Rule? _defaultInstance;

  /// The collection this rule applies to.
  @$pb.TagNumber(2)
  $core.String get collection => $_getSZ(0);
  @$pb.TagNumber(2)
  set collection($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(2)
  $core.bool hasCollection() => $_has(0);
  @$pb.TagNumber(2)
  void clearCollection() => $_clearField(2);

  /// Optional list of instance keys. If provided, the rule only applies to these specific instances within the collection.
  @$pb.TagNumber(3)
  $pb.PbList<$7.Value> get instanceKeys => $_getList(1);

  /// List of verbs allowed for this rule.
  @$pb.TagNumber(4)
  $pb.PbList<Rule_Verb> get verbs => $_getList(2);

  /// Exclusion: If present, these instance keys are explicitly NOT allowed
  @$pb.TagNumber(5)
  $pb.PbList<$7.Value> get excludedInstanceKeys => $_getList(3);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
