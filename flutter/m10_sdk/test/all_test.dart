import 'account_test.dart' as account;
import 'alias_test.dart' as alias;
import 'user_test.dart' as user;
import 'registration_test.dart' as registration;
import 'transfer_tests.dart' as transfer;
import 'image_test.dart' as image;
import 'directory_test.dart' as directory;
import 'observation_test.dart' as observation;
import 'action_test.dart' as action;
import 'transaction_test.dart' as transaction;
import 'utilities/utility.dart';

Future<void> main() async {
  await Utility.init();

  // Print out ledger connection info
  printLedgerInfo();

  // Run all tests
  account.main();
  action.main();
  alias.main();
  transaction.main();
  user.main();
  registration.main();
  transfer.main();
  image.main();
  directory.main();
  observation.main();
}
