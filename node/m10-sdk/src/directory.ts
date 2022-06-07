import { credentials } from "@grpc/grpc-js";

import { m10 } from "../protobufs";

import type { TokenProvider } from "./helpers/auth";
import { getRPCImplWithTokenProvider } from "./utils";

interface CreateObjectResponse {
  presignedUrl: string;
  url: string;
  objectId?: string;
}

export class DirectoryClient {
    private client: m10.directory.DirectoryService;

    public constructor(address: string, tokenProvider: TokenProvider, secure: boolean) {
        const creds = secure
            ? credentials.createSsl()
            : credentials.createInsecure();
        this.client = m10.directory.DirectoryService.create(
            getRPCImplWithTokenProvider(address, creds, tokenProvider, "m10.directory.DirectoryService"),
        );
    }

    public async createImageUrl(mimeType: string): Promise<CreateObjectResponse> {
        const request = new m10.directory.CreateImageUrlRequest({ mimeType: mimeType });
        const response = await this.client.createImageUrl(request);
        return {
            presignedUrl: response.presignedUrl,
            objectId: response.objectId,
            url: response.url,
        };
    }
}
