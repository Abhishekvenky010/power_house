import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { createMint, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import assert from "assert";
import { expect } from "chai";

describe("init market", () => {
  it("init market", async () => {
    const connection = new anchor.web3.Connection("http://127.0.0.1:8899", "confirmed");
    const walletPath = process.env.ANCHOR_WALLET || "/home/abhishek/.config/solana/id.json";
    const walletKeypair = Keypair.fromSecretKey(
      Buffer.from(JSON.parse(require("fs").readFileSync(walletPath, "utf8")))
    );
    const wallet = new anchor.Wallet(walletKeypair);
    const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
    anchor.setProvider(provider);

    const idl = JSON.parse(require("fs").readFileSync("/home/abhishek/projects/power_house/my-dex/target/idl/orderbook.json", "utf8"));
    const program = new anchor.Program(idl, provider);

    const baseMint = await createMint(connection, walletKeypair, walletKeypair.publicKey, walletKeypair.publicKey, 6);
    const quoteMint = await createMint(connection, walletKeypair, walletKeypair.publicKey, walletKeypair.publicKey, 6);

    const marketKeypair = Keypair.generate();

    const [vaultSigner] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault_signer"), marketKeypair.publicKey.toBuffer()],
      program.programId
    );
    const bidsPda = PublicKey.findProgramAddressSync(
      [Buffer.from("bids"), marketKeypair.publicKey.toBuffer()],
      program.programId
    )[0];
    const asksPda = PublicKey.findProgramAddressSync(
      [Buffer.from("asks"), marketKeypair.publicKey.toBuffer()],
      program.programId
    )[0];

    const tx = await program.methods
      .initialiseMarket(new anchor.BN(1000), new anchor.BN(1000), new anchor.BN(0), new anchor.BN(0))
      .accounts({
        market: marketKeypair.publicKey,
        bids: bidsPda,
        asks: asksPda,
        baseVault: vaultSigner,
        quoteVault: vaultSigner,
        vaultSigner: vaultSigner,
        admin: walletKeypair.publicKey,
        baseMint,
        quoteMint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([marketKeypair])
      .rpc();

    console.log("tx:", tx);
    console.log("market:", marketKeypair.publicKey.toBase58());
  });
});
