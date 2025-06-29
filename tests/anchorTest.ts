import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import { CpiGuardLayout, createAssociatedTokenAccountInstruction, getAccount, getAssociatedTokenAddress } from "@solana/spl-token";
import { SimpleTokenSwap } from "../target/types/Simple_Token_Swap";
import { BN } from "bn.js";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SimpleTokenSwap as anchor.Program<SimpleTokenSwap>;

  const tokenA_mint_address = new web3.PublicKey("6brEek47QhmqAxAuqBBnRMjshVM4XphbxFLjdVNE3uiM");
  const tokenB_mint_address = new web3.PublicKey("3vwLsA3XrM6Kqg1v6ACF4qYoZUQTM54i1atCFTPEKZG5");

  const [vault_token_account_a, bump1] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vaultTokenA"), tokenA_mint_address.toBuffer()],
    program.programId
  );

  const [vault_token_account_b, bump2] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vaultTokenB"), tokenB_mint_address.toBuffer()],
    program.programId
  );

  const [vault_auth_a, bump3] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vaultTokenA"), tokenA_mint_address.toBuffer()],
    program.programId
  );

  const [vault_auth_b, bump4] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vaultTokenB"), tokenB_mint_address.toBuffer()],
    program.programId
  );

  const [userPDALiquidity, bump5] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("liquidityPDA"), program.provider.publicKey.toBuffer()],
    program.programId
  );

  it("initializes a Vault Account For Token A", async () => {
    const [vault_token_account, bump1] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultTokenA"), tokenA_mint_address.toBuffer()],
      program.programId
    );

    const [vaultPDA, bump2] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultTokenA"), tokenA_mint_address.toBuffer()],
      program.programId
    );

    console.log("This is the Token vault_token_account for Token A: ", vault_token_account.toString());
    console.log("This is the vaultPDA for Token A: ", vaultPDA.toString());

    // Send Transaction
    const txHash = await program.methods
      .initializeVaultTokenA()
      .accounts({
        vaultTokenAccount: vault_token_account,
        vault_auth: vaultPDA,
        payer: program.provider.publicKey,
        mint: tokenA_mint_address,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm Transaction
    await program.provider.connection.confirmTransaction(txHash);
  });

  it("initializes a Vault Account For Token B", async () => {
    const [vault_token_account, bump1] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultTokenB"), tokenB_mint_address.toBuffer()],
      program.programId
    )

    const [vaultPDA, bump2] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultTokenB"), tokenB_mint_address.toBuffer()],
      program.programId
    );

    // Send Transaction
    const txHash = await program.methods
      .initializeVaultTokenB()
      .accounts({
        vaultTokenAccount: vault_token_account,
        vault_auth: vaultPDA,
        payer: program.provider.publicKey,
        mint: tokenB_mint_address,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm Transaction
    await program.provider.connection.confirmTransaction(txHash);

    const pda_token_value = await getAccount(program.provider.connection, vault_token_account);
    console.log("Vault Token B Account Balance: ", pda_token_value.amount.toString());
  });

  it("initializes a liquidity account", async () => {

    const [userPDALiquidity, bump] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("liquidityPDA"), program.provider.publicKey.toBuffer()],
      program.programId
    );

    const accountInfo = await program.provider.connection.getAccountInfo(userPDALiquidity);

    if (accountInfo) {
      console.log("User Liquidity account is already initialized");
      return;
    }

    const txHash = await program.methods
      .initializeUserLiquidityAccount()
      .accounts({
        user: program.provider.publicKey,
        userPdaAccount: userPDALiquidity,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm Transaction
    await program.provider.connection.confirmTransaction(txHash);
  });

  it("adds liquidity to the liquidity pool", async () => {

    // Deriving user Token ATA
    const user_token_a_ata = await getAssociatedTokenAddress(
      tokenA_mint_address,
      program.provider.publicKey
    );

    const user_token_b_ata = await getAssociatedTokenAddress(
      tokenB_mint_address,
      program.provider.publicKey
    );

    const token_amount = new BN(10_000_000_000);

    const txHash = await program.methods
      .addLiquidity(token_amount)
      .accounts({
        user: program.provider.publicKey,
        userPdaAccount: userPDALiquidity,
        userTokenAccountForTokenA: user_token_a_ata,
        userTokenAccountForTokenB: user_token_b_ata,
        vaultTokenAAccount: vault_token_account_a,
        vaultTokenBAccount: vault_token_account_b,
        vaultAuthA: vault_auth_a,
        vaultAuthB: vault_auth_b,
        mintA: tokenA_mint_address,
        mintB: tokenB_mint_address,
        tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm Transaction
    await program.provider.connection.confirmTransaction(txHash);
  });

  it("removes liquidity from the liquidity pool", async () => {

    const user_token_a_ata = await getAssociatedTokenAddress(
      tokenA_mint_address,
      program.provider.publicKey
    );

    const user_token_b_ata = await getAssociatedTokenAddress(
      tokenB_mint_address,
      program.provider.publicKey
    );

    const token_amount = new BN(1_000_000_000);

    const txHash = await program.methods
      .removeLiquidity(token_amount)
      .accounts({
        user: program.provider.publicKey,
        userPdaAccount: userPDALiquidity,
        userTokenAccountForTokenA: user_token_a_ata,
        userTokenAccountForTokenB: user_token_b_ata,
        vaultTokenAAccount: vault_token_account_a,
        vaultTokenBAccount: vault_token_account_b,
        vaultAuthA: vault_auth_a,
        vaultAuthB: vault_auth_b,
        mintA: tokenA_mint_address,
        mintB: tokenB_mint_address,
        tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm Transaction
    await program.provider.connection.confirmTransaction(txHash);
  })

  it("Swap Token B for Token A", async () => {

    const userATAforTokenA = new web3.PublicKey("Bd1Ho7Y9PsZ1zK6YpXnHGmzSoygPVM9FhYMuj85ytEUg");
    const userATAforTokenB = new web3.PublicKey("7TWc3HMxNi2FH33BWzdMeexN93C8DLXdDBtfKgXuXEQc");

    const amount = new BN(10_000_000_000);

    const txHash = await program.methods
      .swapBForA(amount)
      .accounts({
        user: program.provider.publicKey,
        userTokenAccountForTokenA: userATAforTokenA,
        userTokenAccountForTokenB: userATAforTokenB,
        vaultTokenAAccount: vault_token_account_a,
        vaultTokenBAccount: vault_token_account_b,
        vaultAuthA: vault_auth_a,
        vaultAuthB: vault_auth_b,
        mintA: tokenA_mint_address,
        mintB: tokenB_mint_address,
        tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm Transaction
    await program.provider.connection.confirmTransaction(txHash);
  })

  it("Swap Token A for Token B", async () => {

    const userATAforTokenA = new web3.PublicKey("Bd1Ho7Y9PsZ1zK6YpXnHGmzSoygPVM9FhYMuj85ytEUg");
    const userATAforTokenB = new web3.PublicKey("7TWc3HMxNi2FH33BWzdMeexN93C8DLXdDBtfKgXuXEQc");

    const amount = new BN(10_000_000_000);

    const txHash = await program.methods
      .swapAForB(amount)
      .accounts({
        user: program.provider.publicKey,
        userTokenAccountForTokenA: userATAforTokenA,
        userTokenAccountForTokenB: userATAforTokenB,
        vaultTokenAAccount: vault_token_account_a,
        vaultTokenBAccount: vault_token_account_b,
        vaultAuthA: vault_auth_a,
        vaultAuthB: vault_auth_b,
        mintA: tokenA_mint_address,
        mintB: tokenB_mint_address,
        tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm Transaction
    await program.provider.connection.confirmTransaction(txHash);
  });
});