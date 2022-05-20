"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.setupRoleBindings = exports.updateRoleBinding = void 0;
const uuid = __importStar(require("uuid"));
const protobufs_1 = require("../../protobufs");
const utils = __importStar(require("../utils"));
const __1 = require("..");
function updateRoleBinding(client, signer, id, subjects) {
    return __awaiter(this, void 0, void 0, function* () {
        utils.validate(id, "uuid", uuid.validate);
        const document = new protobufs_1.m10.sdk.RoleBinding({
            id: utils.getUint8ArrayFromDocumentId(id),
            subjects: subjects.map(subject => new Uint8Array(subject)),
        });
        const documentUpdatePayload = new __1.collections.DocumentUpdate(document, ["subjects"]);
        const transactionData = __1.collections.getUpdateTransactionDataFromDocument(protobufs_1.m10.sdk.RoleBinding.encode(document).finish(), document.id, documentUpdatePayload, __1.collections.Collection.RoleBinding);
        const transactionRequestPayload = client.transactionRequest(transactionData);
        const response = yield client.createTransaction(signer, transactionRequestPayload);
        utils.checkTransactionResponse(response);
    });
}
exports.updateRoleBinding = updateRoleBinding;
function setupRoleBindings(ledgerClient, bankAdminSigner, usersToAdd) {
    return __awaiter(this, void 0, void 0, function* () {
        const listRoleBindingsRequest = { name: "node-test-customer" };
        const listRoleBindingsResponse = yield ledgerClient.listRoleBindings(bankAdminSigner, listRoleBindingsRequest);
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
        const roleBindingId = utils.unwrap(roleBinding.id, "roleBinding.id");
        const roleBindingSubjects = utils.unwrap(roleBinding.subjects, "roleBinding.subjects");
        const subjects = usersToAdd.filter((publicKey) => !roleBindingSubjects.includes(publicKey));
        if (utils.arrayIsNotEmpty(subjects)) {
            yield updateRoleBinding(ledgerClient, bankAdminSigner, roleBindingId, subjects);
        }
    });
}
exports.setupRoleBindings = setupRoleBindings;
//# sourceMappingURL=role-bindings.js.map