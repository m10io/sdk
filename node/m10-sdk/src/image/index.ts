import superagent from "superagent";


export class ImageClient {

    public address: string;

    public constructor(address: string) {
        this.address = address;
    }

    public async put_image(name: string, image: Blob): Promise<Result<string>> {

        const url = `${this.address}/images/${name}`;

        try {
            await superagent.put(url).send(image);
            return url;
        } catch (error) {
            return error as Error;
        }
    }

    public async get_image(name: string): Promise<Result<Blob>> {

        const url = `${this.address}/images/${name}`;

        try {
            const response = await superagent.get(url)
                .responseType("blob");

            return response.body;
        } catch (error) {
            return error as Error;
        }
    }
}
