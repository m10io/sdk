import 'package:grpc/grpc.dart';
import 'package:m10_sdk/m10_sdk.dart';
import 'package:test/test.dart';
import 'utilities/utility.dart';

Future<void> expectUserToContainAccount({
  required M10Sdk sdk,
  required String userId,
  required String operator,
  required String accountId,
  int maxAttempts = 20,
  Duration interval = const Duration(milliseconds: 250),
}) async {
  for (var attempt = 1; attempt <= maxAttempts; attempt++) {
    final user = await sdk.getUser(userId: userId, operator: operator);
    final found = user.accounts.any((account) => account == accountId);
    if (found) {
      return;
    }
    if (attempt < maxAttempts) {
      await Future<void>.delayed(interval);
    }
  }
  fail(
    'Account $accountId not found for user $userId after $maxAttempts attempts',
  );
}

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
        const accountId = '05800002000000003d00000000000003';
        await userSdk.updateUser(
          userId: userId,
          accounts: [accountId],
          operator: operator,
        );

        await expectUserToContainAccount(
          sdk: userSdk,
          userId: userId,
          operator: operator,
          accountId: accountId,
        );
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
      Object? exception;
      try {
        await bankAdmin.getUser(
          userId: userId,
          operator: operator,
        );
      } catch (e) {
        exception = e;
      }
      expect(exception, isNotNull);
      expect(exception is GrpcError, isTrue);
      expect((exception as GrpcError?)!.code, 5);
    });
  });
}
