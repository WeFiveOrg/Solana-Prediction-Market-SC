import * as anchor from "@coral-xyz/anchor";
import { BN, Program, web3 } from "@coral-xyz/anchor";
import fs from "fs";

import { Keypair, Connection, PublicKey, Transaction, VersionedTransaction } from "@solana/web3.js";

import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

import { Takesfun } from "../target/types/takesfun";
import {
  createConfigTx,
  createMarketTx,
  mintNoTokenTx,
  swapTx,
  creatorClaimTx,
  creatorClaimSecondTx,
  swapSecondTx,
} from "../lib/scripts";
import { execTx, execVtx } from "../lib/util";
import {
  TEST_DECIMALS,
  TEST_YES_NAME,
  TEST_YES_SYMBOL,
  TEST_YES_URI,
  MARKET_INFO,
  TEST_NO_NAME,
  TEST_NO_SYMBOL,
  TEST_NO_URI,
  TEST_TOKEN_SUPPLY,
  TEST_VIRTUAL_RESERVES,
  TEST_LIMIT_TIMESTAMP,
  TEST_INITIAL_VIRTUAL_TOKEN_RESERVES,
  TEST_INITIAL_VIRTUAL_SOL_RESERVES,
  TEST_INITIAL_REAL_TOKEN_RESERVES,
  SEED_CONFIG,
  SEED_MARKET,
  SEED_WHITELIST,
  CREATOR,
} from "../lib/constant";

import crypto from 'crypto';

let solConnection: Connection = null;
let program: Program<Takesfun> = null;
let payer: NodeWallet = null;
let walletKeypair: Keypair = null;

/**
 * Set cluster, provider, program
 * If rpc != null use rpc, otherwise use cluster param
 * @param cluster - cluster ex. mainnet-beta, devnet ...
 * @param keypair - wallet keypair
 * @param rpc - rpc
 */
export const setClusterConfig = async (
  cluster: web3.Cluster,
  keypair: string,
  rpc?: string
) => {
  if (!rpc) {
    solConnection = new web3.Connection(web3.clusterApiUrl(cluster));
  } else {
    solConnection = new web3.Connection(rpc);
  }

  walletKeypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(fs.readFileSync(keypair, "utf-8"))),
    { skipValidation: true }
  );
  payer = new NodeWallet(walletKeypair);

  console.log("Wallet Address: ", payer.publicKey.toBase58());

  anchor.setProvider(
    new anchor.AnchorProvider(solConnection, payer, {
      skipPreflight: true,
      commitment: "confirmed",
    })
  );

  // Generate the program client from IDL.
  program = anchor.workspace.takesfun as Program<Takesfun>;

  console.log("ProgramId: ", program.programId.toBase58());


};

export const configProject = async () => {
  // Create a dummy config object to pass as argument.
  const newConfig = {
    authority: payer.publicKey,
    pendingAuthority: PublicKey.default,
    backendSignAuthority: payer.publicKey,

    teamWallet: payer.publicKey,
    teamWallet2: payer.publicKey,

    platformBuyFee: new BN(100), // Example fee: 1%
    platformSellFee: new BN(100), // Example fee: 1%

    platformBuySmallFee: new BN(80), // Example fee: 0.8%
    platformSellSmallFee: new BN(80), // Example fee: 0.8%

    creatorBuyFee: new BN(20), // Example fee: 0.2%
    creatorSellFee: new BN(20), // Example fee: 0.2%

    tokenSupplyConfig: new BN(TEST_INITIAL_REAL_TOKEN_RESERVES),
    tokenDecimalsConfig: 6,

    initialVirtualYesTokenReservesConfig: new BN(TEST_INITIAL_VIRTUAL_TOKEN_RESERVES),
    initialVirtualYesSolReservesConfig: new BN(TEST_INITIAL_VIRTUAL_SOL_RESERVES),
    initialRealYesTokenReservesConfig: new BN(TEST_INITIAL_REAL_TOKEN_RESERVES),

    initialVirtualNoTokenReservesConfig: new BN(TEST_INITIAL_VIRTUAL_TOKEN_RESERVES),
    initialVirtualNoSolReservesConfig: new BN(TEST_INITIAL_VIRTUAL_SOL_RESERVES),
    initialRealNoTokenReservesConfig: new BN(TEST_INITIAL_REAL_TOKEN_RESERVES),

    limitTimestamp: new BN(TEST_LIMIT_TIMESTAMP),

    crossSolFactor: 0.2,
    minSolLiquidity: new BN(5_000),

    initialized: true,
  };
  const tx = await createConfigTx(
    payer.publicKey,
    newConfig,
    solConnection,
    program
  );

  await execTx(tx, solConnection, payer);
};

