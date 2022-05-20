/// <reference types="node" />
export declare class ImageClient {
    address: string;
    constructor(address: string);
    put_image(name: string, image: Blob): Promise<Result<string>>;
    get_image(name: string): Promise<Result<Blob>>;
}
