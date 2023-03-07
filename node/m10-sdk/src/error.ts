import { m10 } from "../protobufs";

import { unwrapOr } from "./utils/common";

export enum M10ErrorType {
    Signing = "SIGNING",                        // FROM: SigningError
    Status = "STATUS",                          // FROM: Status
    Transaction = "TRANSACTION",                // FROM: TransactionError
    InvalidAccountId = "INVALID_ACCOUNT_ID",    // FROM: AccountIdError
    InvalidTransaction = "INVALID_TRANSACTION",
    Other = "OTHER",
}

export class M10Error extends Error {

    private _type: M10ErrorType;

    public constructor(message: string, type: M10ErrorType) {
        super(M10Error.format(message, type));
        this._type = type;
    }

    public getDisplay(): string {
        return M10Error.format(this.message, this._type);
    }

    private static format(message: string, type: M10ErrorType): string {
        return `[${type}] ${message}`;
    }

    public static Signing(message: string): M10Error {
        return new M10Error(message, M10ErrorType.Signing);
    }

    public static Status(message: string): M10Error {
        return new M10Error(message, M10ErrorType.Status);
    }

    public static Transaction(error: m10.sdk.transaction.ITransactionError): M10Error {
        const errorCode = unwrapOr(error.code, m10.sdk.transaction.TransactionError.Code.UNKNOWN);
        const errorMessage = error.message || "EMPTY_ERROR_MESSAGE";

        return new M10Error(
            `[${m10.sdk.transaction.TransactionError.Code[errorCode]}] ${errorMessage}`,
            M10ErrorType.Transaction,
        );
    }

    public static InvalidAccountId(message?: string): M10Error {
        return new M10Error(message || "Failed AccountId validation", M10ErrorType.InvalidAccountId);
    }

    public static InvalidTransaction(message?: string): M10Error {
        return new M10Error(message || "Invalid transaction", M10ErrorType.InvalidTransaction);
    }

    public static Other(message: string): M10Error {
        return new M10Error(message, M10ErrorType.Other);
    }
}
