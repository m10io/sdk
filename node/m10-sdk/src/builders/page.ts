import { m10 } from "../../protobufs";
import type { DocumentId, PublicKey } from "../ids";


interface IFilter {
    name?: Option<string>;
    owner?: Option<PublicKey>;
}

export interface IPageBuilder {
    filter: IFilter;
    limit: number;
    lastId: Option<DocumentId>;
}

export class PageBuilder {
    private _filter: IFilter;
    private _limit: number;
    private _lastId: Option<DocumentId>;

    private static readonly DEFAULT_LIMIT: number = 20;


    public constructor(properties: IPageBuilder) {
        this._filter = properties.filter;
        this._limit = properties.limit;
        this._lastId = properties.lastId;
    }

    public filter(filter: IFilter): PageBuilder {
        this._filter = filter;
        return this;
    }

    public limit(limit: number): PageBuilder {
        this._limit = limit;
        return this;
    }

    public lastId(id: DocumentId): PageBuilder {
        this._lastId = id;
        return this;
    }

    public static byName(name: string): PageBuilder {
        return new PageBuilder({
            filter: { name },
            limit: PageBuilder.DEFAULT_LIMIT,
            lastId: null,
        });
    }

    public static byOwner(owner: PublicKey): PageBuilder {
        return new PageBuilder({
            filter: { owner },
            limit: PageBuilder.DEFAULT_LIMIT,
            lastId: null,
        });
    }

    public toPage(): m10.sdk.Page {
        return new m10.sdk.Page({
            limit: this._limit,
            lastId: this._lastId?.toUint8Array(),
        });
    }

    public toListRolesRequest(): m10.sdk.ListRolesRequest {
        return new m10.sdk.ListRolesRequest({
            page: this.toPage(),
            name: this._filter.name,
        });
    }

    public toListRoleBindingsRequest(): m10.sdk.ListRoleBindingsRequest {
        return new m10.sdk.ListRoleBindingsRequest({
            page: this.toPage(),
            name: this._filter.name,
        });
    }


    public toListAccountSetsRequest(): m10.sdk.ListAccountSetsRequest {
        return new m10.sdk.ListAccountSetsRequest({
            page: this.toPage(),
            name: this._filter.name,
            owner: this._filter.owner?.toUint8Array(),
        });
    }

    public toListAccountMetadataRequest(): m10.sdk.ListAccountMetadataRequest {
        return new m10.sdk.ListAccountMetadataRequest({
            page: this.toPage(),
            name: this._filter.name,
            owner: this._filter.owner?.toUint8Array(),
        });
    }
}
