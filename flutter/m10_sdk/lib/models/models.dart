import 'dart:convert';
import 'dart:typed_data';

import 'package:convert/convert.dart';
import 'package:equatable/equatable.dart';
import 'package:fixnum/fixnum.dart';
import 'package:m10_sdk/library.dart';
import 'package:m10_sdk/m10_sdk.dart';
import 'package:m10_sdk/security/digest.dart';
import 'package:m10_sdk/src/generated/sdk/document.pb.dart' as document;
import 'package:uuid/uuid.dart';
import 'package:m10_sdk/security/security.dart' as security;
import 'package:m10_sdk/security/request_signing.dart';
import 'package:collection/collection.dart';
import 'package:m10_sdk/metadata.dart';

abstract class _Document<T> {
  final T _model;

  _Document(this._model);

  T get model => _model;
}

class AccountDoc extends _Document<Account> {
  IndexedAccount? _indexedAccount;

  AccountDoc(Account model) : super(model);

  factory AccountDoc.fromModel(
    Account model,
    IndexedAccount indexedAccount,
  ) {
    final accountDoc = AccountDoc(model);
    accountDoc._indexedAccount = indexedAccount;

    return accountDoc;
  }

  factory AccountDoc.fromJson(Map<String, dynamic> json) {
    final model = Account.fromJson(json['model'] as String);
    final indexedAccount = IndexedAccount.fromJson(
      json['indexedAccount'] as String,
    );
    return AccountDoc.fromModel(model, indexedAccount);
  }

  Map<String, dynamic> toJson() {
    return {
      'model': _model.writeToJson(),
      'indexedAccount': _indexedAccount?.writeToJson(),
    };
  }

  String get owner => base64.encode(_model.owner);
  String get id => hex.encode(_model.id);
  String get name => _model.name;
  String get publicName => _model.publicName;
  String get profileImageUrl => _model.profileImageUrl;
  int get balance => _indexedAccount?.balance.toInt() ?? 0;
  String? get instrument => _indexedAccount?.instrument.code;
  int? get decimalPlaces => _indexedAccount?.instrument.decimalPlaces;
  int get issuedBalance => _indexedAccount?.issuance.issuedBalance.toInt() ?? 0;
}

class AccountSetDoc extends _Document<AccountSet> {
  AccountSetDoc(AccountSet model) : super(model);

  String get id => Uuid.unparse(_model.id);
  String get owner => base64.encode(_model.owner);
  List<AccountRefDoc> get accounts =>
      _model.accounts.map((accountRef) => AccountRefDoc(accountRef)).toList();

  @override
  String toString() => 'AccountSet $id $owner accounts=$accounts';
}

class RoleBindingDoc extends _Document<RoleBinding> {
  RoleBindingDoc(RoleBinding model) : super(model);

  String get id => Uuid.unparse(_model.id);
  String get name => _model.name;
  String get role => Uuid.unparse(_model.role);
  List<String> get subjects =>
      _model.subjects.map((s) => base64.encode(s)).toList();
  bool get isUniversal => _model.isUniversal;
}

class TransferDoc extends _Document<FinalizedTransfer> with EquatableMixin {
  TransferDoc(FinalizedTransfer model, this.operator) : super(model);

  factory TransferDoc.fromJson(Map<String, dynamic> json) {
    final model = FinalizedTransfer.fromJson(json['model'] as String);
    final operator = json['operator'] as String;
    return TransferDoc(model, operator);
  }

  Map<String, dynamic> toJson() {
    return {'model': _model.writeToJson(), 'operator': operator};
  }

  final String operator;

  List<TransferStepDoc> get steps =>
      _model.transferSteps.map((step) => TransferStepDoc(step)).toList();

  int get id => _model.txId.toInt();
  bool get failed => _model.hasError();
  List<int>? get contextId =>
      _model.contextId.isEmpty ? null : _model.contextId;
  FinalizedTransfer_TransferState get state => _model.state;

  DateTime get timestamp => DateTime.fromMicrosecondsSinceEpoch(
        _model.timestamp.toInt(),
        isUtc: true,
      );

  @override
  List<Object?> get props => [model.writeToJson(), operator];
}

