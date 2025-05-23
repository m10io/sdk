import { parse, stringify, v4 } from "uuid";

export class ResourceId {
    private value: Uint8Array;

    private constructor(value: Uint8Array) {
        this.value = value;
    }

    public static generate(): ResourceId {
        return new ResourceId(parse(v4()));
    }

    public static fromBytes(value: Uint8Array): ResourceId {
        return new ResourceId(value);
    }

    public static fromString(value: string): ResourceId {
        return new ResourceId(parse(value));
    }

    public get bytes(): Uint8Array {
        return this.value;
    }

    public get string(): string {
        return stringify(this.value);
    }
}
