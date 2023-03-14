import 'package:m10_sdk/library.dart';

const attachmentTypeUrl = 'm10.sdk.metadata.Attachment';
const memoTypeUrl = 'm10.sdk.metadata.Memo';
const feeTypeUrl = 'm10.sdk.metadata.Fee';
const withdrawTypeUrl = 'm10.sdk.metadata.Withdraw';
const depositTypeUrl = 'm10.sdk.metadata.Deposit';
const contractTypeUrl = 'm10.sdk.metadata.Contract';
const selfTransferTypeUrl = 'm10.sdk.metadata.SelfTransfer';
const rebalanceTransferTypeUrl = 'm10.sdk.metadata.RebalanceTransfer';

extension MetadataExt on List<Any> {
  List<Attachment> get attachments =>
      where((a) => a.typeUrl == attachmentTypeUrl)
          .map((a) => Attachment.fromBuffer(a.value))
          .toList();

  String get memo {
    for (final a in this) {
      if (a.typeUrl == memoTypeUrl) {
        return Attachment.fromBuffer(a.value).objectId;
      }
    }
    return '';
  }

  SelfTransfer? get selfTransfer {
    for (final a in this) {
      if (a.typeUrl == selfTransferTypeUrl) {
        return SelfTransfer.fromBuffer(a.value);
      }
    }
    return null;
  }

  String? get bankId {
    for (final a in this) {
      switch (a.typeUrl) {
        case withdrawTypeUrl:
          return Withdraw.fromBuffer(a.value).bankAccountId;
        case depositTypeUrl:
          return Deposit.fromBuffer(a.value).bankAccountId;
      }
    }
    return null;
  }

  Contract? get contract {
    for (final a in this) {
      if (a.typeUrl == contractTypeUrl) {
        return Contract.fromBuffer(a.value);
      }
    }
    return null;
  }

  bool get isFee => where((a) => a.typeUrl == feeTypeUrl).isNotEmpty;

  bool get isDeposit => where((a) => a.typeUrl == depositTypeUrl).isNotEmpty;

  bool get isWithdraw => where((a) => a.typeUrl == withdrawTypeUrl).isNotEmpty;

  bool get isSelfTransfer => selfTransfer != null;

  bool get isRebalanceTransfer =>
      where((a) => a.typeUrl == rebalanceTransferTypeUrl).isNotEmpty;

  bool get isFx => where((a) => a.typeUrl == contractTypeUrl).isNotEmpty;
}

abstract class Metadata {
  static Any attachment(Attachment attachment) => Any(
        typeUrl: attachmentTypeUrl,
        value: attachment.writeToBuffer(),
      );

  static Any memo(String plaintext) => Any(
        typeUrl: memoTypeUrl,
        value: Memo(plaintext: plaintext).writeToBuffer(),
      );

  static Any fee() => Any(
        typeUrl: feeTypeUrl,
        value: Fee().writeToBuffer(),
      );

  static Any contract(Contract contract) => Any(
        typeUrl: contractTypeUrl,
        value: contract.writeToBuffer(),
      );

  static Any withdraw(String bankAccountId) => Any(
        typeUrl: withdrawTypeUrl,
        value: Withdraw(bankAccountId: bankAccountId).writeToBuffer(),
      );

  static Any deposit(String bankAccountId) => Any(
        typeUrl: depositTypeUrl,
        value: Deposit(bankAccountId: bankAccountId).writeToBuffer(),
      );

  static Any selfTransfer(SelfTransfer selfTransfer) => Any(
        typeUrl: selfTransferTypeUrl,
        value: selfTransfer.writeToBuffer(),
      );
}