class TransferStepDoc extends _Document<TransferStep> {
  TransferStepDoc(TransferStep model) : super(model);
  TransferStepDoc.fromFields({
    required String fromAccountId,
    required String toAccountId,
    required int amount,
    List<Any> metadata = const [],
  }) : super(TransferStep(
            fromAccountId: hex.decode(fromAccountId),
            toAccountId: hex.decode(toAccountId),
            amount: Int64(amount),
            metadata: metadata));

  String get fromAccountId => hex.encode(_model.fromAccountId);
  String get toAccountId => hex.encode(_model.toAccountId);
  int get amount => _model.amount.toInt();
  List<Any> get metadata => _model.metadata;
}

class TransferRequestDoc extends _Document<CreateTransfer> {
  TransferRequestDoc(CreateTransfer model) : super(model);
  List<TransferStepDoc> get steps =>
      _model.transferSteps.map((step) => TransferStepDoc(step)).toList();
}

class EnhancedTransferStepDoc extends _Document<TransferStep>
    with EquatableMixin {
  EnhancedTransferStepDoc({
    required TransferStep step,
    required AccountInfo from,
    required AccountInfo to,
    AccountInfo? fromBank,
    AccountInfo? toBank,
  })  : _from = from,
        _to = to,
        _fromBank = fromBank,
        _toBank = toBank,
        super(step);

  factory EnhancedTransferStepDoc.fromJson(Map<String, dynamic> json) {
    final step = TransferStep.fromJson(json['step'] as String);
    final from = AccountInfo.fromJson(json['from'] as String);
    final to = AccountInfo.fromJson(json['to'] as String);

    final fromBankStr = json['from_bank'] as String?;
    final toBankStr = json['to_bank'] as String?;
    final fromBank =
        fromBankStr != null ? AccountInfo.fromJson(fromBankStr) : null;
    final toBank = toBankStr != null ? AccountInfo.fromJson(toBankStr) : null;
    return EnhancedTransferStepDoc(
      step: step,
      from: from,
      to: to,
      fromBank: fromBank,
      toBank: toBank,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'step': _model.writeToJson(),
      'from': _from.writeToJson(),
      'to': _to.writeToJson(),
      'from_bank': _fromBank?.writeToJson(),
      'to_bank': _toBank?.writeToJson(),
    };
  }

  final AccountInfo _from;
  final AccountInfo _to;
  AccountInfo? _fromBank;
  AccountInfo? _toBank;

  String get fromAccountId => hex.encode(_model.fromAccountId);
  String get toAccountId => hex.encode(_model.toAccountId);
  int get amount => _model.amount.toInt();
  String get instrument => _from.code;
  int get decimalPlaces => _from.decimalPlaces;
  List<Any> get metadata => _model.metadata;

  String get senderName => _from.publicName;
  String get senderBankName => _fromBank?.publicName ?? "";
  String get senderProfileImageUrl => _from.profileImageUrl;
  String get receiverName => _to.publicName;
  String get receiverBankName => _toBank?.publicName ?? "";
  String get receiverProfileImageUrl => _to.profileImageUrl;

  @override
  List<Object?> get props => [
        _model.writeToJson(),
        _from,
        _to,
        _fromBank,
        _toBank,
      ];
}

class ActionDoc extends _Document<Action> {
  ActionDoc(Action model) : super(model);

  int get id => _model.txId.toInt();
  String get name => _model.name;
  List<int> get payload => _model.payload;
  String get fromAccountId => hex.encode(_model.fromAccount);
  String? get targetAccountId =>
      _model.target.hasAccountId() ? hex.encode(_model.target.accountId) : null;
  String? get contextId =>
      _model.contextId.isEmpty ? null : hex.encode(_model.contextId);
}

enum RequestStatus { pending, completed, canceled, declined, inProgress }

class InvokeActionDoc extends _Document<InvokeAction> {
  InvokeActionDoc(InvokeAction model) : super(model);

  String get name => _model.name;
  List<int> get payload => _model.payload;
  String get fromAccountId => hex.encode(_model.fromAccount);
  String? get targetAccountId =>
      _model.target.hasAccountId() ? hex.encode(_model.target.accountId) : null;

  @override
  String toString() => 'Action $name $fromAccountId -> $targetAccountId';
}

class PaymentRequestDoc {
  PaymentRequestDoc._(RequestDoc requestDoc, RequestStatus _status)
      : _requestDoc = requestDoc,
        status = _status;