export const createMarket = async () => {

  const noTokenMintTx = await mintNoTokenTx(

    //  metadata
    TEST_NO_SYMBOL,
    TEST_NO_URI,

    payer.publicKey,

    solConnection,
    program
  );

  const configPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_CONFIG)],
    program.programId
  )[0];

  const configAccount = await program.account.config.fetch(configPda);

  const marketCreationTx = await createMarketTx(

    //  metadata
    TEST_YES_SYMBOL,
    TEST_YES_URI,
    MARKET_INFO,

    payer.publicKey,
    configAccount.teamWallet,
    noTokenMintTx.no_tokenKp.publicKey,


    solConnection,
    program
  );

  const transaction = new Transaction()
  transaction.add(...noTokenMintTx.tx.instructions)
  transaction.add(...marketCreationTx.tx.instructions)

  transaction.feePayer = payer.publicKey;
  transaction.recentBlockhash = (await solConnection.getLatestBlockhash()).blockhash;
  transaction.sign(noTokenMintTx.no_tokenKp, marketCreationTx.yes_tokenKp);



  await execTx(transaction, solConnection, payer);

  const marketPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_MARKET), marketCreationTx.yes_tokenKp.publicKey.toBytes(), noTokenMintTx.no_tokenKp.publicKey.toBytes()],
    program.programId
  )[0];

  console.log("🚀 ~ createMarket ~ no_tokenKp:", noTokenMintTx.no_tokenKp.publicKey.toBase58());
  console.log("🚀 ~ createMarket ~ yes_tokenKp:", marketCreationTx.yes_tokenKp.publicKey.toBase58());
  console.log("🚀 ~ createMarket ~ marketPda:", marketPda.toBase58())
  const marketAccount = await program.account.market.fetch(marketPda);
  console.log("🚀 ~ createMarket ~ marketAccount:", marketAccount)

};


export const swap = async (
  yes_token: PublicKey,
  no_token: PublicKey,

  amount: number,
  style: number,
  token_type: number,
) => {
  const tx = await swapTx(
    payer.publicKey,
    yes_token,
    no_token,
    amount,
    style,
    token_type,
    solConnection,
    program
  );

  await execTx(tx, solConnection, payer);

  const marketPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_MARKET), yes_token.toBytes(), no_token.toBytes()],
    program.programId
  )[0];

  console.log("🚀 ~ createMarket ~ marketPda:", marketPda.toBase58())
  const marketAccount = await program.account.market.fetch(marketPda);
  console.log("🚀 ~ createMarket ~ marketAccount:", marketAccount)

};

export const creatorClaim = async (
  yes_token: PublicKey,
  no_token: PublicKey,
) => {
  const tx = await creatorClaimTx(payer.publicKey, yes_token, no_token, solConnection, program);
  await execTx(tx, solConnection, payer);
};

export const whitelist = async (
  backendTx: string,
) => {

  const serializedBuffer: Buffer = Buffer.from(backendTx, "base64");
  console.log("🚀 ~ serializedBuffer:", serializedBuffer)

  const vtx: VersionedTransaction = VersionedTransaction.deserialize(Uint8Array.from(serializedBuffer));
  vtx.sign([walletKeypair]);
  console.log("🚀 ~ walletKeypair:", walletKeypair)

  await execVtx(vtx, solConnection);

  const whitelistPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_WHITELIST), payer.publicKey.toBytes()],
    program.programId
  )[0];
  console.log("🚀 ~ createMarket ~ marketPda:", whitelistPda.toBase58());

  const whitelistAccount = await program.account.whitelist.fetch(whitelistPda);
  console.log("🚀 ~ createMarket ~ marketAccount:", whitelistAccount);
};

export const createMarketSecondary = async (backendTx: string,) => {
  const serializedBuffer: Buffer = Buffer.from(backendTx, "base64");
  console.log("🚀 ~ serializedBuffer:", serializedBuffer)

  const vtx: VersionedTransaction = VersionedTransaction.deserialize(Uint8Array.from(serializedBuffer));
  vtx.sign([walletKeypair]);
  console.log("🚀 ~ walletKeypair:", walletKeypair);

  await execVtx(vtx, solConnection);
}

export const swapSecond = async (
  yes_token: PublicKey,
  no_token: PublicKey,

  market_info: string,

  amount: number,
  style: number,
  token_type: number,
) => {
  const tx = await swapSecondTx(
    payer.publicKey,
    yes_token,
    no_token,
    market_info,
    amount,
    style,
    token_type,
    solConnection,
    program
  );

  await execTx(tx, solConnection, payer);

  let hexString = crypto.createHash('sha256').update(market_info, 'utf-8').digest('hex');
  let market_info_hash = Uint8Array.from(Buffer.from(hexString, 'hex'));

  let [marketPDA, _] = await PublicKey.findProgramAddress([
    Buffer.from(SEED_MARKET), market_info_hash
  ], program.programId);

  console.log("🚀 ~ createMarket ~ marketPda:", marketPDA.toBase58())
  const marketAccount = await program.account.market.fetch(marketPDA);
  console.log("🚀 ~ createMarket ~ marketAccount:", marketAccount)

};

export const changeCreator = async (backendTx: string) => {
  const serializedBuffer: Buffer = Buffer.from(backendTx, "base64");
  console.log("🚀 ~ serializedBuffer:", serializedBuffer)

  const vtx: VersionedTransaction = VersionedTransaction.deserialize(Uint8Array.from(serializedBuffer));
  vtx.sign([walletKeypair]);
  console.log("🚀 ~ walletKeypair:", walletKeypair);

  await execVtx(vtx, solConnection);
}

export const creatorClaimSecond = async (
  yes_token: PublicKey,
  no_token: PublicKey,
  market_info: string,
) => {
  const tx = await creatorClaimSecondTx(payer.publicKey, yes_token, no_token, market_info, solConnection, program);
  await execTx(tx, solConnection, payer);
};


function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
} 