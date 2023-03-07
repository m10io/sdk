
export function arrayIsNotEmpty(array: unknown[]): boolean {
    return (Array.isArray(array) && array.length > 0);
}

/**
 * @throws  {Error} that was provided if value found during unwrapping is `None`
 */
export function unwrap<T>(value: Option<T>, error: Error): T {
    if (isNone(value)) {
        throw error;
    }
    return value;
}

export function unwrapOr<T>(value: Option<T>, defaultValue: T): T {
    return isSome(value) ? value : defaultValue;
}


export function isSome<T>(value: Option<T>): value is T {
    return (value !== null) && (value !== undefined);
}

export function isNone<T>(value: Option<T>): value is None {
    return (value === null) || (value === undefined);
}
