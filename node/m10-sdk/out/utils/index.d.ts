import * as grpc from "@grpc/grpc-js";
import type { RPCImpl } from "protobufjs";
import type { m10 } from "../../protobufs";
export declare function getRPCImpl(ledgerUrl: string, credentials: grpc.ChannelCredentials, serviceName: string): RPCImpl;
export declare function getRPCImplStream(ledgerUrl: string, credentials: grpc.ChannelCredentials, serviceName: string): RPCImpl;
/**
 * Throws an exception if the value is not valid
 */
export declare function validate<T>(value: T, name: string, verifier: (value: T) => boolean): T;
/**
 * Throws an exception if the transaction contains an error
 */
export declare function checkTransactionResponse(response: m10.sdk.transaction.ITransactionResponse): void;
export declare function getPrivateKey(privateKey: string): string;
export * from "./signer";
export * from "./metadata";
export * from "./account_id";
export * from "./document_id";
export * from "./public_key";
export * from "./common";
