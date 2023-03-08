import type { TxId } from "../ids";


export interface ITxnFilter {
    min: TxId;
    max: TxId;
    limit: number;
}

export class TxnFilter {

    private _min: Option<TxId>;
    private _max: Option<TxId>;
    private _limit: Option<number>;

    public static readonly DEFAULT_TXN_LIMIT: number = 20;


    public constructor(properties: ITxnFilter) {
        this._min = properties.min;
        this._max = properties.max;
        this._limit = properties.limit;
    }


    public getMinTxId(): Option<TxId> {
        return this._min;
    }
    public getMaxTxId(): Option<TxId> {
        return this._max;
    }
    public getLimit(): Option<number> {
        return this._limit;
    }

    public min(txId: TxId): TxnFilter {
        this._min = txId;
        return this;
    }

    public max(txId: TxId): TxnFilter {
        this._max = txId;
        return this;
    }

    public limit(count: number): TxnFilter {
        this._limit = count;
        return this;
    }
}
