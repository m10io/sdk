import { m10 } from "../../protobufs";
import type { LedgerClient } from "../client";
import * as utils from "../utils";

export async function createTransfer(
    client: LedgerClient,
    fromAccount: utils.AccountId,
    toAccount: utils.AccountId,
    amount: number | Long,
    reference: string,
    signer: utils.CryptoSigner,
): Promise<m10.sdk.transaction.ITransactionResponse> {
    const transferPayload = new m10.sdk.transaction.CreateTransfer({
        transferSteps: [ {
            fromAccountId: utils.getUint8ArrayFromAccountId(fromAccount),
            toAccountId: utils.getUint8ArrayFromAccountId(toAccount),
            amount,
            // this is limited to 512 KiB serialized
            metadata: [ utils.convertMemoToAny({ plaintext: reference }) ],
        } ],
    });

    const request = client.transactionRequest({ transfer: transferPayload });

    const tx = await client.createTransaction(signer, request);

    if (tx.error) {
        // TODO: add a specialized error handler
        throw new Error(
            "CODE: " + utils.unwrapOr<m10.sdk.transaction.TransactionError.Code>(tx.error.code, m10.sdk.transaction.TransactionError.Code.UNKNOWN) +
            "MSG: " + utils.unwrapOr<string>(tx.error.message, ""),
        );
    }

    return tx;
}
