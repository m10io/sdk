import 'dart:convert';

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
        'it should register a new user, create an account, activate the account'
        ' and issue funds to the new account', () async {
      final userId = await userSdk.createUser(
        operator: operator,
      );

      final owner = base64Encode(await bankAdmin.signer.publicKey());
      final issuanceAccount = (await bankAdmin.findAccountByOwner(
        owner: owner,
        operator: operator,
      ))
          .first;
      final accountId = await bankAdmin.createAccount(
        parentId: issuanceAccount.id,
        operator: operator,
        owner: base64Encode(await userSdk.signer.publicKey()),
      );

      await userSdk.updateUser(
        userId: userId,
        operator: operator,
        accounts: [accountId],
      );

      print('Get user');

      final user = await userSdk.getUser(
        userId: userId,
        operator: operator,
      );
      expect(user.model.accounts.length, 1);

      print('Update account status');

      await bankAdmin.updateAccountStatus(
          id: accountId, operator: operator, frozen: false,);

      print('creating transfer');

      await bankAdmin.createTransfer(
        fromAccountId: issuanceAccount.id,
        toAccountId: accountId,
        amount: 100,
        operator: operator,
      );

      await userSdk.listTransfers(
        accountId: accountId,
        operator: operator,
      );
    }, timeout: Timeout(Duration(seconds: 60)),);
  });
}
