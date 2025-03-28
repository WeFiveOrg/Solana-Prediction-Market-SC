import { program } from "commander";
import { PublicKey } from "@solana/web3.js";
import {
  configProject,
  createMarket,
  setClusterConfig,
  swap,
  whitelist,
  creatorClaim,
  createMarketSecondary,
  changeCreator,
  creatorClaimSecond,
  swapSecond,
} from "./scripts";

program.version("0.0.1");

programCommand("config").action(async (directory, cmd) => {
  const { env, keypair, rpc } = cmd.opts();

  console.log("Solana Cluster:", env);
  console.log("Keypair Path:", keypair);
  console.log("RPC URL:", rpc);


  await setClusterConfig(env, keypair, rpc);

  await configProject();
});

programCommand("market").action(async (directory, cmd) => {
  const { env, keypair, rpc } = cmd.opts();

  console.log("Solana Cluster:", env);
  console.log("Keypair Path:", keypair);
  console.log("RPC URL:", rpc);

  await setClusterConfig(env, keypair, rpc);

  await createMarket();
});

programCommand("swap")
  .option("-y, --yesToken <string>", "yesToken address")
  .option("-n, --noToken <string>", "noToken address")
  .option("-a, --amount <number>", "swap amount")
  .option("-s, --style <string>", "0: buy token, 1: sell token")
  .option("-t, --tokenType <string>", "0: no token, 1: yes token")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, yesToken, noToken, amount, style, tokenType } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (yesToken === undefined) {
      console.log("Error yesToken address");
      return;
    }

    if (noToken === undefined) {
      console.log("Error noToken address");
      return;
    }

    if (amount === undefined) {
      console.log("Error swap amount");
      return;
    }

    if (style === undefined) {
      console.log("Error swap style");
      return;
    }

    if (tokenType === undefined) {
      console.log("Error token style");
      return;
    }

    await swap(new PublicKey(yesToken), new PublicKey(noToken), amount, style, tokenType);
  });

programCommand("claim")
  .option("-y, --yesToken <string>", "yesToken address")
  .option("-n, --noToken <string>", "noToken address")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, yesToken, noToken } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (yesToken === undefined) {
      console.log("Error yesToken address");
      return;
    }

    if (noToken === undefined) {
      console.log("Error noToken address");
      return;
    }

    await creatorClaim(new PublicKey(yesToken), new PublicKey(noToken));
  });


programCommand("whitelist")
  .option("-t, --backendTx <string>", "transaction from backend")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, backendTx } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (backendTx === undefined) {
      console.log("Error backendTx");
      return;
    }

    await whitelist(backendTx);
  });

programCommand("market2")
  .option("-t, --backendTx <string>", "transaction from backend")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, backendTx } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (backendTx === undefined) {
      console.log("Error backendTx");
      return;
    }

    await createMarketSecondary(backendTx);

  });


programCommand("swap2")
  .option("-y, --yesToken <string>", "yesToken address")
  .option("-n, --noToken <string>", "noToken address")
  .option("-a, --amount <number>", "swap amount")
  .option("-s, --style <string>", "0: buy token, 1: sell token")
  .option("-t, --tokenType <string>", "0: no token, 1: yes token")
  .option("-m, --marketInfo <string>", "market info")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, yesToken, noToken, amount, style, tokenType, marketInfo } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (yesToken === undefined) {
      console.log("Error yesToken address");
      return;
    }

    if (noToken === undefined) {
      console.log("Error noToken address");
      return;
    }

    if (amount === undefined) {
      console.log("Error swap amount");
      return;
    }

    if (style === undefined) {
      console.log("Error swap style");
      return;
    }

    if (tokenType === undefined) {
      console.log("Error token style");
      return;
    }

    if (marketInfo === undefined) {
      console.log("Error marketInfo");
      return;
    }

    await swapSecond(new PublicKey(yesToken), new PublicKey(noToken), marketInfo, amount, style, tokenType);
  });

programCommand("changeCreator")
  .option("-t, --backendTx <string>", "transaction from backend")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, backendTx } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (backendTx === undefined) {
      console.log("Error backendTx");
      return;
    }

    await changeCreator(backendTx);

  });