  RequestDoc _requestDoc;
  RequestStatus status;
  CreateTransfer get request => _requestDoc.request!.transfer;
  String get fromAccountId => _requestDoc.fromAccountId;
  String get toAccountId => _requestDoc.targetAccountId;
  String? get contextId => _requestDoc.contextId;
  DateTime get timestamp => _requestDoc.timestamp;

  // Note: This needs to be augmented manually
  AccountInfo? fromAccount;
  AccountInfo? toAccount;

  static PaymentRequestDoc? from(List<TransactionDoc> transactions) {
    // Extract requests
    final requestTxs = transactions
        .where((t) => t.error == null)
        .map((t) => RequestDoc.from(t))
        .whereNotNull()
        .toList();

    // Sort by txId
    requestTxs.sort((left, right) => left.txId.compareTo(right.txId));

    // Extract initial request
    final RequestDoc? invokedRequest =
        requestTxs.firstWhereOrNull((tx) => tx.request != null);

    if (invokedRequest == null) {
      return null;
    }

    // Extract potential failure status
    final lastNonPendingTxStatus = requestTxs
        .lastWhereOrNull((tx) => tx.status != RequestStatus.pending)
        ?.status;

    // Check if there's a matching transfer
    final transfers = transactions
        .where((t) => t.error == null && t.txId > invokedRequest.txId)
        .whereNotNull();

    final int requestValue = invokedRequest.amount;

    // Check if single or multi-phase transfer is complete
    final isComplete = transfers
        .map((t) => t.commitTransfer ?? t.transfer)
        .whereNotNull()
        .any((transfer) => transfer.steps
            .where((step) => !step.metadata.isFee)
            .any((step) => step.amount == requestValue));

    late final RequestStatus requestStatus;
    if (isComplete) {
      requestStatus = RequestStatus.completed;
    } else {
      // Check if initiateTransfer exists without completed commitTransfer
      requestStatus = transfers.any((t) => t.initiateTransfer != null)
          ? RequestStatus.inProgress
          : RequestStatus.pending;
    }

    return PaymentRequestDoc._(
      invokedRequest,
      lastNonPendingTxStatus ?? requestStatus,
    );
  }

  Future<void> populateAccountInfo({
    required M10Sdk sdk,
    required String operator,
  }) async {
    fromAccount = await sdk.getAccountInfo(
      id: fromAccountId,
      operator: operator,
    );
    toAccount = await sdk.getAccountInfo(
      id: toAccountId,
      operator: operator,
    );
  }
}

class RequestDoc {
  RequestDoc(TransactionDoc model, PaymentRequest? request)
      : _model = model,
        request = request;

  TransactionDoc _model;
  PaymentRequest? request;

  static RequestDoc? from(TransactionDoc tx) {
    if (!(tx.action?.name == 'm10.Request')) {
      return null;
    }
    try {
      final payment = PaymentRequest.fromBuffer(tx.action!.payload);
      return RequestDoc(tx, payment);
    } catch (e) {
      return null;
    }
  }

  DateTime get timestamp => _model.timestamp;
  int get txId => _model.txId;
  String? get contextId => (_model.contextId?.isEmpty ?? true)
      ? null
      : hex.encode(_model.contextId!);
  String get fromAccountId => _model.action!.fromAccountId;
  String get targetAccountId => _model.action!.targetAccountId!;
  int get amount =>
      (request?.transfer.transferSteps
          .firstWhereOrNull((step) => !step.metadata.isFee)
          ?.amount
          .toInt()) ??
      0;

  RequestStatus get status {
    if (request == null) {
      return RequestStatus.pending;
    }
    switch (request!.status) {
      case PaymentRequest_PaymentRequestStatus.CANCELED:
        return RequestStatus.canceled;
      case PaymentRequest_PaymentRequestStatus.DECLINED:
        return RequestStatus.declined;
      case PaymentRequest_PaymentRequestStatus.IN_PROGRESS:
        return RequestStatus.inProgress;
      case PaymentRequest_PaymentRequestStatus.PENDING:
        return RequestStatus.pending;
      default:
        return RequestStatus.pending;
    }
  }
}

class TransactionResponseDoc {
  TransactionResponseDoc(TransactionResponse response, String? contextId)
      : _response = response,
        _contextId = contextId;

  final TransactionResponse _response;
  final String? _contextId;

