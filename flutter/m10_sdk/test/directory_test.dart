import 'package:test/test.dart';
import 'utilities/utility.dart';

void main() {
  group('Directory', () {
    test('it should get all ledgers', () async {
      final ledgers = await directory.listLedgers();
      expect(ledgers.isEmpty, false);
    });
  });
}
