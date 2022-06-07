import * as grpc from "@grpc/grpc-js";
import type { RPCImpl } from "protobufjs";

import type { m10 } from "../../protobufs";
import type { TokenProvider } from "../helpers/auth";

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

export function getRPCImplWithTokenProvider(
    ledgerUrl: string,
    credentials: grpc.ChannelCredentials,
    tokenProvider: TokenProvider,
    serviceName: string,
): RPCImpl {
    const Client = grpc.makeGenericClientConstructor({}, serviceName);
    const client = new Client(ledgerUrl, credentials);

    return (method, requestData, callback) => {
        const options: grpc.CallOptions = {
            credentials: grpc.CallCredentials.createFromGoogleCredential({
                getRequestHeaders: async (): Promise<{ [index: string]: string }> => {
                    const accessToken = await tokenProvider.getAccessToken();
                    return { Authorization: `Bearer ${accessToken}` };
                },
            }),
        };

        client.makeUnaryRequest(`/${serviceName}/${method.name}`, arg => Buffer.from(arg), arg => arg, requestData, options, callback);
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

// -------------------------------------------------------------------------
// Exports
// -------------------------------------------------------------------------

export * from "./signer";
export * from "./metadata";
export * from "./account_id";
export * from "./document_id";
export * from "./public_key";
export * from "./common";