  int get txId => this._response.txId.toInt();
  String get accountCreated => hex.encode(this._response.accountCreated);
  bool get hasError => this._response.hasError();
  String? get contextId => this._contextId;
  TransactionError_Code get errorCode => this._response.error.code;
}

class TransactionDoc extends _Document<FinalizedTransaction> {
  TransactionDoc(FinalizedTransaction model) : super(model);

  int get txId => _model.response.txId.toInt();
  DateTime get timestamp =>
      DateTime.fromMicrosecondsSinceEpoch(_model.response.timestamp.toInt(),
          isUtc: true);
  int get nonce => _model.request.nonce.toInt();
  List<int>? get contextId =>
      _model.request.contextId.isEmpty ? null : _model.request.contextId;
  String? get error => _model.response.hasError()
      ? "${_model.response.error.message} (code ${_model.response.error.code})"
      : null;
  TransferRequestDoc? get transfer => _model.request.data.hasTransfer()
      ? TransferRequestDoc(_model.request.data.transfer)
      : null;
  TransferRequestDoc? get initiateTransfer =>
      _model.request.data.hasInitiateTransfer()
          ? TransferRequestDoc(_model.request.data.initiateTransfer)
          : null;

  TransferRequestDoc? get commitTransfer =>
      _model.request.data.hasCommitTransfer()
          ? TransferRequestDoc(_model.response.transferCommitted)
          : null;
  InvokeActionDoc? get action => _model.request.data.hasInvokeAction()
      ? InvokeActionDoc(_model.request.data.invokeAction)
      : null;
  String? get accountCreated => _model.request.data.hasCreateLedgerAccount()
      ? hex.encode(_model.response.accountCreated)
      : null;
  AccountFrozenDoc? get accountFrozen => _model.request.data.hasSetFreezeState()
      ? AccountFrozenDoc(_model.request.data.setFreezeState)
      : null;
  document.DocumentOperations? get documentOperations =>
      _model.request.data.hasDocumentOperations()
          ? _model.request.data.documentOperations
          : null;
  RequestDoc? asRequestDoc() => RequestDoc.from(this);

  @override
  String toString() =>
      '${transfer ?? action ?? (accountCreated != null ? 'Account $accountCreated Created' : null) ?? accountFrozen ?? documentOperations ?? 'Unknown transaction'} [error=$error]';
}

class AccountFrozenDoc extends _Document<SetFreezeState> {
  AccountFrozenDoc(SetFreezeState model) : super(model);

  String get accountId => hex.encode(_model.accountId);
  bool get isFrozen => _model.frozen;

  @override
  String toString() => 'Account $accountId} frozen=$isFrozen';
}

class AliasDoc {
  AliasDoc(Alias alias) : _alias = alias;

  final Alias _alias;
  Alias get model => _alias;
  String get handle => _alias.handle;
  String get displayName =>
      _alias.displayName.isNotEmpty ? _alias.displayName : handle;
  String get accountSetId {
    // Hack to deal with empty ids.
    // Can be removed once Directory has a search by user
    // function
    try {
      return Uuid.unparse(_alias.accountSetId);
    } catch (e) {
      return "";
    }
  }

  Alias_Type get aliasType => _alias.aliasType;
  String get operator => _alias.operator;
}

extension ContractExt on Contract {
  Uint8List get _contractId => LocalDigest().hash(this.transactions);
  String get contractId => hex.encode(_contractId);
  CreateLedgerTransfers get requests =>
      CreateLedgerTransfers.fromBuffer(this.transactions);

  // Checks if the contract has at least one signature for every ledger involved
  bool get isFullyEndorsed {
    return requests.transfers.any((transfer) => !this
        .endorsements
        .any((signature) => signature.ledgerId == transfer.ledgerId));
  }

  // Extracts a list of the proposed transfers
  List<TransferInfo> get transferInfo {
    return requests.transfers
        .map((transferRequest) =>
            transferRequest.transfer.transferSteps.map((step) => TransferInfo(
                  ledgerId: transferRequest.ledgerId,
                  nonce: transferRequest.nonce.toInt(),
                  fromAccountId: hex.encode(step.fromAccountId),
                  toAccountId: hex.encode(step.toAccountId),
                  amount: step.amount.toInt(),
                  metadata: step.metadata,
                )))
        .expand((e) => e)
        .toList();
  }

