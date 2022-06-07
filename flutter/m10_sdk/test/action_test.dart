import 'dart:io';

import 'package:fixnum/fixnum.dart';

import 'package:convert/convert.dart';
import 'package:m10_sdk/library.dart';
import 'package:m10_sdk/m10_sdk.dart';
import 'package:m10_sdk/metadata.dart';
import 'package:test/test.dart';

import 'utilities/utility.dart';

void main() {
  group('Actions', () {
    final account = parentAccountId;
    final targetAccount = bobsAccountId;

    test('it should invoke an action', () async {
      final transactionId = await bankAdmin.invokeAction(
        name: "m10.Request",
        fromAccountId: account,
        targetAccountId: targetAccount,
        payload: "This is a test action".codeUnits,
        operator: operator,
      );

      expect(transactionId.txId > 0, true);
    });

    test('it should get an action by ID', () async {
      final payload = "This is a test action".codeUnits;
      String name = "m10.Request";
      final response = await bankAdmin.invokeAction(
        name: name,
        fromAccountId: account,
        targetAccountId: targetAccount,
        payload: payload,
        operator: operator,
      );

      final action = await bankAdmin.getAction(
        txId: response.txId,
        operator: operator,
      );

      expect(action.fromAccountId, account);
      expect(action.targetAccountId, targetAccount);
      expect(action.name, name);
      expect(action.contextId, null);
      expect(action.payload, payload);
    });

    test('it should list actions', () async {
      String name = "m10.Request";
      final first = await bankAdmin.invokeAction(
        name: name,
        fromAccountId: account,
        targetAccountId: targetAccount,
        payload: [1],
        operator: operator,
      );

      final second = await bankAdmin.invokeAction(
        name: name,
        fromAccountId: account,
        targetAccountId: targetAccount,
        payload: [2],
        operator: operator,
      );

      final actions = await bankAdmin.listActions(
          operator: operator,
          name: name,
          accountId: account,
          minTxId: first.txId,
          maxTxId: second.txId);

      expect(actions.length, 2);
      actions.forEach((action) {
        expect(action.fromAccountId, account);
        expect(action.targetAccountId, targetAccount);
        expect(action.name, name);
        expect(action.contextId, null);
      });
      expect(actions[0].payload, [2], reason: "First payload should be '2'");
      expect(actions[1].payload, [1], reason: "First payload should be '1'");
    });

    test('it should filter actions by name', () async {
      String name = "m10.Request";
      final first = await bankAdmin.invokeAction(
        name: name,
        fromAccountId: account,
        targetAccountId: targetAccount,
        payload: [1],
        operator: operator,
      );

      final second = await bankAdmin.invokeAction(
        name: name,
        fromAccountId: account,
        targetAccountId: targetAccount,
        payload: [2],
        operator: operator,
      );

      bool didFail = false;
      try {
        await alice.getAction(
          txId: first.txId,
          operator: operator,
        );
      } catch (e) {
        didFail = true;
      }
      expect(didFail, true, reason: "Retrieving action did not fail");

      final actions = await alice.listActions(
          operator: operator,
          name: "not-a-request",
          accountId: account,
          minTxId: first.txId,
          maxTxId: second.txId);

      expect(actions.isEmpty, true, reason: "Listed actions were not empty");
    });

    test('it should cancel a request', () async {
      final requestResponse = await bankAdmin.request(
          operator: operator,
          transferRequest: CreateTransfer(
            transferSteps: [
              TransferStep(
                  fromAccountId: hex.decode(targetAccount),
                  toAccountId: hex.decode(account),
                  amount: Int64(100))
            ],
          ));
      expect(requestResponse.hasError, false);

      final requestAction =
          await bob.getAction(txId: requestResponse.txId, operator: operator);
      expect(requestAction.name, M10Actions.requestActionName);

      final cancelResponse = await bob.cancel(
          operator: operator,
          fromAccountId: targetAccount,
          targetAccountId: account,
          contextId: requestAction.contextId!);
      expect(cancelResponse.hasError, false);

      final cancelAction = await bankAdmin.getAction(
          txId: cancelResponse.txId, operator: operator);
      expect(cancelAction.name, M10Actions.requestActionName);
      final cancelPayload = PaymentRequest.fromBuffer(cancelAction.payload);
      expect(
          cancelPayload.status, PaymentRequest_PaymentRequestStatus.CANCELED);
      expect(cancelAction.contextId, requestAction.contextId);
    });

    test('it should list requests', () async {
      final rawAccount = hex.decode(account);
      final rawTargetAccount = hex.decode(targetAccount);
      final request1 = CreateTransfer(transferSteps: [
        TransferStep(
            amount: Int64(2000),
            fromAccountId: rawTargetAccount,
            toAccountId: rawAccount,
            metadata: [Metadata.memo("First")])
      ]);
      final first = await bankAdmin.request(
          operator: operator, transferRequest: request1);

      final request2 = CreateTransfer(transferSteps: [
        TransferStep(
            amount: Int64(2500),
            fromAccountId: rawTargetAccount,
            toAccountId: rawAccount,
            metadata: [Metadata.memo("Second")])
      ]);
      final second = await bankAdmin.request(
          operator: operator, transferRequest: request2);

      final request3 = CreateTransfer(transferSteps: [
        TransferStep(
            amount: Int64(2600),
            fromAccountId: rawAccount,
            toAccountId: rawTargetAccount,
            metadata: [Metadata.memo("Third")])
      ]);
      final third = await bankAdmin.request(
          operator: operator, transferRequest: request3);

      await bankAdmin.cancel(
          operator: operator,
          fromAccountId: account,
          targetAccountId: targetAccount,
          contextId: second.contextId!);

      final completion = await bankAdmin.createTransfer(
        fromAccountId: account,
        toAccountId: targetAccount,
        amount: 2600,
        memo: "Funds",
        operator: operator,
        contextId: third.contextId!,
      );
      sleep(Duration(seconds: 2));

      final List<PaymentRequestDoc> requests = await bankAdmin.listRequests(
          operator: operator,
          accountId: account,
          minTxId: first.txId,
          maxTxId: completion.txId,
          limit: 3);

      final pendingReq =
          requests.firstWhere((req) => req.contextId == first.contextId);
      final cancledReq =
          requests.firstWhere((req) => req.contextId == second.contextId);
      final completedReq =
          requests.firstWhere((req) => req.contextId == third.contextId);

      expect(pendingReq.status, RequestStatus.pending,
          reason: "First request should be 'pending'");
      expect(cancledReq.status, RequestStatus.canceled,
          reason: "Second request should be 'canceled'");
      expect(completedReq.status, RequestStatus.completed,
          reason: "Third request should be 'completed'");
    });
  });
}
