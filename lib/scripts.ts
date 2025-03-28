import { BN, Program } from "@coral-xyz/anchor";
import {
  ComputeBudgetProgram,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
} from "@solana/web3.js";

import { Takesfun } from "../target/types/takesfun";
import {
  ammProgram,
  feeDestination,
  marketProgram,
  SEED_MARKET,
  SEED_CONFIG,
} from "./constant";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  NATIVE_MINT,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

import crypto from 'crypto';

export const createConfigTx = async (
  admin: PublicKey,

  newConfig: any,

  connection: Connection,
  program: Program<Takesfun>
) => {
  const [configPda, _] = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_CONFIG)],
    program.programId
  );

  console.log("configPda: ", configPda.toBase58());

  const tx = await program.methods
    .configure(newConfig)
    .accounts({
      payer: admin,
    })
    .transaction();

  console.log("configPda after: ", configPda.toBase58());

  tx.feePayer = admin;
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

  return tx;
};

export const createMarketTx = async (

  yes_symbol: string,   //Yes Token Symbol
  yes_uri: string,      //Yes Token Uri

  market_info: string,  //market info

  user: PublicKey,      //user pubkey
  teamWallet: PublicKey,//teamWallet pubkey  
  noToken: PublicKey,   //noToken pubkey

  connection: Connection,
  program: Program<Takesfun>
) => {
  const yes_tokenKp = Keypair.generate();
  const no_tokenKp = Keypair.generate();

  console.log("🚀 ~ yes_tokenKp:", yes_tokenKp.publicKey.toBase58());

  // Send the transaction to launch a token
  const tx = await program.methods
    .createMarket(
      //  metadata
      yes_symbol,
      yes_uri,
      market_info,
    )
    .accounts({
      yesToken: yes_tokenKp.publicKey,
      noToken,//no_tokenKp.publicKey,
      creator: user,
      teamWallet: teamWallet,
    })
    .transaction();

  tx.feePayer = user;
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.sign(yes_tokenKp);
  // tx.partialSign(no_tokenKp);

  return { tx, yes_tokenKp };
};

export const mintNoTokenTx = async (

  no_symbol: string,  //no Token Symbol
  no_uri: string,     //no Token Uri

  user: PublicKey,    //user pubkey

  connection: Connection,
  program: Program<Takesfun>
) => {

  const no_tokenKp = Keypair.generate();
  console.log("🚀 ~ no_tokenKp:", no_tokenKp.publicKey.toBase58());

  // Send the transaction to launch a token
  const tx = await program.methods
    .mintNoToken(
      //  metadata
      no_symbol,
      no_uri,
    )
    .accounts({
      noToken: no_tokenKp.publicKey,
      creator: user,
    })
    .transaction();
  tx.feePayer = user;
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.sign(no_tokenKp);

  // const sim = await connection.simulateTransaction(tx)
  // console.log('no token createion sim', sim)
  return { tx, no_tokenKp };
};

export const swapTx = async (
  user: PublicKey,
  yes_token: PublicKey,
  no_token: PublicKey,

  amount: number,
  style: number,
  token_type: number,

  connection: Connection,
  program: Program<Takesfun>
) => {
  const configPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_CONFIG)],
    program.programId
  )[0];
  const configAccount = await program.account.config.fetch(configPda);

  const marketPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_MARKET), yes_token.toBytes(), no_token.toBytes()],
    program.programId
  )[0];
  const marketAccount = await program.account.market.fetch(marketPda);

  const tx = await program.methods
    .swap(new BN(amount), style, token_type, new BN(0))
    .accounts({
      teamWallet: configAccount.teamWallet,
      teamWallet2: configAccount.teamWallet2,
      creator: marketAccount.creator,
      user,
      noToken: no_token,
      yesToken: yes_token,
    })
    .transaction();

  tx.feePayer = user;
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

  return tx;
};

export const creatorClaimTx = async (
  creator: PublicKey,
  yes_token: PublicKey,
  no_token: PublicKey,

  connection: Connection,
  program: Program<Takesfun>
) => {

  const tx = await program.methods
    .creatorClaim()
    .accounts({
      creator,
      yesToken: yes_token,
      noToken: no_token,

    })
    .transaction();

  tx.feePayer = creator;
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

  return tx;
};

export const creatorClaimSecondTx = async (
  creator: PublicKey,
  yes_token: PublicKey,
  no_token: PublicKey,
  market_info: string,

  connection: Connection,
  program: Program<Takesfun>
) => {
  console.log("🚀 ~ market_info:", market_info);

  let hexString = crypto.createHash('sha256').update(market_info, 'utf-8').digest('hex');
  let market_info_hash = Uint8Array.from(Buffer.from(hexString, 'hex'));

  let [marketPDA, _] = await PublicKey.findProgramAddress([
    Buffer.from(SEED_MARKET), market_info_hash
  ], program.programId);
  console.log("🚀 ~ marketPDA:", marketPDA.toBase58());

  const tx = await program.methods
    .creatorClaimSecond(market_info)
    .accounts({
      creator,
      yesToken: yes_token,
      noToken: no_token,
      //@ts-ignore
      market: marketPDA,
    })
    .transaction();

  tx.feePayer = creator;
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

  return tx;
};

export const swapSecondTx = async (
  user: PublicKey,
  yes_token: PublicKey,
  no_token: PublicKey,
  market_info: string,

  amount: number,
  style: number,
  token_type: number,

  connection: Connection,
  program: Program<Takesfun>
) => {
  const configPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_CONFIG)],
    program.programId
  )[0];
  const configAccount = await program.account.config.fetch(configPda);

  console.log("🚀 ~ market_info:", market_info);

  let hexString = crypto.createHash('sha256').update(market_info, 'utf-8').digest('hex');
  let market_info_hash = Uint8Array.from(Buffer.from(hexString, 'hex'));

  let [marketPDA, _] = await PublicKey.findProgramAddress([
    Buffer.from(SEED_MARKET), market_info_hash
  ], program.programId);
  console.log("🚀 ~ marketPDA:", marketPDA.toBase58());

  const marketAccount = await program.account.market.fetch(marketPDA);

  const tx = await program.methods
    .swapSecond(market_info, new BN(amount), style, token_type, new BN(0))
    .accounts({
      teamWallet: configAccount.teamWallet,
      teamWallet2: configAccount.teamWallet2,
      creator: marketAccount.creator,
      user,
      noToken: no_token,
      yesToken: yes_token,
      //@ts-ignore
      market: marketPDA,
    })
    .transaction();

  tx.feePayer = user;
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

  return tx;
};