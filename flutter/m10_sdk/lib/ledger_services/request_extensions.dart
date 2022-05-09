import 'dart:core';

import 'package:protobuf/protobuf.dart';

import 'package:m10_sdk/ledger_services/document_store/builder.dart';
import 'package:m10_sdk/src/generated/sdk/document.pb.dart';
import 'package:m10_sdk/src/generated/google/protobuf/field_mask.pb.dart';
import 'package:m10_sdk/library.dart';

extension DocumentOperation on GeneratedMessage {
  TransactionRequestPayload createTransactionFrom(String collection) {
    final insertOp = Operation_InsertDocument(
        collection: collection, document: this.writeToBuffer());
    final operation = Operation(insertDocument: insertOp);
    final operations = DocumentOperations(operations: [operation]);
    return TransactionRequestPayload(
        data: TransactionData(documentOperations: operations));
  }

  TransactionRequestPayload updateTransactionFrom(
      List<int> id, String collection, FieldMask mask) {
    final updateOp = Operation_UpdateDocument(
        collection: collection,
        document: this.writeToBuffer(),
        primaryKey: Value(bytesValue: id),
        fieldMask: mask);
    final operation = Operation(updateDocument: updateOp);
    final operations = DocumentOperations(operations: [operation]);
    return TransactionRequestPayload(
        data: TransactionData(documentOperations: operations));
  }

  static TransactionRequestPayload deleteTransactionFrom(
      List<int> id, String collection) {
    final deleteOp = Operation_DeleteDocument(
      collection: collection,
      primaryKey: Value(bytesValue: id),
    );
    final operation = Operation(deleteDocument: deleteOp);
    final operations = DocumentOperations(operations: [operation]);
    return TransactionRequestPayload(
        data: TransactionData(documentOperations: operations));
  }
}

extension AccountExt on Account {
  static String _collection = "accounts";

  TransactionRequestPayload createRequest() {
    return this.createTransactionFrom(_collection);
  }

  static TransactionRequestPayload updateRequest(
      DocumentUpdate<Account> builder) {
    final account = builder.document;
    return account.updateTransactionFrom(account.id, _collection, builder.mask);
  }

  static TransactionRequestPayload deleteRequest(List<int> id) {
    return DocumentOperation.deleteTransactionFrom(id, _collection);
  }
}

extension AccountSetExt on AccountSet {
  static String _collection = "account-sets";

  TransactionRequestPayload createRequest() {
    return this.createTransactionFrom(_collection);
  }

  static TransactionRequestPayload updateRequest(
      DocumentUpdate<AccountSet> builder) {
    final user = builder.document;
    return user.updateTransactionFrom(user.id, _collection, builder.mask);
  }

  static TransactionRequestPayload deleteRequest(List<int> id) {
    return DocumentOperation.deleteTransactionFrom(id, _collection);
  }
}

extension RoleBindingExt on RoleBinding {
  static String _collection = "role-bindings";

  TransactionRequestPayload createRequest() {
    return this.createTransactionFrom(_collection);
  }

  static TransactionRequestPayload updateRequest(
      DocumentUpdate<RoleBinding> builder) {
    final roleBinding = builder.document;
    return roleBinding.updateTransactionFrom(
        roleBinding.id, _collection, builder.mask);
  }

  static TransactionRequestPayload deleteRequest(List<int> id) {
    return DocumentOperation.deleteTransactionFrom(id, _collection);
  }
}

extension RoleExt on Role {
  static String _collection = "roles";

  TransactionRequestPayload createRequest() {
    return this.createTransactionFrom(_collection);
  }

  static TransactionRequestPayload updateRequest(DocumentUpdate<Role> builder) {
    final role = builder.document;
    return role.updateTransactionFrom(role.id, _collection, builder.mask);
  }

  static TransactionRequestPayload deleteRequest(List<int> id) {
    return DocumentOperation.deleteTransactionFrom(id, _collection);
  }
}
