import 'dart:convert';

import 'package:m10_sdk/library.dart';
import 'package:m10_sdk/m10_sdk.dart';
import 'package:test/test.dart';
import 'utilities/utility.dart';

void main() {
  group('Registration Flow', () {
    late final M10Sdk userSdk;

    setUpAll(() async {
      userSdk = await Utility.newUser();
    });

    test(
        'it should register a new user, create an account, activate the account and issue funds to the new account',
        () async {
      final userId = await userSdk.createUser(
        instrument: instrument,
      );

      final owner = base64Encode(await bankAdmin.signer.publicKey());
      final issuanceAccount = (await bankAdmin.findAccountByOwner(
              owner: owner, instrument: instrument))
          .first;
      final accountId = await bankAdmin.createAccount(
        parentId: issuanceAccount.id,
        instrument: instrument,
        owner: base64Encode(await userSdk.signer.publicKey()),
      );

      final accountRef = AccountRefDoc.fromIds('$instrument.m10', accountId);

      await userSdk.updateUser(
        userId: userId,
        instrument: instrument,
        accounts: [accountRef.model],
      );

      print("Get user");

      final user = await userSdk.getUser(
        userId: userId,
        instrument: instrument,
      );
      expect(user.model.accounts.length, 1);

      print("Update account status");

      await bankAdmin.updateAccountStatus(
          id: accountRef.accountId, instrument: instrument, frozen: false);

      print("creating transfer");

      await bankAdmin.createTransfer(
        fromAccountId: issuanceAccount.id,
        toAccountId: accountRef.accountId,
        amount: 100,
        instrument: instrument,
      );

      await userSdk.listTransfers(
        accountId: accountRef.accountId,
        instrument: instrument,
      );
    }, timeout: Timeout(Duration(seconds: 60)));
  });
}
