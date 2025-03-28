import { MAINNET_PROGRAM_ID, DEVNET_PROGRAM_ID } from "@raydium-io/raydium-sdk";
import { Cluster, PublicKey } from "@solana/web3.js";

export const SEED_CONFIG = "config";
export const SEED_MARKET = "market";
export const SEED_WHITELIST = "whitelist";
export const CREATOR = "creator";

export const TEST_YES_NAME = "Agree";
export const TEST_YES_SYMBOL = "agree";
export const TEST_YES_URI =
  "https://gateway.irys.xyz/GwKuTp6xH8FktZcLfF9Uk7kPZX5iME5DsrPU2nVe6nWM";

export const TEST_NO_NAME = "Disagree";
export const TEST_NO_SYMBOL = "disagree";
export const TEST_NO_URI =
  "https://gateway.irys.xyz/AQtBhVsa5h6oj2XnBoEZu6xscRpzAYUKDrRAVnu3gK6E";

export const MARKET_INFO = "GwKuTp6xH8FktZcLfF9Uk7kPZX5iME5DsrPU2nVe6nWM";

export const TEST_VIRTUAL_RESERVES = 20_000_000_000;
export const TEST_TOKEN_SUPPLY = 1_000_000_000_000_000;
export const TEST_DECIMALS = 6;

export const TEST_LIMIT_TIMESTAMP = 2_592_000; // 3600 * 24 * 30

export const TEST_INITIAL_VIRTUAL_TOKEN_RESERVES = 1_000_000_000_000_000;
export const TEST_INITIAL_VIRTUAL_SOL_RESERVES = 20_000_000_000;
export const TEST_INITIAL_REAL_TOKEN_RESERVES = 1_000_000_000_000_000;

const cluster: Cluster = "devnet";

export const raydiumProgramId =
  cluster.toString() == "mainnet-beta" ? MAINNET_PROGRAM_ID : DEVNET_PROGRAM_ID;

export const ammProgram =
  cluster.toString() == "mainnet-beta"
    ? new PublicKey("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8") // mainnet-beta
    : new PublicKey("HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8"); // devnet

export const marketProgram =
  cluster.toString() == "mainnet-beta"
    ? new PublicKey("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX") // mainnet-beta
    : new PublicKey("EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj"); // devnet

export const feeDestination =
  cluster.toString() == "mainnet-beta"
    ? new PublicKey("7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5") // Mainnet
    : new PublicKey("3XMrhbv989VxAMi3DErLV9eJht1pHppW5LbKxe9fkEFR"); // Devnet
