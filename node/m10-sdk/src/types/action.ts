import { google, m10 } from "../../protobufs";
import { M10Error } from "../error";
import type { TxId } from "../ids";
import { AccountId } from "../ids/account_id";
import { isSome, unwrap } from "../utils";



export class Target {
    public accountId: Option<AccountId>;


    public constructor(accountId?: Option<AccountId>) {
        this.accountId = accountId;
    }

    public into(): m10.sdk.transaction.Target {
        return (
            isSome(this.accountId)
                ? new m10.sdk.transaction.Target({ accountId: this.accountId.toUint8Array() })
                : new m10.sdk.transaction.Target({ anyAccount: new google.protobuf.Empty() })
        );
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public static tryFrom(target: m10.sdk.transaction.ITarget): Target {
        return (
            isSome(target.anyAccount)
                ? new Target()
                : new Target(
                    AccountId.fromUint8Array(
                        unwrap(target.accountId, M10Error.InvalidTransaction()),
                    ),
                )
        );
    }
}

export interface IAction {
    txId: TxId;
    contextId: Uint8Array;
    name: string;
    target: Target;
    payload: Uint8Array;
}

export class Action implements IAction {
    public txId: TxId;
    public contextId: Uint8Array;
    public name: string;
    public target: Target;
    public payload: Uint8Array;

    public constructor(properties: IAction) {
        this.txId = properties.txId;
        this.contextId = properties.contextId;
        this.name = properties.name;
        this.target = properties.target;
        this.payload = properties.payload;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public static tryFrom(action: m10.sdk.transaction.IAction): Action {
        return new Action({
            txId: unwrap(action.txId, M10Error.InvalidTransaction()),
            contextId: unwrap(action.contextId, M10Error.InvalidTransaction()),
            name: unwrap(action.name, M10Error.InvalidTransaction()),
            target: Target.tryFrom(unwrap(action.target, M10Error.InvalidTransaction())),
            payload: unwrap(action.payload, M10Error.InvalidTransaction()),
        });
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFromFinalizedTransaction(txn: m10.sdk.IFinalizedTransaction): Action {
        return new Action({
            txId: unwrap(txn.response?.txId, M10Error.InvalidTransaction()),
            contextId: unwrap(txn.request?.contextId, M10Error.InvalidTransaction()),
            name: unwrap(txn.request?.data?.invokeAction?.name, M10Error.InvalidTransaction()),
            target: new Target(),
            payload: unwrap(txn.request?.data?.invokeAction?.payload, M10Error.InvalidTransaction()),
        });
    }
}