  // Gets the validUntil date time
  DateTime get validUntil {
    return DateTime.fromMicrosecondsSinceEpoch(
      requests.validUntil.toInt(),
    );
  }

  // Adds a signature to the contract, thereby approving the contents
  Future<void> endorse({
    required String ledgerId,
    required security.Signing signer,
  }) async {
    final signature = await signer.signPayload(this.transactions);
    final endorsement = Endorsement(ledgerId: ledgerId, signature: signature);
    this.endorsements.add(endorsement);
  }
}

class TransferResultDoc extends _Document<FinalizedTransaction> {
  TransferResultDoc(FinalizedTransaction model) : super(model);

  factory TransferResultDoc.fromModel(FinalizedTransaction model) {
    return TransferResultDoc(model);
  }
  int get txId => _model.response.txId.toInt();
  List<TransferStepDoc> get steps => _model.request.data.transfer.transferSteps
      .map((step) => TransferStepDoc(step))
      .toList();
}

class TransferInfo {
  const TransferInfo({
    required this.ledgerId,
    required this.fromAccountId,
    required this.toAccountId,
    required this.amount,
    required this.metadata,
    required this.nonce,
  });

  final String ledgerId;
  final String fromAccountId;
  final String toAccountId;
  final int amount;
  final List<Any> metadata;
  final int nonce;
}

class ResourceResultDoc extends _Document<FinalizedTransaction> {
  ResourceResultDoc(FinalizedTransaction model) : super(model);

  factory ResourceResultDoc.fromModel(FinalizedTransaction model) {
    return ResourceResultDoc(model);
  }

  int get txId => _model.response.txId.toInt();
  List<document.Operation> get operations =>
      _model.request.data.documentOperations.operations;
  bool get failed => _model.response.hasError();
  List<int>? get contextId =>
      _model.request.contextId.isEmpty ? null : _model.request.contextId;
  String? get error => failed
      ? "${_model.response.error.message} (code ${_model.response.error.code})"
      : null;
}

class AccountRefDoc extends _Document<AccountRef> {
  AccountRefDoc(AccountRef model) : super(model);

  factory AccountRefDoc.fromString(String accountId) {
    final accountRef = accountId.parse();
    return AccountRefDoc(accountRef);
  }

  factory AccountRefDoc.fromIds(String ledgerId, Object accountId) {
    var intAccountId = <int>[];
    if (accountId is String) {
      intAccountId = hex.decode(accountId);
    } else if (!(accountId is List<int>)) {
      throw ArgumentError('Invalid type for account ID');
    } else {
      intAccountId = accountId;
    }

    final accountRef = AccountRef()
      ..ledgerId = ledgerId
      ..accountId = intAccountId;
    return AccountRefDoc(accountRef);
  }

  String get ledgerId => _model.ledgerId;
  String get accountId => hex.encode(_model.accountId);

  AccountRef intoInner() {
    return _model;
  }

  bool operator ==(o) =>
      o is AccountRefDoc && o.ledgerId == ledgerId && o.accountId == accountId;
  int get hashCode => ledgerId.hashCode ^ accountId.hashCode;

  @override
  String toString() => '${_model.ledgerId}/${hex.encode(_model.accountId)}';
}

class TransactionMetricsDoc extends _Document<TransactionMetrics> {
  TransactionMetricsDoc(TransactionMetrics model) : super(model);

  factory TransactionMetricsDoc.fromModel(TransactionMetrics model) {
    return TransactionMetricsDoc(model);
  }

  int get transferVolume => model.transferVolume.toInt();
  int get transferCount => model.transferCount.toInt();
  int get transferErrors => model.transferErrors.toInt();
  int get accountsCreated => model.accountsCreated.toInt();
}

extension ParseAccountRef on String {
  AccountRef parse() {
    final split = this.split('/');
    final accountRef = AccountRef();
    accountRef.ledgerId = split.first;
    accountRef.accountId = hex.decode(split.last);
    return accountRef;
  }
}

extension LedgerExt on Ledger {
  String get id => '$operator';
  String get host => Uri.parse(url).host;

  Map<String, dynamic> asMap() => {
        'operator': operator,
        'url': Uri.parse(url),
      };
}

extension AccountInfoExt on AccountInfo {
  String get encodedAccountId => hex.encode(accountId);
  String get encodedParentAccountId => hex.encode(parentAccountId);
}
