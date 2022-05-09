import 'package:m10_sdk/library.dart';
import 'package:test/test.dart';
import 'package:random_string/random_string.dart';
import 'utilities/utility.dart';

void main() {
  late String handle;

  group('Alias', () {
    test('it should create a new alias', () async {
      handle =
          '${randomAlpha(10)}${DateTime.now().toUtc().millisecondsSinceEpoch}';
      await directory.createAlias(
          handle: handle,
          displayName: '',
          accountSetId: aliceId,
          aliasType: Alias_Type.HANDLE,
          operator: operator,
          instrument: instrument);
    });

    test('it should resolve an alias', () async {
      final accountId = await bob.createAccount(
        parentId: parentAccountId,
        name: "Spending Account",
        instrument: instrument,
      );
      final ledgerId = "$instrument.m10";
      final accountRef = AccountRefDoc.fromIds(ledgerId, accountId);
      await bob.updateUser(
          userId: bobId, instrument: instrument, accounts: [accountRef.model]);

      final handle = randomAlpha(10);

      await directory.createAlias(
          handle: handle,
          displayName: '',
          accountSetId: bobId,
          aliasType: Alias_Type.HANDLE,
          operator: operator,
          instrument: instrument);

      final alias = await directory.getAlias(handle);
      expect(alias?.accountSetId, bobId);

      final resolvedAccounts = await alice.getUser(
        userId: bobId,
        instrument: instrument,
      );
      expect(resolvedAccounts.accounts.contains(accountRef), true);
    });
  });
}
