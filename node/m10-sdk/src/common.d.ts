
type None = null | undefined;
type Some<T> = T;
type Option<T> = None | Some<T>;

type Dictionary<TKey extends string | number | symbol, TValue> = {
    [key in TKey]?: TValue;
};

type LongNumber = number | Long;
