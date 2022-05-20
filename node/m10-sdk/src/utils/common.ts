
export function arrayIsNotEmpty(array: unknown[]): boolean {
    return (Array.isArray(array) && array.length > 0);
}

export function unwrap<T>(value: Option<T>, name: string): T {
    if (!isSome(value)) {
        throw new Error(`Error: Unexpectedly found None while unwrapping an Option value [${name}]`);
    }

    return value;
}

export function unwrapOr<T>(value: Option<T>, defaultValue: T): T {
    return isSome(value) ? value : defaultValue;
}


export function isSome<T>(value: Option<T>): value is T {
    return (value !== null) && (value !== undefined);
}
