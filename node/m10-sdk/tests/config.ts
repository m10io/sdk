/* eslint-disable @typescript-eslint/no-magic-numbers */
import { Operation_InsertDocument } from "../src/protobufs/sdk/document";
import { AccountMetadata } from "../src/protobufs/sdk/model/model";
import { Role, Rule_Verb, RoleBinding } from "../src/protobufs/sdk/rbac";
import { CryptoSigner, DirectoryClient, getBaseAccessToken, M10Client } from "../src";
import { AccountId, Collection, ResourceId } from "../src";

export const testCase = {
    "contextId": "fd5cd0c2b6ca7ae8e6c1ce4c7034c0cd5830b5b03e95d3de223c69bb19ce0a7e",
    "usdCentralBankId": "3a000000000000000000000000000000",
    "eurCentralBankId": "3b000000000000000000000000000000",

    "bankUSDId": "05800000000000000000000000000002",
    "bankEURId": "06800000000000000000000000000002",

    "aliceUSDAccountId": "05800000800000000000000000000004",
    "bobEURAccountId": "06800000800000000000000000000004",

    "fxUsdAccountId": "05000000000000000000000000000001",
    "fxEurAccountId": "06000000000000000000000000000001",
};

export const LEDGER_ID = "m10";
export const LEDGER_URL = process.env.LEDGER_URL || "https://app.dev.m10.net";
export const OPERATOR_KEY = "MFMCAQEwBQYDK2VwBCIEIHyr+m5Z4gy9JxoMdgrrX/EE8uhzkj3ztWx28zJxpStqoSMDIQAAIwpWR4i34vnPf3GTlge6ONw3tsuGer5QiQsGXKY0zg==";
export const USD_BANK_ID = AccountId.fromHex(testCase.bankUSDId);
export const EUR_BANK_ID = AccountId.fromHex(testCase.bankEURId);

export const TEST_USERNAME = "ops+directory_test_admin@m10.io";
export const TEST_PASSWORD = "AMsrNB9YuYfmYBw7";

export type TestCaseInstances = {
    operatorSigner: CryptoSigner;
    accountSigner: CryptoSigner;
    operatorClient: M10Client;
    accountClient: M10Client;
    directoryClient: DirectoryClient;
};

export const initTestCaseInstances = async (): Promise<TestCaseInstances> => {
    const operatorSigner = await CryptoSigner.fromPkcs8Pem(OPERATOR_KEY);
    const accountSigner = await CryptoSigner.generateKeyPair();

    const operatorClient = new M10Client(LEDGER_URL, operatorSigner);
    const accountClient = new M10Client(LEDGER_URL, accountSigner);

    const accessToken = await getBaseAccessToken(LEDGER_URL.replace("grpc-", ""), TEST_USERNAME, TEST_PASSWORD);
    const directoryClient = new DirectoryClient(LEDGER_URL, accessToken);

    return {
        operatorSigner,
        accountSigner,
        operatorClient,
        accountClient,
        directoryClient,
    };
};

export const createCurrencyAccounts = async (
    {
        operatorClient,
        accountSigner,
        directoryClient,
    }: TestCaseInstances,
    bankIds = [USD_BANK_ID, EUR_BANK_ID],
    additionalLedgerAccRuleVerbs?: Rule_Verb[],
): Promise<{
    accountNames: string[];
    accountIds: Record<string, AccountId>;
}> => {
    const accountNames: string[] = [];
    const accountIds: Record<string, AccountId> = {};

    await Promise.all(bankIds.map(async (bankId, idx) => {
        const [, accountId] = await operatorClient.createAccount({
            parentId: bankId.bytes,
            frozen: false,
            issuance: true,
        });

        if (accountIds[bankId.hex]) {
            accountIds[`${bankId.hex}-${idx}`] = accountId;
        } else {
            accountIds[bankId.hex] = accountId;
        }

        const name = `kek_name-${accountId.hex}`;

        await operatorClient.documents([
            {
                oneofKind: "insertDocument",
                insertDocument: Operation_InsertDocument.create({
                    collection: Collection.AccountMetadata,
                    document: AccountMetadata.toBinary(
                        AccountMetadata.create({
                            name: name,
                            owner: accountSigner.getPublicKey().toUint8Array(),
                            publicName: name,
                            id: accountId.bytes,
                        }),
                    ),
                }),
            },
        ]);
        const bankCustomerRole = Role.create({
            id: ResourceId.generate().bytes,
            name: `${name}`,
            rules: [
                {
                    collection: Collection.AccountMetadata,
                    verbs: [Rule_Verb.CREATE],
                },
                {
                    collection: Collection.LedgerAccount,
                    verbs: [
                        Rule_Verb.CREATE,
                        ...(additionalLedgerAccRuleVerbs ?? []),
                    ],
                },
            ],
        });

        await operatorClient.documents([{
            oneofKind: "insertDocument",
            insertDocument: Operation_InsertDocument.create({
                collection: Collection.Role,
                document: Role.toBinary(bankCustomerRole),
            }),
        }]);
        const bankCustomerRoleBinding = RoleBinding.create({
            id: ResourceId.generate().bytes,
            name: `${name}-role-binding`,
            isUniversal: false,
            subjects: [accountSigner.getPublicKey().toUint8Array()],
            expressions: [],
            owner: accountSigner.getPublicKey().toUint8Array(),
            role: bankCustomerRole.id,
        });


        await operatorClient.documents([{
            oneofKind: "insertDocument",
            insertDocument: Operation_InsertDocument.create({
                collection: Collection.RoleBinding,
                document: RoleBinding.toBinary(bankCustomerRoleBinding),
            }),
        }]);

        // FIXME: use account set resource instead
        await directoryClient.createAlias(accountId.bytes, name, LEDGER_ID);
        accountNames.push(name);
    }));

    return {
        accountNames,
        accountIds,
    };
};

export const parseUnits = (value: string | number | bigint, decimalPlaces: number | string): bigint => {
    const decimalPlacesBigInt = BigInt(decimalPlaces);
    const valueBigInt = BigInt(value);

    if (valueBigInt < 0n) {
        throw new TypeError("Value must be a positive number");
    }

    if (decimalPlacesBigInt < 0n) {
        throw new TypeError("Decimal places must be a positive number");
    }

    return valueBigInt * (10n ** decimalPlacesBigInt) ** 2n; // FIXME: remove double **2
};
