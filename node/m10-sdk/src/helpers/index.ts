import * as uuid from "uuid";

import { m10 } from "../../protobufs";
import type { LedgerClient } from "../client";
import { checkTransactionResponse, CryptoSigner, getUint8ArrayFromDocumentId } from "../utils";
import { collections } from "..";


// -----------------------------------------------------------------------------
// User
// -----------------------------------------------------------------------------

export async function createUser(client: LedgerClient): Promise<[ CryptoSigner, string ]> {

    const signer = CryptoSigner.generateKeyPair();
    const userId = uuid.v4();

    const user: m10.sdk.model.IAccountSet = {
        owner: Uint8Array.from(signer.getPublicKey()),
        id: getUint8ArrayFromDocumentId(userId),
    };

    const userPayload = new m10.sdk.model.AccountSet(user);
    const transactionData = collections.getCreateTransactionDataFromDocument(
        m10.sdk.model.AccountSet.encode(userPayload).finish(),
        collections.Collection.AccountSet,
    );

    const transactionRequestPayload = client.transactionRequest(transactionData);
    const transactionResponse = await client.createTransaction(signer, transactionRequestPayload);

    checkTransactionResponse(transactionResponse);

    return [ signer, userId ];
}

// -------------------------------------------------------------------------
// Exports
// -------------------------------------------------------------------------

export * as roleBindings from "./role-bindings";
export * as accounts from "./accounts";
