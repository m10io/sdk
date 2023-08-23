import 'package:convert/convert.dart';
import 'package:m10_sdk/src/generated/sdk/document.pb.dart';
import 'package:test/test.dart';
import 'utilities/utility.dart';

void main() {
  group('Observations', () {
    final account = '00800001800000000000000000000004';
    final targetAccount = '00800001800000000000000000000005';

    test('it should observe a successful transfer', () async {
      final stream = await bankAdmin
          .observeTransfers(operator: operator, accounts: [account]);

      final transaction = await bankAdmin.createTransfer(
        fromAccountId: account,
        toAccountId: targetAccount,
        amount: 100,
        memo: 'observation test',
        operator: operator,
      );

      await stream.timeout(
        Duration(seconds: 10),
        onTimeout: (sink) {
          sink.close();
          fail('Did not observe message in time');
        },
      ).any(
        (transferResults) =>
            transferResults.any((result) => result.txId == transaction.txId),
      );
    });

    test('it should observe a resource change', () async {
      final dennisAccountId =
          await Utility.createAccount(bankAdmin, publicName: 'Dennis V1');

      final stream = await bankAdmin.observeResources(
        operator: operator,
        collection: 'account-metadata',
        expression: '|id, doc| id == account',
        variables: {
          'account': Value()..bytesValue = hex.decode(dennisAccountId),
        },
      );

      final updateTxnId = await bankAdmin.updateAccount(
        id: dennisAccountId,
        operator: operator,
        publicName: 'Dennis V2',
      );

      final operations = await stream.timeout(
        Duration(seconds: 10),
        onTimeout: (sink) {
          sink.close();
          fail('Did not observe resource change in time');
        },
      ).firstWhere(
        (resourceResults) =>
            resourceResults.any((result) => result.txId == updateTxnId),
      );

      expect(operations.length, 1);
    });
  });
}
