// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { SolanaStablecoin } from "../target/types/solana_stablecoin";
// import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

// describe("solana-stablecoin", () => {
//   // Configure the client to use the local cluster.
//   const provider = anchor.AnchorProvider.env();
//   const connection = provider.connection;
//   const wallet = provider.wallet as anchor.Wallet;
//   anchor.setProvider(provider);

//   const program = anchor.workspace
//     .SolanaStablecoin as Program<SolanaStablecoin>;
//   const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });
//   const SOL_PRICE_FEED_ID =
//     "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
//   const solUsdPriceFeedAccount = pythSolanaReceiver.getPriceFeedAccountAddress(
//     0,
//     SOL_PRICE_FEED_ID
//   );
//   const [collateralAccount] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("collateral"), wallet.publicKey.toBuffer()],
//     program.programId
//   );

//   it("Is initialized!", async () => {
//     const transaction = await program.methods
//       .initializeConfig()
//       .accounts({})
//       .rpc({ skipPreflight: true, commitment: "confirmed" });
//     console.log("transaction signature:", transaction);
//   });

//   it("Deposit Collateral and Mint USDC", async () => {
//     const amountCollateral = 1_000_000_000;
//     const amountToMint = 1_000_000_000;

//     const transaction = await program.methods
//       .depositCollateralAndMintTokens(
//         new anchor.BN(amountCollateral),
//         new anchor.BN(amountToMint)
//       )
//       // where is the token program?
//       .accounts({ priceUpdate: solUsdPriceFeedAccount })
//       .rpc({ skipPreflight: true, commitment: "confirmed" });
//     console.log("transaction signature:", transaction);
//   });

//   it("Redeem collateral and Burn USDC", async () => {
//     const amountCollateral = 500_000_000;
//     const amountToBurn = 500_000_000;
//     const transaction = await program.methods
//       .redeemCollateralAndBurnTokens(
//         new anchor.BN(amountCollateral),
//         new anchor.BN(amountToBurn)
//       )
//       .accounts({ priceUpdate: solUsdPriceFeedAccount })
//       .rpc({ skipPreflight: true, commitment: "confirmed" });
//     console.log("transaction signature:", transaction);
//   });

//   it("Update Config", async () => {
//     const transaction = await program.methods
//       .updateConfig(new anchor.BN(100))
//       .accounts({})
//       .rpc({
//         skipPreflight: true,
//         commitment: "confirmed",
//       });
//     console.log("transaction signature:", transaction);
//   });

//   it("Liquidate", async () => {
//     const amountToBurn = 500_000_000;
//     const transaction = await program.methods
//       .liquidate(new anchor.BN(amountToBurn))
//       .accounts({ collateralAccount, priceUpdate: solUsdPriceFeedAccount })
//       .rpc({ skipPreflight: true, commitment: "confirmed" });
//     console.log("transaction signature:", transaction);
//   });

//   it("Update Config", async () => {
//     const transaction = await program.methods
//       .updateConfig(new anchor.BN(1))
//       .accounts({})
//       .rpc({
//         skipPreflight: true,
//         commitment: "confirmed",
//       });
//     console.log("transaction signature:", transaction);
//   });
// });

// gpt generated code also didn't work
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaStablecoin } from "../target/types/solana_stablecoin";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

describe("stablecoin", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace
    .SolanaStablecoin as Program<SolanaStablecoin>;

  /* ---------------- PYTH ---------------- */
  const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });
  const SOL_PRICE_FEED_ID =
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

  const priceUpdate = pythSolanaReceiver.getPriceFeedAccountAddress(
    0,
    SOL_PRICE_FEED_ID
  );

  /* ---------------- PDAs ---------------- */

  const [configAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );

  const [collateralAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("collateral"), wallet.publicKey.toBuffer()],
    program.programId
  );

  const [solAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("sol"), wallet.publicKey.toBuffer()],
    program.programId
  );

  const tokenAccount = getAssociatedTokenAddressSync(
    mintAccount,
    wallet.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  /* ---------------- TESTS ---------------- */

  it("Is initialized!", async () => {
    const tx = await program.methods.initializeConfig().accounts({}).rpc();

    console.log("Initialize tx:", tx);
  });

  it("Deposit Collateral and Mint USDS", async () => {
    const amountCollateral = 1_000_000_000;
    const amountToMint = 1_000_000_000;

    const tx = await program.methods
      .depositCollateralAndMintTokens(
        new anchor.BN(amountCollateral),
        new anchor.BN(amountToMint)
      )
      .accounts({
        depositor: wallet.publicKey,
        priceUpdate,
      })
      .rpc();

    console.log("Deposit + Mint tx:", tx);
  });

  it("Redeem Collateral and Burn USDS", async () => {
    const amountCollateral = 500_000_000;
    const amountToBurn = 500_000_000;

    const tx = await program.methods
      .redeemCollateralAndBurnTokens(
        new anchor.BN(amountCollateral),
        new anchor.BN(amountToBurn)
      )
      .accounts({
        depositor: wallet.publicKey,
        priceUpdate,
      })
      .rpc();

    console.log("Redeem + Burn tx:", tx);
  });

  it("Update Config (increase min health factor)", async () => {
    const tx = await program.methods
      .updateConfig(new anchor.BN(100))
      .accounts({
        configAccount,
      })
      .rpc();

    console.log("Update config tx:", tx);
  });

  it("Liquidate", async () => {
    const amountToBurn = 500_000_000;

    const tx = await program.methods
      .liquidate(new anchor.BN(amountToBurn))
      .accounts({
        liquidator: wallet.publicKey,
        priceUpdate,
        collateralAccount,
      })
      .rpc();

    console.log("Liquidate tx:", tx);
  });

  it("Update Config (restore min health factor)", async () => {
    const tx = await program.methods
      .updateConfig(new anchor.BN(1))
      .accounts({
        configAccount,
      })
      .rpc();

    console.log("Update config tx:", tx);
  });
});
