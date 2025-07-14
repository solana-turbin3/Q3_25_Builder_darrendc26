import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";

import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { Buffer } from "buffer";

import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  Account,
  TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  
} from "@solana/spl-token";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.escrow as Program<Escrow>;

  // all accounts
  let maker: Keypair;
  let taker: Keypair;
  let mintA: PublicKey;
  let mintB: PublicKey;
  let makerAtaA: Account;
  let makerAtaB: Account;
  let takerAtaA: Account;
  let takerAtaB: Account;
  let vault: PublicKey;
  let escrow: PublicKey;
  let seed: anchor.BN;
  
  beforeEach(async () => {
    maker = anchor.web3.Keypair.generate();
    taker = anchor.web3.Keypair.generate();
    console.log("Maker public key:", maker.publicKey.toBase58());
    console.log("Taker public key:", taker.publicKey.toBase58());
    
    // airdrop SOL to maker and taker
    await connection.confirmTransaction(
      await connection.requestAirdrop(maker.publicKey, 2 * LAMPORTS_PER_SOL),
      "confirmed" 
    );
    await connection.confirmTransaction(
      await connection.requestAirdrop(taker.publicKey, 2 * LAMPORTS_PER_SOL),
      "confirmed"
    );
    console.log("Airdropped 2 SOL to maker and taker accounts");

    // create mint A and mint B
    mintA = await createMint(
      connection,
      maker,
      maker.publicKey,
      null,
      6 // decimals
    );
    console.log("Mint A created:", mintA.toBase58());

    mintB = await createMint(
      connection,
      maker, // maker creates both mints
      maker.publicKey,
      null,
      6 // decimals
    );
    console.log("Mint B created:", mintB.toBase58());

    // create associated token accounts for maker and taker
    makerAtaA = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mintA,
      maker.publicKey
    );
    console.log("Maker ATA A created:", makerAtaA.address.toBase58());

    makerAtaB = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mintB,
      maker.publicKey
    );
    console.log("Maker ATA B created:", makerAtaB.address.toBase58());

    // create associated token accounts for taker
    takerAtaA = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mintA,
      taker.publicKey
    );
    console.log("Taker ATA A created:", takerAtaA.address.toBase58());

    takerAtaB = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mintB,
      taker.publicKey
    );  
    console.log("Taker ATA B created:", takerAtaB.address.toBase58());

    // mint tokens to maker's ATA A (maker has mint authority)
    await mintTo(
      connection,
      maker,
      mintA,
      makerAtaA.address,
      maker.publicKey,
      10 * Math.pow(10, 6) // 10 tokens with 6 decimals
    );
    console.log("Minted 10 tokens to maker ATA A");

    // mint tokens to taker's ATA B (maker has mint authority for both mints)
    await mintTo(
      connection,
      maker,
      mintB,
      takerAtaB.address,
      maker.publicKey, // maker is the mint authority
      10 * Math.pow(10, 6) // 10 tokens with 6 decimals
    );
    console.log("Minted 10 tokens to taker ATA B");

    // create seed for PDA
    seed = new anchor.BN(42);

    // create vault account (ATA for escrow PDA)
    const [escrowPda, escrowBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8), // u64 -> little-endian
      ],
      program.programId
    );
    
    escrow = escrowPda;
    console.log("Escrow PDA created:", escrow.toBase58());

    // create vault as ATA for the escrow PDA
    vault = getAssociatedTokenAddressSync(
      mintA,
      escrow, // vault is owned by the escrow PDA
      true, // allowOwnerOffCurve = true for PDA
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    console.log("Vault created:", vault.toBase58());
  });

  it("Make escrow!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(seed, new BN(1e6), new anchor.BN(1e6))
      .accountsPartial({
        maker: maker.publicKey,
        escrow,
        vault,
        makerAtaA: makerAtaA.address,
        mintA: mintA,
        mintB: mintB,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([maker])
      .rpc();

    console.log("Make escrow signature", tx);
  });

  it("Take escrow!", async () => {
    const tx = await program.methods
      .initialize(seed, new BN(1e6), new anchor.BN(1e6))
      .accountsPartial({
        maker: maker.publicKey,
        escrow,
        vault,
        makerAtaA: makerAtaA.address,
        mintA: mintA,
        mintB: mintB,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([maker])
      .rpc();

    console.log("Make escrow signature", tx);

    const tx1 = await program.methods.take(new BN(1e6)).accountsPartial({
      escrow,
      taker: taker.publicKey,
      maker: maker.publicKey,
      mintA,
      mintB,
      makerAtaA: makerAtaA.address,
      takerAtaB: takerAtaB.address,
      vault,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,

    }).signers([maker, taker]).rpc();
    console.log("Take escrow signature", tx);
  });
});