programCommand("claim2")
  .option("-y, --yesToken <string>", "yesToken address")
  .option("-n, --noToken <string>", "noToken address")
  .option("-m, --marketInfo <string>", "market info")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, yesToken, noToken, marketInfo } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (yesToken === undefined) {
      console.log("Error yesToken address");
      return;
    }

    if (noToken === undefined) {
      console.log("Error noToken address");
      return;
    }

    if (marketInfo === undefined) {
      console.log("Error marketInfo");
      return;
    }

    await creatorClaimSecond(new PublicKey(yesToken), new PublicKey(noToken), marketInfo);
  });


function programCommand(name: string) {
  return program
    .command(name)
    .option(
      //  mainnet-beta, testnet, devnet
      "-e, --env <string>",
      "Solana cluster env name",
      "devnet"
    )
    .option(
      "-r, --rpc <string>",
      "Solana cluster RPC name",
      "https://devnet.helius-rpc.com/?api-key=d4602612-225a-400a-a286-1dd8fc2b378b" //https://api.devnet.solana.com
    )
    .option(
      "-k, --keypair <string>",
      "Solana wallet Keypair Path",
      "./keys/EgBcC7KVQTh1QeU3qxCFsnwZKYMMQkv6TzgEDkKvSNLv.json" //EgBcC7KVQTh1QeU3qxCFsnwZKYMMQkv6TzgEDkKvSNLv //EcagE8oN5WLAbEUBmALRqRA7H5auvRLbt8ve8Nf3atX4 //JCeJCM1YUEuU24VxmxaJPDhEneRS64Lg2PExYXD7VW4g
    );
}


program.parse(process.argv);

