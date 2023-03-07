import type { google } from "../../protobufs";
import { m10 } from "../../protobufs";
import { M10Error } from "../error";
import type { TxId } from "../ids";
import { AccountId } from "../ids";
import type { EnhancedTransfer, EnhancedTransferStep } from "../transfer_ext";
import { isNone, unwrap } from "../utils";


export interface ITransferStep {
    from: AccountId;
    to: AccountId;
    amount: LongNumber;
    metadata: google.protobuf.IAny[];
}

export class TransferStep implements ITransferStep{
    public from: AccountId;
    public to: AccountId;
    public amount: LongNumber;
    public metadata: google.protobuf.IAny[];

    public constructor(properties: ITransferStep) {
        this.from = properties.from;
        this.to = properties.to;
        this.amount = properties.amount;
        this.metadata = properties.metadata;
    }

    // TODO: Implement `with_type` function when metadate is a bit more defined
    // impl MetadataExt for TransferStep {
    //     fn with_type<M: MetadataType>(&self) -> Option<&[u8]> {
    //         self.metadata
    //             .iter()
    //             .find(|a| a.type_url == M::TYPE_URL)
    //             .map(|a| a.value.as_slice())
    //     }
    // }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public static tryFrom(transferStep: m10.sdk.transaction.ITransferStep): TransferStep {
        return new TransferStep({
            from: AccountId.fromUint8Array(unwrap(transferStep.fromAccountId, M10Error.InvalidTransaction())),
            to: AccountId.fromUint8Array(unwrap(transferStep.toAccountId, M10Error.InvalidTransaction())),
            amount: unwrap(transferStep.amount, M10Error.InvalidTransaction()),
            metadata: unwrap(transferStep.metadata, M10Error.InvalidTransaction()),
        });
    }
}

export enum TransferStatus {
    Pending,
    Accepted,
    Rejected,
    Expired,
}

export interface ITransfer {
    txId: TxId;
    contextId: Uint8Array;
    timestamp: LongNumber;
    steps: TransferStep[];
    success: boolean;
    status: TransferStatus;
}

export class Transfer implements ITransfer {
    public txId: TxId;
    public contextId: Uint8Array;
    public timestamp: LongNumber;
    public steps: TransferStep[];
    public success: boolean;
    public status: TransferStatus;

    public constructor(properties: ITransfer) {
        this.txId = properties.txId;
        this.contextId = properties.contextId;
        this.timestamp = properties.timestamp;
        this.steps = properties.steps;
        this.success = properties.success;
        this.status = properties.status;
    }

    // TODO: Implement `with_type` function when metadate is a bit more defined
    // impl MetadataExt for Transfer {
    //     fn with_type<M: MetadataType>(&self) -> Option<&[u8]> {
    //         self.steps.iter().find_map(TransferStep::with_type::<M>)
    //     }
    // }

