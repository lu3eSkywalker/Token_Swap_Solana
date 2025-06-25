import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import { CpiGuardLayout, getAssociatedTokenAddress } from "@solana/spl-token";
import { SimpleTokenSwap } from "../target/types/Simple_Token_Swap";
import { BN } from "bn.js";

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SimpleTokenSwap as anchor.Program<SimpleTokenSwap>;

  const tokenA_mint_address = new web3.PublicKey("4uUgYR1DMsWNq4UMx6coUFaouN4K6WybqpWyVDnDdG4z");
  const tokenB_mint_address = new web3.PublicKey("J7zEQiEnBuuUubVgX2siPWQWwJ9EqoiesVmmHazfX9oj");

  it("initializes a Vault Account For Token A", async () => {
    const [vault_token_account, bump1] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultTokenA"), tokenA_mint_address.toBuffer()],
      program.programId
    )

    const [vaultPDA, bump2] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultTokenA"), tokenA_mint_address.toBuffer()],
      program.programId
    );

    console.log("This is the Token vault_token_account for Token A: ", vault_token_account.toString());
    console.log("This is the vaultPDA for Token B: ", vaultPDA.toString());

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

  it("initializes a Vault Account For Token B", async() => {
    const [vault_token_account, bump1] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultTokenB"), tokenB_mint_address.toBuffer()],
      program.programId
    )

    const [vaultPDA, bump2] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultTokenB"), tokenB_mint_address.toBuffer()],
      program.programId
    );

    console.log("This is the Token vault_token_account for Token B: ", vault_token_account.toString());
    console.log("This is the vaultPDA for Token B: ", vaultPDA.toString());

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
  });

  it("deposit Token A to vault", async () => {

    const [vault_token_account, bump1] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultTokenA"), tokenA_mint_address.toBuffer()],
      program.programId
    )

    const amount_to_deposit = new BN(10_000_000_000);

    const userATA = new web3.PublicKey("GzsqFZFN8D3xhqQWDxHXwcT86BjbDWzf9ykXxPjda1EZ");

    // Send Transaction
    const txHash = await program.methods
    .depositToVaultTokenA(amount_to_deposit)
    .accounts({
      user: program.provider.publicKey,
      userTokenAccount: userATA,
      vaultTokenAccount: vault_token_account,
      mint: tokenA_mint_address,
      tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),      
    })
    .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm Transaction
    await program.provider.connection.confirmTransaction(txHash);
  });

  it("deposit Token B to vault", async () => {
    const [vault_token_account, bump1] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultTokenB"), tokenB_mint_address.toBuffer()],
      program.programId
    )

    const amount_to_deposit = new BN(10_000_000_000);

    const userATA = new web3.PublicKey("GwQyCotM95ntoG9i5Gw2coFZyyMgLtqXudLXXYKtiUEr");

    // Send Transaction
    const txHash = await program.methods
    .depositToVaultTokenB(amount_to_deposit)
    .accounts({
      user: program.provider.publicKey,
      userTokenAccount: userATA,
      vaultTokenAccount: vault_token_account,
      mint: tokenB_mint_address,
      tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    })
    .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm Transaction
    await program.provider.connection.confirmTransaction(txHash);
  })
});