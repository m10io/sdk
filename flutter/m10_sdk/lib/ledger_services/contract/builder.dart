import 'dart:math';
import 'dart:typed_data';

import 'package:m10_sdk/library.dart';
import 'package:m10_sdk/metadata.dart';
import 'package:m10_sdk/src/generated/sdk/transaction/transaction.pb.dart'
    as tx;
import 'package:fixnum/fixnum.dart';

class ContractBuilder {
  ContractBuilder() : _transfers = [];

  // Contract transfers
  final List<tx.CreateLedgerTransfer> _transfers;

  void transfer({
    required String ledgerId,
    required Uint8List fromAccountId,
    required Uint8List toAccountId,
    required Int64 amount,
    String? memo,
  }) {
    final metadata = <Any>[];
    if (memo != null) {
      metadata.add(Metadata.memo(memo));
    }

    final rand = Random.secure();
    final transfer = tx.CreateLedgerTransfer(
      ledgerId: ledgerId,
      nonce: Int64.fromInts(rand.nextInt(1 << 32), rand.nextInt(1 << 32)),
      transfer: tx.CreateTransfer(transferSteps: [
        tx.TransferStep(
          fromAccountId: fromAccountId,
          toAccountId: toAccountId,
          amount: amount,
          metadata: metadata,
        )
      ],),
    );

    _transfers.add(transfer);
  }

  Contract build() {
    final validUntil = DateTime.now().add(Duration(minutes: 5));
    final ledgerTransferRequests = tx.CreateLedgerTransfers(
        transfers: _transfers,
        validUntil: Int64(validUntil.microsecondsSinceEpoch),);
    return Contract(transactions: ledgerTransferRequests.writeToBuffer());
  }
}