    public static getTransferStatus(state: m10.sdk.transaction.FinalizedTransfer.TransferState): TransferStatus {
        switch (state) {
            case m10.sdk.transaction.FinalizedTransfer.TransferState.PENDING:
                return TransferStatus.Pending;
            case m10.sdk.transaction.FinalizedTransfer.TransferState.ACCEPTED:
                return TransferStatus.Accepted;
            case m10.sdk.transaction.FinalizedTransfer.TransferState.REJECTED:
                return TransferStatus.Rejected;
            case m10.sdk.transaction.FinalizedTransfer.TransferState.EXPIRED:
                return TransferStatus.Expired;
        }
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFrom(transfer: m10.sdk.transaction.IFinalizedTransfer): Transfer {
        return new Transfer({
            txId: unwrap(transfer.txId, M10Error.InvalidTransaction()),
            contextId: unwrap(transfer.contextId, M10Error.InvalidTransaction()),
            timestamp: unwrap(transfer.timestamp, M10Error.InvalidTransaction()),
            steps: unwrap(transfer.transferSteps, M10Error.InvalidTransaction()).map(TransferStep.tryFrom),
            success: isNone(transfer.error),
            status: Transfer.getTransferStatus(unwrap(transfer.state, M10Error.InvalidTransaction())),
        });
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFromTxn(txn: m10.sdk.IFinalizedTransaction): Transfer {
        const txnData = new m10.sdk.transaction.TransactionData(unwrap(txn.request?.data, M10Error.InvalidTransaction()));

        const contextId = unwrap(txn.request?.contextId, M10Error.InvalidTransaction());
        const success = isNone(txn.response?.error);
        const txId = unwrap(txn.response?.txId, M10Error.InvalidTransaction());
        const timestamp = unwrap(txn.response?.timestamp, M10Error.InvalidTransaction());

        switch (unwrap(txnData.data, M10Error.InvalidTransaction())) {
            case "transfer":
                return new Transfer({
                    txId: txId,
                    contextId: contextId,
                    timestamp: timestamp,
                    steps: unwrap(txnData.transfer?.transferSteps, M10Error.InvalidTransaction())
                        .map(TransferStep.tryFrom),
                    success: success,
                    status: TransferStatus.Accepted,
                });
            case "initiateTransfer":
                return new Transfer({
                    txId: txId,
                    contextId: contextId,
                    timestamp: timestamp,
                    steps: unwrap(txnData.initiateTransfer?.transferSteps, M10Error.InvalidTransaction())
                        .map(TransferStep.tryFrom),
                    success: success,
                    status: TransferStatus.Pending,
                });
            case "commitTransfer":
                return new Transfer({
                    txId: txId,
                    contextId: contextId,
                    timestamp: timestamp,
                    steps: unwrap(txn.response?.transferCommitted?.transferSteps, M10Error.InvalidTransaction())
                        .map(TransferStep.tryFrom),
                    success: success,
                    status: (
                        success
                            ? Transfer.getTransferStatus(
                                unwrap(txn.request?.data?.commitTransfer?.newState, M10Error.InvalidTransaction()),
                            )
                            : TransferStatus.Pending
                    ),
                });
            default:
                throw M10Error.InvalidTransaction();
        }
    }
}


export interface ExpandedAccount {
    id: AccountId;
    publicName: string;
    profileImageUrl: string;
    code: string;
    decimals: number;
}


export interface IExpandedTransferStep {
    from: ExpandedAccount;
    to: ExpandedAccount;
    amount: LongNumber;
    metadata: google.protobuf.IAny[];
}

export class ExpandedTransferStep implements IExpandedTransferStep {
    public from: ExpandedAccount;
    public to: ExpandedAccount;
    public amount: LongNumber;
    public metadata: google.protobuf.IAny[];


    public constructor(properties: IExpandedTransferStep) {
        this.from = properties.from;
        this.to = properties.to;
        this.amount = properties.amount;
        this.metadata = properties.metadata;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public static tryFrom(step: EnhancedTransferStep, raw: m10.sdk.transaction.ITransferStep): ExpandedTransferStep {
        const from: ExpandedAccount = {
            id: AccountId.fromUint8Array(unwrap(step.from?.accountId, M10Error.InvalidTransaction())),
            publicName: unwrap(step.from?.publicName, M10Error.InvalidTransaction()),
            profileImageUrl: unwrap(step.from?.profileImageUrl, M10Error.InvalidTransaction()),
            code: unwrap(step.from?.code, M10Error.InvalidTransaction()),
            decimals: unwrap(step.from?.decimalPlaces, M10Error.InvalidTransaction()),
        };

        const to: ExpandedAccount = {
            id: AccountId.fromUint8Array(unwrap(step.to?.accountId, M10Error.InvalidTransaction())),
            publicName: unwrap(step.to?.publicName, M10Error.InvalidTransaction()),
            profileImageUrl: unwrap(step.to?.profileImageUrl, M10Error.InvalidTransaction()),
            code: unwrap(step.to?.code, M10Error.InvalidTransaction()),
            decimals: unwrap(step.to?.decimalPlaces, M10Error.InvalidTransaction()),
        };

        return new ExpandedTransferStep({
            from: from,
            to: to,
            amount: unwrap(raw.amount, M10Error.InvalidTransaction()),
            metadata: unwrap(raw.metadata, M10Error.InvalidTransaction()),
        });
    }
}


export interface IExpandedTransfer {
    txId: TxId;
    contextId: Uint8Array;
    timestamp: LongNumber;
    steps: ExpandedTransferStep[];
    success: boolean;
    status: TransferStatus;
}

export class ExpandedTransfer implements IExpandedTransfer {
    public txId: TxId;
    public contextId: Uint8Array;
    public timestamp: LongNumber;
    public steps: ExpandedTransferStep[];
    public success: boolean;
    public status: TransferStatus;


    public constructor(properties: IExpandedTransfer) {
        this.txId = properties.txId;
        this.contextId = properties.contextId;
        this.timestamp = properties.timestamp;
        this.steps = properties.steps;
        this.success = properties.success;
        this.status = properties.status;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public static tryFrom(transfer: EnhancedTransfer): ExpandedTransfer {
        return new ExpandedTransfer({
            txId: transfer.transfer.txId,
            contextId: transfer.transfer.contextId,
            timestamp: transfer.transfer.timestamp,
            steps: transfer.enhanced_steps.map((step, index) => ExpandedTransferStep.tryFrom(step, transfer.transfer.transferSteps[index])),
            success: isNone(transfer.transfer.error),
            status: Transfer.getTransferStatus(transfer.transfer.state),
        });
    }
}
