import 'package:m10_sdk/library.dart';

const String ATTACHMENT_TYPE_URL = "m10.sdk.metadata.Attachment";
const String MEMO_TYPE_URL = "m10.sdk.metadata.Memo";
const String FEE_TYPE_URL = "m10.sdk.metadata.Fee";
const String WITHDRAW_TYPE_URL = "m10.sdk.metadata.Withdraw";
const String DEPOSIT_TYPE_URL = "m10.sdk.metadata.Deposit";
const String CONTRACT_TYPE_URL = "m10.sdk.metadata.Contract";
const String SELF_TRANSFER_TYPE_URL = "m10.sdk.metadata.SelfTransfer";
const String REBALANCE_TRANSFER_TYPE_URL = "m10.sdk.metadata.RebalanceTransfer";

extension MetadataExt on List<Any> {
  List<Attachment> get attachments {
    return this
        .where((a) => a.typeUrl == ATTACHMENT_TYPE_URL)
        .map((a) => Attachment.fromBuffer(a.value))
        .toList();
  }

  String get memo {
    for (final a in this) {
      if (a.typeUrl == MEMO_TYPE_URL) {
        return Attachment.fromBuffer(a.value).objectId;
      }
    }
    return '';
  }

  SelfTransfer? get selfTransfer {
    for (final a in this) {
      if (a.typeUrl == SELF_TRANSFER_TYPE_URL) {
        return SelfTransfer.fromBuffer(a.value);
      }
    }
    return null;
  }

  String? get bankId {
    for (final a in this) {
      switch (a.typeUrl) {
        case WITHDRAW_TYPE_URL:
          return Withdraw.fromBuffer(a.value).bankAccountId;
        case DEPOSIT_TYPE_URL:
          return Deposit.fromBuffer(a.value).bankAccountId;
      }
    }
    return null;
  }

  Contract? get contract {
    for (final a in this) {
      if (a.typeUrl == CONTRACT_TYPE_URL) {
        return Contract.fromBuffer(a.value);
      }
    }
    return null;
  }

  bool get isFee => this.where((a) => a.typeUrl == FEE_TYPE_URL).isNotEmpty;

  bool get isDeposit =>
      this.where((a) => a.typeUrl == DEPOSIT_TYPE_URL).isNotEmpty;

  bool get isWithdraw =>
      this.where((a) => a.typeUrl == WITHDRAW_TYPE_URL).isNotEmpty;

  bool get isSelfTransfer => selfTransfer != null;

  bool get isRebalanceTransfer =>
      where((a) => a.typeUrl == REBALANCE_TRANSFER_TYPE_URL).isNotEmpty;

  bool get isFx => this.where((a) => a.typeUrl == CONTRACT_TYPE_URL).isNotEmpty;
}

abstract class Metadata {
  static Any attachment(Attachment attachment) => Any(
        typeUrl: ATTACHMENT_TYPE_URL,
        value: attachment.writeToBuffer(),
      );

  static Any memo(String plaintext) => Any(
        typeUrl: MEMO_TYPE_URL,
        value: Memo(plaintext: plaintext).writeToBuffer(),
      );

  static Any fee() => Any(
        typeUrl: FEE_TYPE_URL,
        value: Fee().writeToBuffer(),
      );

  static Any contract(Contract contract) => Any(
        typeUrl: CONTRACT_TYPE_URL,
        value: contract.writeToBuffer(),
      );

  static Any withdraw(String bankAccountId) => Any(
        typeUrl: WITHDRAW_TYPE_URL,
        value: Withdraw(bankAccountId: bankAccountId).writeToBuffer(),
      );

  static Any deposit(String bankAccountId) => Any(
        typeUrl: DEPOSIT_TYPE_URL,
        value: Deposit(bankAccountId: bankAccountId).writeToBuffer(),
      );

  static Any selfTransfer(SelfTransfer selfTransfer) => Any(
        typeUrl: SELF_TRANSFER_TYPE_URL,
        value: selfTransfer.writeToBuffer(),
      );
}
