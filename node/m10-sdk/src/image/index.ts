import superagent from "superagent";

interface StatusObject {
    status: number;
}

export class ImageClient {

    public address: string;

    public constructor(address: string) {
        this.address = address;
    }

    public async putImage(image: Buffer): Promise<StatusObject> {
        const url = this.address;
        return await superagent.put(url).send(image);
    }

    public async getImage(url: string): Promise<Uint8Array> {
        const response = await superagent.get(url);
        return response.body;
    }
}
