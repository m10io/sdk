import 'package:m10_sdk/library.dart';
import 'package:m10_sdk/metadata.dart';
import 'package:test/test.dart';

import 'utilities/utility.dart';

void main() {
  group('Transfers', () {
    final account = parentAccountId;
    final targetAccount = bobsAccountId;

    test('it should create a transfer, get a transfer by id', () async {
      final response = await bankAdmin.createTransfer(
        fromAccountId: account,
        toAccountId: targetAccount,
        amount: 100,
        memo: "Funds",
        operator: operator,
      );

      final transfer = await bankAdmin.getTransfer(
        txId: response.txId,
        operator: operator,
      );
      final step = (await bankAdmin.enhance(transfer))[0];

      expect(step.fromAccountId, account);
      expect(step.toAccountId, targetAccount);
      expect(step.senderName, "DartTB TTT");
      expect(step.receiverName, bobsName);
    });

    test('it should get a list of transactions', () async {
      final transfers = await bankAdmin.listTransfers(
        accountId: targetAccount,
        operator: operator,
      );

      expect(transfers.isNotEmpty, true);
    });

    test('transfer with fees', () async {
      // response from GET https://develop.m10.net/trust-oxide/api/v1/fees/usd/48000:
      // ignore: unused_local_variable
      final feeResponse = """
{
  "fees": {
    "Diapay": {
      "account": "00800000000000000200000000000003",
      "amount": 2450
    },
    "M10": {
      "account": "00800000000000000100000000000003",
      "amount": 2450
    }
  }
}""";

      final steps = [
        TransferStepDoc.fromFields(
            fromAccountId: aliceAccountId,
            toAccountId: bobsAccountId,
            amount: 48000,
            metadata: [Metadata.memo("transfer")]),
        TransferStepDoc.fromFields(
            fromAccountId: aliceAccountId,
            toAccountId: parentAccountId,
            amount: 2450,
            metadata: [Metadata.fee()]),
        TransferStepDoc.fromFields(
            fromAccountId: aliceAccountId,
            toAccountId: parentAccountId,
            amount: 2450,
            metadata: [Metadata.fee()])
      ];
      final response = await alice.transfer(
        steps: steps,
        operator: operator,
      );
      expect(response.txId > 0, true);
      final transfer = await alice.getTransfer(
        txId: response.txId,
        operator: operator,
      );
      expect(transfer.steps.length, 3);
    });
  });
}