/*
yarn script config
yarn script market
yarn script swap -y 6fmXZMANN52hYjsGED7cznpDVjndFoTHWNzzZEwx1bQ6 -n 49mewVqkN2FNmH8byFyKg3kU6oCPbstx4AsuwiGfoYyx -a 2000000  -s 0 -t 1
yarn script claim -y 6fmXZMANN52hYjsGED7cznpDVjndFoTHWNzzZEwx1bQ6 -n 49mewVqkN2FNmH8byFyKg3kU6oCPbstx4AsuwiGfoYyx
yarn script whitelist -t AgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADyv0IdEUkGAucNYlIjTgUL5UHbvinOJhZBsbC4V1JxwMVi7NnNi8aqjbKCTNdnEnNgleg3CvtLZND/LUlRfhANgAIAAgb/kjzngt7EFB3AKRkcvTlCkEV2/Fn6286MDzp9ZM0mKcsxkx66WFzMhUvbNwtLS1HWysQLaXTH5Bs/bUZy+5GvTOOdEF4VsTaVN3/ZnVmRBxMAwiBEnuDbriutr9vFzbRT/P/ikQn86857GD1q1+qPtoT9ZOrk2UavJ47Mlp/i/quCFRiH2jWZPUdbl8IYrlKd67l9jeQiiKTaZvbNRj0iAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC3FZeBXwzit5o+d7hnFMZisNn/5rVAtQ7y278V/YuqiwEEBQIDAQAFKL+EdndxUv+K/5I854LexBQdwCkZHL05QpBFdvxZ+tvOjA86fWTNJikA

yarn script market2 -t BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABO0/dCPGhKCAtbhxU6eMlqb+KZoSTpbGK+ZvsCPCyK0RMVfxcBtTEP2w/A7udeIel5OQODR7WXsGHR1QXWPa0CVhR7NOCsGs2X9M+Slke2SWIgkjHpPb65NrvyXuyWtwERMWXtXau0Zv1tflwEGRCv1OQH/gACDoqDzSgnqWMFAY2gmdevdmFlV8I75nEJomXQ/q87ZGvfB7E/52SoBXv7UXnhbDdFxCr6dicqZkv/aQziIZB0VbxGtH2iLwRbVg2ABAAGEspFVhdnP/85kMR2NKYdaL6/KoOl2oEzuwjqAqOhBnvDRS3eX7dghQsUsYshTaPGril9JLqSLhTfXyu7JHEj8hnJbo31jAwSvGN93/6N3UDywjLh1S26KtWmDeYjqILKhMsxkx66WFzMhUvbNwtLS1HWysQLaXTH5Bs/bUZy+5GvfAiFbdXvX4EHZdPL4IVXeembnKNvyODu1hDNZD7jyOZA2hcHvHXMCB+3142YCKHGHOITPqVHNpbIix+Zy3RddSkzoRvsfA3ywblUj6r9y6VRwazXv2+hoR36lxkiLiCOaTKOn7AnzV+fWFCniy+tVutl8LZb14Qk4LRc5KJZxFonnFPuVXINNi9S/X/AXouEy1e3b6fkMd0VjdJq01Zbjf0cPpuBwwWJ2NaFUKR11NjSnzhR7KHAomnoYKvRFM6/TETv1mE5wPnd2rOKjyvzW3AH03GJJat+H9T8USAeKPILsK9lWPk6njNjT3BvXt1+DTmep9lRRaEIY9wADbVDHi+mgnffIU5xcMplaIVtgK0BQUwvtqa6CLbAtOFHnxMBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAGp9UXGSxcUSGMyUw9SvF/WNruCJuh/UTj29mKAAAAAAbd9uHXZaGT2cvhRs7reawctIXtX1s3kTqM9YV+/wCpjJclj04kifG7PRApFI4NgwtaE5na/xCEBI572Nvp+FkLcGWx49F8RTidUn9rBMPNWLhscxqg/bVJttG8A/gpRkMxOxo8qQTg9TSGnvjOTk80whFRSWuTYQynYfyUFI36AgwLBAUAAQYHDQ4PEBFdxqHQvHpF7IAIAAAAZGlzYWdyZWVFAAAAaHR0cHM6Ly9nYXRld2F5LmlyeXMueHl6L0FRdEJoVnNhNWg2b2oyWG5Cb0VadTZ4c2NScHpBWVVLRHJSQVZudTNnSzZFDBAEBQgAAgEJCgYLAw0ODxARhQHqkMAw/Cab0gUAAABhZ3JlZUUAAABodHRwczovL2dhdGV3YXkuaXJ5cy54eXovR3dLdVRwNnhIOEZrdFpjTGZGOVVrN2tQWlg1aU1FNURzclBVMm5WZTZuV03LMZMeulhczIVL2zcLS0tR1srEC2l0x+QbP21GcvuRrwcAAAA1NjQ5NjQ1AA==
yarn script swap2 -y 9do8t1niJgDvPbtACgSpjMDcq1Tb66Dwcv68a3PV6kEY -n GF9KyyTWGaPn2pNScn13WyWHBe8DVRTC2G4py9Fwo181 -a 2000000  -s 0 -t 1 -m 5649645
yarn script changeCreator -t AfS2c0Y9vQ1FlVDNYY1y3E5blxCo0RHQJjuHy47CJH23ldIODRb2vQunklaefqPuk7KZVTZs0lcekTQMB+Gp1g2AAQADBssxkx66WFzMhUvbNwtLS1HWysQLaXTH5Bs/bUZy+5GvfAiFbdXvX4EHZdPL4IVXeembnKNvyODu1hDNZD7jyOb9HD6bgcMFidjWhVCkddTY0p84UeyhwKJp6GCr0RTOvy+mgnffIU5xcMplaIVtgK0BQUwvtqa6CLbAtOFHnxMByW6N9YwMErxjfd/+jd1A8sIy4dUtuirVpg3mI6iCyoRFLd5ft2CFCxSxiyFNo8auKX0kupIuFN9fK7skcSPyGe33fLijz6kBDiKLmxXWQ69nWDo+4HAG4Ff4ukOTcpHzAQMGAQAABAUCKHrdRVpQ6m80yzGTHrpYXMyFS9s3C0tLUdbKxAtpdMfkGz9tRnL7ka8A
yarn script claim2 -y 9do8t1niJgDvPbtACgSpjMDcq1Tb66Dwcv68a3PV6kEY -n GF9KyyTWGaPn2pNScn13WyWHBe8DVRTC2G4py9Fwo181 -m 5649645
*/