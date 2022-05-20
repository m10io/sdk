import * as grpc from "@grpc/grpc-js";
import type { RPCImpl } from "protobufjs";

import type { m10 } from "../../protobufs";

import { isSome } from "./common";


export function getRPCImpl(
    ledgerUrl: string,
    credentials: grpc.ChannelCredentials,
    serviceName: string,
): RPCImpl {
    const Client = grpc.makeGenericClientConstructor({}, serviceName);
    const client = new Client(ledgerUrl, credentials);

    return (method, requestData, callback) => {
        client.makeUnaryRequest(`/${serviceName}/${method.name}`, arg => Buffer.from(arg), arg => arg, requestData, callback);
    };
}

export function getRPCImplStream(
    ledgerUrl: string,
    credentials: grpc.ChannelCredentials,
    serviceName: string,
): RPCImpl {
    const Client = grpc.makeGenericClientConstructor({}, serviceName);
    const client = new Client(ledgerUrl, credentials);

    return (method, requestData) => {
        client.makeServerStreamRequest(`/${serviceName}/${method.name}`, arg => Buffer.from(arg), arg => arg, requestData);
    };
}

/**
 * Throws an exception if the value is not valid
 */
export function validate<T>(value: T, name: string, verifier: (value: T) => boolean): T {
    if (verifier(value)) {
        return value;
    }
    else {
        throw new Error(`Invalid ${name}`);
    }
}

/**
 * Throws an exception if the transaction contains an error
 */
export function checkTransactionResponse(response: m10.sdk.transaction.ITransactionResponse): void {
    if (isSome(response.error)) {
        throw new Error(`TransactionError: ${response.error.message}`);
    }
}

export function getPrivateKey(privateKey: string): string {
    const KEY_HEADER = "-----BEGIN PRIVATE KEY-----\n";
    const KEY_FOOTER = "-----END PRIVATE KEY-----\n";
    return `${KEY_HEADER}${privateKey}\n${KEY_FOOTER}`;
}

// -------------------------------------------------------------------------
// Exports
// -------------------------------------------------------------------------

export * from "./signer";
export * from "./metadata";
export * from "./account_id";
export * from "./document_id";
export * from "./public_key";
export * from "./common";
