import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

import type { SearchAliasesResponse } from "./protobufs/directory/api";
import { Alias, Alias_Type, CheckAliasRequest, CreateImageUrlRequest, SearchAliasesRequest } from "./protobufs/directory/api";
import { DirectoryServiceClient } from "./protobufs/directory/api.client";

const DEFAULT_PAGE_SIZE = 100;

interface CreateObjectResponse {
    presignedUrl: string;
    url: string;
    objectId?: string;
}

export class DirectoryClient {
    private client: DirectoryServiceClient;

    public constructor(address: string, accessToken: string) {
        this.client = new DirectoryServiceClient(
            new GrpcWebFetchTransport({
                format: "binary",
                baseUrl: address,
                interceptors: [
                    {
                        interceptUnary(next, method, input, options) {
                            if (!options.meta) {
                                options.meta = {};
                            }

                            options.meta["Authorization"] = `Bearer ${accessToken}`;
                            return next(method, input, options);
                        },
                    },
                ],
            }),
        );
    }

    public async createImageUrl(mimeType: string): Promise<CreateObjectResponse> {
        const request = CreateImageUrlRequest.create({
            mimeType,
        });

        const { response } = await this.client.createImageUrl(request, {

        });

        return {
            presignedUrl: response.presignedUrl,
            objectId: response.objectId,
            url: response.url,
        };
    }

    public async createAlias(accoundSetIdBytes: Uint8Array, name: string, operator: string, aliasType = Alias_Type.HANDLE): Promise<void> {
        const request = Alias.create({
            handle: name,
            displayName: name,
            operator,
            aliasType,
            accountSetId: accoundSetIdBytes,
        });

        await this.client.createAlias(request);
    }

    public async getAlias(aliasId: string, pageSize = DEFAULT_PAGE_SIZE): Promise<SearchAliasesResponse> {
        const request = SearchAliasesRequest.create({
            handlePrefix: aliasId,
            pageSize: pageSize,
            // subject: aliasId,
        });

        const { response } = await this.client.searchAliases(request);

        return response;
    }

    public async checkAlias(handle: string): Promise<void> {
        const request = CheckAliasRequest.create({
            handle: handle,
        });

        await this.client.checkAlias(request);
    }
}
