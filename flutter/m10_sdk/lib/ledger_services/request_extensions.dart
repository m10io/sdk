import 'dart:core';

import 'package:protobuf/protobuf.dart';

import 'package:m10_sdk/ledger_services/document_store/builder.dart';
import 'package:m10_sdk/src/generated/sdk/document.pb.dart';
import 'package:m10_sdk/src/generated/google/protobuf/field_mask.pb.dart';
import 'package:m10_sdk/library.dart';

extension DocumentOperation on GeneratedMessage {
  TransactionRequestPayload createTransactionFrom(String collection) {
    final insertOp = Operation_InsertDocument()
      ..collection = collection
      ..document = writeToBuffer();
    final operation = Operation()..insertDocument = insertOp;

    final documentOperations = DocumentOperations();
    documentOperations.operations
      ..clear()
      ..add(operation);

    return TransactionRequestPayload()
      ..data = (TransactionData()..documentOperations = documentOperations);
  }

  TransactionRequestPayload updateTransactionFrom(
    List<int> id,
    String collection,
    FieldMask mask,
  ) {
    final primaryKey = Value()..bytesValue = id;
    final updateOp = Operation_UpdateDocument()
      ..collection = collection
      ..document = writeToBuffer()
      ..primaryKey = primaryKey
      ..fieldMask = mask;

    final operation = Operation()..updateDocument = updateOp;
    final documentOperations = DocumentOperations();
    documentOperations.operations
      ..clear()
      ..add(operation);

    final data = TransactionData()..documentOperations = documentOperations;
    return TransactionRequestPayload()..data = data;
  }

  static TransactionRequestPayload deleteTransactionFrom(
    List<int> id,
    String collection,
  ) {
    final primaryKey = Value()..bytesValue = id;
    final deleteOp = Operation_DeleteDocument()
      ..collection = collection
      ..primaryKey = primaryKey;
    final operation = Operation()..deleteDocument = deleteOp;
    final operations = DocumentOperations();
    operations.operations
      ..clear()
      ..add(operation);
    return TransactionRequestPayload()
      ..data = (TransactionData()..documentOperations = operations);
  }
}

extension AccountMetadataExt on AccountMetadata {
  static final _collection = 'account-metadata';

  TransactionRequestPayload createRequest() =>
      createTransactionFrom(_collection);

  static TransactionRequestPayload updateRequest(
    DocumentUpdate<AccountMetadata> builder,
  ) {
    final account = builder.document;
    return account.updateTransactionFrom(account.id, _collection, builder.mask);
  }

  static TransactionRequestPayload deleteRequest(List<int> id) =>
      DocumentOperation.deleteTransactionFrom(id, _collection);
}

extension AccountSetExt on AccountSet {
  static final _collection = 'account-sets';

  TransactionRequestPayload createRequest() =>
      createTransactionFrom(_collection);

  static TransactionRequestPayload updateRequest(
    DocumentUpdate<AccountSet> builder,
  ) {
    final user = builder.document;
    return user.updateTransactionFrom(user.id, _collection, builder.mask);
  }

  static TransactionRequestPayload deleteRequest(List<int> id) =>
      DocumentOperation.deleteTransactionFrom(id, _collection);
}

extension RoleBindingExt on RoleBinding {
  static final _collection = 'role-bindings';

  TransactionRequestPayload createRequest() =>
      createTransactionFrom(_collection);

  static TransactionRequestPayload updateRequest(
    DocumentUpdate<RoleBinding> builder,
  ) {
    final roleBinding = builder.document;
    return roleBinding.updateTransactionFrom(
      roleBinding.id,
      _collection,
      builder.mask,
    );
  }

  static TransactionRequestPayload deleteRequest(List<int> id) =>
      DocumentOperation.deleteTransactionFrom(id, _collection);
}

extension RoleExt on Role {
  static final _collection = 'roles';

  TransactionRequestPayload createRequest() =>
      createTransactionFrom(_collection);

  static TransactionRequestPayload updateRequest(DocumentUpdate<Role> builder) {
    final role = builder.document;
    return role.updateTransactionFrom(role.id, _collection, builder.mask);
  }

  static TransactionRequestPayload deleteRequest(List<int> id) =>
      DocumentOperation.deleteTransactionFrom(id, _collection);
}
