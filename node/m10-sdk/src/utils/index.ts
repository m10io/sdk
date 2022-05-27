import * as grpc from "@grpc/grpc-js";
import type { RPCImpl } from "protobufjs";

import type { m10 } from "../../protobufs";

import { isSome } from "./common";

const BEGIN_PRIV_KEY_PREFIX = 8;
const END_PRIV_KEY_PREFIX = 56;

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
    let stream: grpc.ClientReadableStream<Buffer>;

    return (method, requestData, callback) => {
        if (!isSome(method) || !isSome(requestData)) {
            client.close();
            stream?.cancel();
            return;
        }

        stream = client.makeServerStreamRequest(`/${serviceName}/${method.name}`, arg => Buffer.from(arg), arg => arg, requestData);
        stream.on("data", (chunk) => callback(null, chunk));
        stream.on("error", (error) => callback(error, null));
        stream.on("close", () => callback(null, null));
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

/**
 * Converts a PKCS#8 v2 key to the v1 format with only the PRV and version.
 */
export function convertPkcs8V2KeyToV1(pkcs8v2Key: string): string {
    const key = "MC4CAQAw" + pkcs8v2Key.substr(BEGIN_PRIV_KEY_PREFIX, END_PRIV_KEY_PREFIX);

    return key;
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
