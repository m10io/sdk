export declare function arrayIsNotEmpty(array: unknown[]): boolean;
export declare function unwrap<T>(value: Option<T>, name: string): T;
export declare function unwrapOr<T>(value: Option<T>, defaultValue: T): T;
export declare function isSome<T>(value: Option<T>): value is T;
