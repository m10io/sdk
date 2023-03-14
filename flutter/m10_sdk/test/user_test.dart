import 'dart:io';

import 'package:grpc/grpc.dart';
import 'package:m10_sdk/library.dart';
import 'package:m10_sdk/m10_sdk.dart';
import 'package:test/test.dart';
import 'utilities/utility.dart';

void main() {
  late final M10Sdk userSdk;
  late String userId;

  group('User', () {
    setUpAll(() async {
      userSdk = await Utility.newUser();
    });

    test('it should create a new user', () async {
      userId = await Utility.createUser(sdk: userSdk);

      expect(userId.isNotEmpty, true);
    });

    test(
      'it should get an existing user',
      () async {
        await userSdk.getUser(
          userId: userId,
          operator: operator,
        );
      },
      skip: false,
    );

    test(
      'it should update an existing user',
      () async {
        await userSdk.updateUser(
          userId: userId,
          accounts: ['ttt.m10/05800002000000003d00000000000003'.parse()],
          operator: operator,
        );

        sleep(Duration(milliseconds: 200));

        final alice = await userSdk.getUser(
          userId: userId,
          operator: operator,
        );

        final found = alice.accounts.any(
          (account) => account.accountId == '05800002000000003d00000000000003',
        );
        expect(found, true);
      },
      skip: false,
    );

    test('it should delete a user', () async {
      // Note: A regular user has no permission to delete
      await userSdk.deleteUser(
        userId: userId,
        operator: operator,
      );

      // Note: Since the record is deleted now, the ownership
      // relation can't be checked anymore, thus the 'user' looses
      // the permission. Admin still can try to get the record.
      // Checks for error "gRPC Error (5, user not found)"
      expect(
        () async => bankAdmin.getUser(
          userId: userId,
          operator: operator,
        ),
        throwsA(predicate((e) => e is GrpcError && e.code == 5)),
      );
    });
  });
}
