
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { useMutation } from "@tanstack/react-query";
import {
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
  TokenAccountNotFoundError,
} from "@solana/spl-token";
import { Transaction, PublicKey, TransactionInstruction } from "@solana/web3.js";

interface CreateUserTokenAccountsInput {
  baseMint: PublicKey;
  quoteMint: PublicKey;
}

interface CreateUserTokenAccountsOutput {
  baseATA: PublicKey;
  quoteATA: PublicKey;
  signature: string | null;
  baseCreated: boolean;
  quoteCreated: boolean;
}

export const useCreateUserTokenAccounts = () => {
  const { publicKey, sendTransaction } = useWallet();
  const { connection } = useConnection();

  return useMutation
    <CreateUserTokenAccountsOutput,
    Error,
    CreateUserTokenAccountsInput
  >({
    mutationKey: ["createUserTokenAccounts"],
    mutationFn: async ({ baseMint, quoteMint }) => {
      // Validation
      if (!publicKey) {
        throw new Error("Wallet not connected. Please connect your wallet.");
      }

      console.log("═══════════════════════════════════════════════════");
      console.log("🔍 CREATE USER TOKEN ACCOUNTS");
      console.log("═══════════════════════════════════════════════════");
      console.log("Wallet:", publicKey.toString());
      console.log("Base Mint:", baseMint.toString());
      console.log("Quote Mint:", quoteMint.toString());

      // Step 1: Derive ATA addresses
      const baseATA = getAssociatedTokenAddressSync(
        baseMint,
        publicKey,
        false,
        TOKEN_PROGRAM_ID
      );

      const quoteATA = getAssociatedTokenAddressSync(
        quoteMint,
        publicKey,
        false,
        TOKEN_PROGRAM_ID
      );

      console.log("\n📍 Derived ATA Addresses:");
      console.log("  Base ATA:", baseATA.toString());
      console.log("  Quote ATA:", quoteATA.toString());

      // Step 2: Check which accounts exist
      console.log("\n🔎 Checking account existence...");

      const checkAccount = async (
        ata: PublicKey,
        name: string
      ): Promise<boolean> => {
        try {
          await getAccount(connection, ata, "confirmed", TOKEN_PROGRAM_ID);
          console.log(`  ✅ ${name} exists`);
          return true;
        } catch (error) {
          if (error instanceof TokenAccountNotFoundError) {
            console.log(`  ❌ ${name} does not exist`);
            return false;
          }
          throw error;
        }
      };

      const [baseExists, quoteExists] = await Promise.all([
        checkAccount(baseATA, "Base ATA"),
        checkAccount(quoteATA, "Quote ATA"),
      ]);

      // Step 3: Early return if both exist
      if (baseExists && quoteExists) {
        console.log("\n✅ Both accounts already exist. No action needed.");
        console.log("═══════════════════════════════════════════════════\n");
        return {
          baseATA,
          quoteATA,
          signature: null,
          baseCreated: false,
          quoteCreated: false,
        };
      }

      // Step 4: Build transaction for missing accounts
      console.log("\n🔨 Building transaction...");
      const instructions: TransactionInstruction[] = [];

      if (!baseExists) {
        console.log("  ➕ Adding Base ATA creation instruction");
        instructions.push(
          createAssociatedTokenAccountInstruction(
            publicKey, // payer
            baseATA, // ata
            publicKey, // owner
            baseMint, // mint
            TOKEN_PROGRAM_ID
          )
        );
      }

      if (!quoteExists) {
        console.log("  ➕ Adding Quote ATA creation instruction");
        instructions.push(
          createAssociatedTokenAccountInstruction(
            publicKey,
            quoteATA,
            publicKey,
            quoteMint,
            TOKEN_PROGRAM_ID
          )
        );
      }

      const transaction = new Transaction().add(...instructions);

      // Step 5: Get latest blockhash
      console.log("\n⏳ Getting latest blockhash...");
      const { blockhash, lastValidBlockHeight } =
        await connection.getLatestBlockhash("finalized");

      transaction.recentBlockhash = blockhash;
      transaction.feePayer = publicKey;

      console.log("  Blockhash:", blockhash);
      console.log("  Last valid block height:", lastValidBlockHeight);

      // Step 6: Send transaction
      console.log("\n📤 Sending transaction...");
      console.log(`  Instructions: ${instructions.length}`);
      console.log(
        `  Estimated cost: ~${(instructions.length * 0.002).toFixed(4)} SOL`
      );

      let signature: string;
      try {
        signature = await sendTransaction(transaction, connection, {
          skipPreflight: false,
          preflightCommitment: "confirmed",
          maxRetries: 3,
        });

        if (!signature) {
          throw new Error("Transaction signature is undefined");
        }

        console.log("  ✅ Transaction sent");
        console.log("  Signature:", signature);
      } catch (error: any) {
        console.error("\n❌ Failed to send transaction");
        console.error("  Error:", error.message);

        // Extract meaningful error
        if (error?.logs) {
          console.error("  Logs:", error.logs.join("\n    "));
        }

        throw new Error(
          `Failed to create token accounts: ${
            error.message || "Unknown error"
          }`
        );
      }

      // Step 7: Confirm transaction
      console.log("\n⏳ Confirming transaction...");
      try {
        const confirmation = await connection.confirmTransaction(
          {
            signature,
            blockhash,
            lastValidBlockHeight,
          },
          "confirmed"
        );

        if (confirmation.value.err) {
          throw new Error(
            `Transaction failed: ${JSON.stringify(confirmation.value.err)}`
          );
        }

        console.log("  ✅ Transaction confirmed");
      } catch (error: any) {
        console.error("\n❌ Failed to confirm transaction");
        console.error("  Error:", error.message);

        // Transaction might still have succeeded
        console.log("  ⚠️  Checking transaction status...");
        try {
          const status = await connection.getSignatureStatus(signature);
          if (
            status?.value?.confirmationStatus === "confirmed" ||
            status?.value?.confirmationStatus === "finalized"
          ) {
            console.log("  ✅ Transaction was actually confirmed!");
          } else {
            throw error;
          }
        } catch (statusError) {
          throw new Error(
            `Failed to confirm transaction: ${error.message || "Unknown error"}`
          );
        }
      }

      // Step 8: Verify accounts were created
      console.log("\n🔍 Verifying account creation...");
      const [baseNowExists, quoteNowExists] = await Promise.all([
        checkAccount(baseATA, "Base ATA"),
        checkAccount(quoteATA, "Quote ATA"),
      ]);

      if (
        (!baseExists && !baseNowExists) ||
        (!quoteExists && !quoteNowExists)
      ) {
        throw new Error(
          "Accounts were not created successfully. Please try again."
        );
      }

      console.log("\n✅ SUCCESS");
      console.log(`  Base ATA: ${!baseExists ? "Created" : "Already existed"}`);
      console.log(
        `  Quote ATA: ${!quoteExists ? "Created" : "Already existed"}`
      );
      console.log(
        `  🔗 Explorer: https://explorer.solana.com/tx/${signature}?cluster=devnet`
      );
      console.log("═══════════════════════════════════════════════════\n");

      return {
        baseATA,
        quoteATA,
        signature,
        baseCreated: !baseExists,
        quoteCreated: !quoteExists,
      };
    },
  });
};