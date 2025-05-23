/* eslint-disable @typescript-eslint/no-magic-numbers */
/* eslint-disable no-console */
import {
    AccountId,
    PublicKey,
    CryptoSigner,
} from "../../src";
import { CreateLedgerTransfers, FinalizedTransfer_TransferState, Target, type FinalizedTransfer } from "../../src/protobufs/sdk/transaction/transaction";
import { Contract, QuoteEvent } from "../../src/protobufs/sdk/metadata";
import { QuoteRequest } from "../../src/protobufs/sdk/metadata";
import { bytesToHex, hexToBytes } from "@noble/hashes/utils";
import { expect } from "chai";
import { createCurrencyAccounts, EUR_BANK_ID, initTestCaseInstances, LEDGER_ID, parseUnits, testCase, USD_BANK_ID, type TestCaseInstances } from "../config";

const QUOTE_TIMEOUT_MS = 10_000;
const FX_QUOTE = "fx.quote";

describe("FX Service", () => {
    let testCaseInstances: TestCaseInstances;

    it("init", async () => {
        testCaseInstances = await initTestCaseInstances();
    });

    it("test public key", async () => {
        const newPK = await CryptoSigner.fromSeed(hexToBytes("8eefc537d5e1be7b0e3600ff5a773bd737b4c942f611eac1b936f436fc2f6759"));
        const publicKeyHex = bytesToHex(newPK.getPublicKey().toUint8Array());
        expect(publicKeyHex).to.be.equal("0bad513fafd266eaea7808d96c763235f096237433a51ba5603db112d2970009");
    });
    let accountNames: string[] = [];
    let accountIds: Record<string, AccountId> = {};
    it("create USD/EUR/TOKEN currency accounts", async () => {
        const currencyAccounts = await createCurrencyAccounts(testCaseInstances);
        accountNames = currencyAccounts.accountNames;
        accountIds = currencyAccounts.accountIds;
    }).timeout(10_000);
    it("list accounts by public key", async () => {
        const accountPubKey = PublicKey.fromUint8Array(testCaseInstances.accountSigner.getPublicKey().toUint8Array());
        const accounts = await testCaseInstances.accountClient.listAccounts({
            filter: {
                oneofKind: "owner",
                owner: accountPubKey.toUint8Array(),
            },
        });

        expect(accounts.length).to.be.greaterThan(0);
    });
    it("list accounts by names", async () => {
        await Promise.all(accountNames.map(async name => {
            const accounts = await testCaseInstances.accountClient.listAccounts({
                filter: {
                    oneofKind: "name",
                    name: name,
                },
            });

            expect(accounts.length).to.be.greaterThan(0);
        }));
    });
    it("find account ids by alias names", async () => {
        await Promise.all(accountNames.map(async name => {
            const aliasesResponse = await testCaseInstances.directoryClient.getAlias(name);

            const accountId = AccountId.fromBytes(aliasesResponse.aliases[0].accountSetId);

            expect(accountId.hex).to.be.equal(name.replace("kek_name-", ""));
        }));
    }).timeout(10_000);
    it("Find account ids by bank id", async () => {
        const accountPubKey = PublicKey.fromUint8Array(testCaseInstances.accountSigner.getPublicKey().toUint8Array());
        const accountListAccounts = await testCaseInstances.accountClient.listAccounts({
            filter: {
                oneofKind: "owner",
                owner: accountPubKey.toUint8Array(),
            },
        });

        const accountListAccountsInfos = await Promise.all(
            accountListAccounts.map(async (account) => {
                return await testCaseInstances.accountClient.getAccountInfo(AccountId.fromBytes(account.id));
            }),
        );

        await Promise.all([USD_BANK_ID, EUR_BANK_ID].map(async (bankId) => {
            const currentBankCustomerAccounts = accountListAccountsInfos.filter(account => {
                return AccountId.fromBytes(account.parentAccountId).hex === bankId.hex;
            });

            expect(currentBankCustomerAccounts.map(el => AccountId.fromBytes(el.parentAccountId).hex)).to.contain(bankId.hex);
        }));
    });
    it("fund currency accounts", async () => {
        if (!Object.values(accountIds).length) throw new TypeError("Account IDs are not initialized");

        await Promise.all([USD_BANK_ID, EUR_BANK_ID].map(async (bankId) => {
            const bankInfo = await testCaseInstances.accountClient.getAccountInfo(bankId);

            await testCaseInstances.operatorClient.transfer([
                {
                    fromAccountId: bankId.bytes,
                    toAccountId: accountIds[bankId.hex].bytes,
                    amount: parseUnits(parseUnits(10, bankInfo.decimalPlaces), bankInfo.decimalPlaces),
                },
            ]);
        }));
    });
    it("get currency accounts info | metadatas | balances", async () => {
        if (!Object.values(accountIds).length) throw new TypeError("Account IDs are not initialized");

        await Promise.all(Object.values(accountIds).map(async (accountId) => {
            const account = await testCaseInstances.accountClient.getAccount(accountId);
            const accountInfo = await testCaseInstances.accountClient.getAccountInfo(accountId);
            // const accountMetadata = await accountClient.getAccountMetadata(
            //     DocumentId.fromUint8Array("bank", accountId.toUint8Array()),
            // );
            const bankInfo = await testCaseInstances.accountClient.getAccountInfo(AccountId.fromBytes(accountInfo.parentAccountId));

            // const calculatedRoot = AccountId.fromUint8Array(account.map((byte, index) => {
            //     return index === 0 ? (byte & 0xff) : 0x00;
            // }));

            console.log("Retrieved info:", {
                aliceBalance: `${
                    (account.balance / parseUnits(10, bankInfo.decimalPlaces))
                } ${accountInfo.code}`,
                bankName: bankInfo.publicName,
                bankImage: bankInfo.profileImageUrl,
                bankId: AccountId.fromBytes(bankInfo.accountId).hex,
                // rootAccountId: calculatedRoot.toString(),
                // rootName: accountMetadata.name,
                // rootImage: accountMetadata.profileImageUrl,
            });
        }));
    });
    it("transfer", async () => {
        if (!Object.values(accountIds).length) throw new TypeError("Account IDs are not initialized");

        const transactionReceipts: FinalizedTransfer[] = [];
        await Promise.all(Object.values(accountIds).map(async (accountId) => {
            const accountInfo = await testCaseInstances.accountClient.getAccountInfo(accountId);
            const bankInfo = await testCaseInstances.accountClient.getAccountInfo(AccountId.fromBytes(accountInfo.parentAccountId));

            const bankAccountId = AccountId.fromBytes(bankInfo.accountId);

            const to = {
                [testCase.bankUSDId]: testCase.aliceUSDAccountId,
                [testCase.bankEURId]: testCase.bobEURAccountId,
            }[bankAccountId.hex];

            const txId = await testCaseInstances.accountClient.transfer([
                {
                    fromAccountId: accountId.bytes,
                    toAccountId: AccountId.fromHex(to).bytes,
                    amount: parseUnits(1, bankInfo.decimalPlaces),
                },
            ]);

            const theTransfer = await testCaseInstances.accountClient.getTransfer(txId);
            transactionReceipts.push(theTransfer);
        }));
        expect(transactionReceipts.every(tx => tx.state === FinalizedTransfer_TransferState.ACCEPTED)).to.be.true;
    });
    it("transfer with FX", async () => {
        if (!Object.values(accountIds).length) throw new TypeError("Account IDs are not initialized");

        const transactionReceipts: FinalizedTransfer[] = [];
        await Promise.all(Object.values(accountIds).map(async (accountId) => {
            const abortController = new AbortController();
            const signal = abortController.signal;

            const accountInfo = await testCaseInstances.accountClient.getAccountInfo(accountId);
            const bankInfo = await testCaseInstances.accountClient.getAccountInfo(AccountId.fromBytes(accountInfo.parentAccountId));

            const bankAccountId = AccountId.fromBytes(bankInfo.accountId);

            const [currencyFrom, currencyTo] = {
                [testCase.bankUSDId]: ["USD", "EUR"],
                [testCase.bankEURId]: ["EUR", "USD"],
            }[bankAccountId.hex];

            const to = {
                [testCase.bankUSDId]: testCase.bobEURAccountId,
                [testCase.bankEURId]: testCase.aliceUSDAccountId,
            }[bankAccountId.hex];

            // Create a quote event for the FX exchange
            const quoteRequest = QuoteEvent.create({
                event: {
                    oneofKind: "request",
                    request: QuoteRequest.create({
                        base: {
                            amount: parseUnits(1, Number(bankInfo.decimalPlaces) - 1),
                            currency: currencyFrom,
                            accountId: accountId.bytes,
                            operator: LEDGER_ID,
                        },
                        target: {
                            currency: currencyTo,
                            accountId: AccountId.fromHex(to).bytes,
                            operator: LEDGER_ID,
                        },
                        memo: "Init exchange",
                    }),
                },
            });

            const startObservation = await testCaseInstances.accountClient.observeActions(
                {
                    name: FX_QUOTE,
                    involvesAccounts: [accountId.bytes],
                },
                {
                    abort: signal,
                },
            );

            const { quoteEvent, contextId } = await new Promise<{
                quoteEvent: QuoteEvent;
                contextId: Uint8Array<ArrayBufferLike>;
            }>((resolve, reject) => {
                // Begin observation
                const service = startObservation();

                const timeoutId = setTimeout(() => {
                    reject(new Error("Timeout waiting for QuoteProposal"));
                }, QUOTE_TIMEOUT_MS);

                service.responses.onNext((message, error) => {
                    if (error) {
                        clearTimeout(timeoutId);
                        reject(error);
                        abortController.abort();
                    }
                    if (message) {
                        try {
                            const payload = message.transactions[0]?.request?.data?.data?.oneofKind === "invokeAction"
                                ? message.transactions[0]?.request?.data?.data?.invokeAction?.payload
                                : undefined;

                            if (!payload) return;

                            const decodedQuote = QuoteEvent.fromBinary(payload);
                            if (
                                decodedQuote.event.oneofKind === "proposal" &&
                                decodedQuote.event.proposal &&
                                message.transactions[0].request?.contextId
                            ) {
                                clearTimeout(timeoutId);
                                resolve({
                                    quoteEvent: decodedQuote,
                                    contextId: message.transactions[0].request?.contextId,
                                });
                                abortController.abort();
                            }
                        } catch (err) {
                            console.error("Unexpected action received during quote observation.", err);
                        }
                    }
                });

                const fxAccountId = {
                    [testCase.bankUSDId]: AccountId.fromHex(testCase.fxUsdAccountId),
                    [testCase.bankEURId]: AccountId.fromHex(testCase.fxEurAccountId),
                }[bankAccountId.hex];

                // Dispatch the FX quote action
                testCaseInstances.accountClient.action({
                    name: FX_QUOTE,
                    fromAccount: accountId.bytes,
                    target: Target.create({
                        target: {
                            oneofKind: "accountId",
                            accountId: fxAccountId.bytes,
                        },
                    }),
                    payload: QuoteEvent.toBinary(quoteRequest),
                });
            });

            const proposal = quoteEvent?.event?.oneofKind === "proposal" ? quoteEvent.event?.proposal : undefined;

            if (!proposal) {
                throw new TypeError("No proposal found in the QuoteEvent.");
            }

            const contract = Contract.create(proposal);
            const ledgerTransfers = CreateLedgerTransfers.fromBinary(contract.transactions);
            const transferStep = ledgerTransfers.transfers[0].transfer?.transferSteps?.[0];

            if (!transferStep || !transferStep.fromAccountId || !transferStep.toAccountId || !transferStep.amount) {
                throw new Error("Incomplete transfer step information in contract.");
            }

            const txId = await testCaseInstances.accountClient.transfer([{
                fromAccountId: transferStep.fromAccountId,
                toAccountId: transferStep.toAccountId,
                amount: transferStep.amount,
                metadata: [
                    {
                        typeUrl: "m10.sdk.metadata.Contract",
                        value: Contract.toBinary(contract),
                    },
                ],
            }], contextId);

            const theTransfer = await testCaseInstances.accountClient.getTransfer(txId);
            transactionReceipts.push(theTransfer);
        }));

        expect(transactionReceipts.every(tx => tx.state === FinalizedTransfer_TransferState.ACCEPTED)).to.be.true;
    }).timeout(10_000);
    it("get currency accounts transactions", async () => {
        if (!Object.values(accountIds).length) throw new TypeError("Account IDs are not initialized");

        await Promise.all(Object.values(accountIds).map(async (accountId) => {
            const transactions = await testCaseInstances.accountClient.listTransfers({
                filter: {
                    oneofKind: "accountId",
                    accountId: accountId.bytes,
                },
            });
            console.log("Retrieved transactions:", transactions);
        }));
    });
    it("delete account", async () => {});
});
