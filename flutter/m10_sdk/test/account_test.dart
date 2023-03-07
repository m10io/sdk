import 'dart:io';

import 'package:m10_sdk/library.dart';
import 'package:test/test.dart';
import 'utilities/utility.dart';

void main() {
  late String accountId;

  group('Account', () {
    test('it should create a new account', () async {
      accountId = await Utility.createAccount(alice);

      final account = await alice.getAccountMetadata(
        id: accountId,
        operator: operator,
      );

      expect(account.id, accountId);
    });

    test('it should get an existing account', () async {
      final AccountMetadataDoc account = await alice.getAccountMetadata(
        id: accountId,
        operator: operator,
      );

      expect(account.id, accountId);
    }, skip: false);

    test('it should update an existing account', () async {
      final publicName = "Alice R.";
      final profileImageUrl = "https://fake.m10.net/images/alice";
      await bankAdmin.updateAccount(
        id: accountId,
        operator: operator,
        publicName: publicName,
        profileImageUrl: profileImageUrl,
      );

      sleep(Duration(milliseconds: 200));

      final AccountMetadataDoc account = await alice.getAccountMetadata(
        id: accountId,
        operator: operator,
      );

      expect(account.publicName, publicName);
      expect(account.profileImageUrl, profileImageUrl);
    }, skip: false);
  });
}
