import * as uuid from "uuid";

import { m10 } from "../../protobufs";
import type { LedgerClient } from "../client";
import * as utils from "../utils";
import { collections } from "..";


export async function updateRoleBinding(
    client: LedgerClient, signer: utils.CryptoSigner, id: string, subjects: Uint8Array[],
): Promise<void> {

    utils.validate(id, "uuid", uuid.validate);

    const document = new m10.sdk.RoleBinding({
        id: utils.getUint8ArrayFromDocumentId(id),
        subjects: subjects.map(subject => new Uint8Array(subject)),
    });

    const documentUpdatePayload = new collections.DocumentUpdate(document, [ "subjects" ]);
    const transactionData = collections.getUpdateTransactionDataFromDocument(
        m10.sdk.RoleBinding.encode(document).finish(),
        document.id,
        documentUpdatePayload,
        collections.Collection.RoleBinding,
    );

    const transactionRequestPayload = client.transactionRequest(transactionData);
    const response = await client.createTransaction(signer, transactionRequestPayload);

    utils.checkTransactionResponse(response);
}

export async function setupRoleBindings(
    ledgerClient: LedgerClient,
    bankAdminSigner: utils.CryptoSigner,
    usersToAdd: Buffer[],
): Promise<void> {

    const listRoleBindingsRequest: m10.sdk.IListRoleBindingsRequest = { name: "node-test-customer" };
    const listRoleBindingsResponse = await ledgerClient.listRoleBindings(bankAdminSigner, listRoleBindingsRequest);

    const roleBindings = (listRoleBindingsResponse.roleBindings || [])
        .map(roleBinding => ({
            id: roleBinding.id ? utils.getDocumentIdFromUint8Array(roleBinding.id) : null,
            name: roleBinding.name,
            subjects: roleBinding.subjects,
        }));

    if (!utils.arrayIsNotEmpty(roleBindings)) {
        throw new Error("roleBindings is empty");
    }

    const roleBinding = roleBindings[0];
    const roleBindingId = utils.unwrap<string>(roleBinding.id, "roleBinding.id");
    const roleBindingSubjects = utils.unwrap<Uint8Array[]>(roleBinding.subjects, "roleBinding.subjects");

    const subjects = usersToAdd.filter((publicKey) => !roleBindingSubjects.includes(publicKey));

    if (utils.arrayIsNotEmpty(subjects)) {
        await updateRoleBinding(ledgerClient, bankAdminSigner, roleBindingId, subjects);
    }
}

