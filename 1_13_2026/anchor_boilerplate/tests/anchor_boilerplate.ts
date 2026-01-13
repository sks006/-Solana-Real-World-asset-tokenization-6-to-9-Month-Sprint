import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorBoilerplate } from "../target/types/anchor_boilerplate";
import { 
  createMint, 
  getOrCreateAssociatedTokenAccount, 
  mintTo, 
  TOKEN_PROGRAM_ID,
  getAccount
} from "@solana/spl-token";
import { expect } from "chai";

describe("Lending Protocol: Week 6 Genesis", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorBoilerplate as Program<AnchorBoilerplate>;
  const admin = anchor.web3.Keypair.generate();
  const user1 = anchor.web3.Keypair.generate();
  
  let mint: anchor.web3.PublicKey;
  let adminTokenAccount: anchor.web3.PublicKey;
  let user1TokenAccount: anchor.web3.PublicKey;
  let vaultPda: anchor.web3.PublicKey;

  before(async () => {
    console.log("🚀 === SETTING UP TEST ENVIRONMENT ===");
    
    // 1. FUND ADMIN
    console.log("💰 Airdropping SOL to admin...");
    const adminAirdropSig = await provider.connection.requestAirdrop(
      admin.publicKey, 
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(adminAirdropSig);
    console.log(`✅ Admin funded: ${admin.publicKey.toString()}`);

    // 2. FUND USER1
    console.log("💰 Airdropping SOL to user1...");
    const userAirdropSig = await provider.connection.requestAirdrop(
      user1.publicKey, 
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(userAirdropSig);
    console.log(`✅ User1 funded: ${user1.publicKey.toString()}`);

    // 3. CREATE MINT
    console.log("🏦 Creating USDC-like mint...");
    mint = await createMint(
      provider.connection, 
      admin, 
      admin.publicKey, 
      null, 
      6 // 6 decimals like USDC
    );
    console.log(`✅ Mint created: ${mint.toString()}`);

    // 4. DERIVE VAULT PDA
    [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pool_vault"), mint.toBuffer()],
      program.programId
    );
    console.log(`🔐 Vault PDA derived: ${vaultPda.toString()}`);

    // 5. SETUP ADMIN TOKEN ACCOUNT
    console.log("👤 Setting up admin token account...");
    const adminAta = await getOrCreateAssociatedTokenAccount(
      provider.connection, 
      admin, 
      mint, 
      admin.publicKey
    );
    adminTokenAccount = adminAta.address;
    console.log(`✅ Admin ATA: ${adminTokenAccount.toString()}`);

    // 6. MINT TOKENS TO ADMIN
    console.log("🪙 Minting 1000 USDC to admin...");
    await mintTo(
      provider.connection, 
      admin, 
      mint, 
      adminTokenAccount, 
      admin, 
      1000_000_000 // 1000 tokens with 6 decimals
    );
    
    const adminBalance = await getAccount(provider.connection, adminTokenAccount);
    console.log(`📊 Admin balance: ${Number(adminBalance.amount) / 1_000_000} USDC`);

    // 7. SETUP USER1 TOKEN ACCOUNT
    console.log("👤 Setting up user1 token account...");
    const user1Ata = await getOrCreateAssociatedTokenAccount(
      provider.connection, 
      user1, 
      mint, 
      user1.publicKey
    );
    user1TokenAccount = user1Ata.address;
    console.log(`✅ User1 ATA: ${user1TokenAccount.toString()}`);

    // 8. TRANSFER TOKENS TO USER1
    console.log("🔄 Transferring 500 USDC to user1...");
    await mintTo(
      provider.connection,
      admin,
      mint,
      user1TokenAccount,
      admin,
      500_000_000 // 500 tokens
    );
    
    const user1Balance = await getAccount(provider.connection, user1TokenAccount);
    console.log(`📊 User1 balance: ${Number(user1Balance.amount) / 1_000_000} USDC`);
    
    console.log("✅ === TEST ENVIRONMENT SETUP COMPLETE ===\n");
  });

  describe("📦 VAULT INITIALIZATION", () => {
    it("Initializes the Global Vault", async () => {
      console.log("🧪 Test: Initialize global vault");
      
      const vaultBefore = await provider.connection.getAccountInfo(vaultPda);
      console.log(`📊 Vault exists before init: ${vaultBefore !== null}`);
      
      console.log("🔄 Executing initialize transaction...");
      const tx = await program.methods
        .initialize()
        .accounts({
          admin: admin.publicKey,
          mint: mint,
          vault: vaultPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([admin])
        .rpc();

      console.log(`✅ Transaction successful: ${tx}`);
      console.log(`🔗 Explorer: https://explorer.solana.com/tx/${tx}?cluster=devnet`);

      const vaultAfter = await provider.connection.getAccountInfo(vaultPda);
      console.log(`📊 Vault exists after init: ${vaultAfter !== null}`);
      console.log(`📊 Vault owner: ${vaultAfter?.owner.toString()}`);
      console.log(`📊 Vault data length: ${vaultAfter?.data.length} bytes`);

      expect(vaultAfter).to.not.be.null;
      expect(vaultAfter?.owner.toString()).to.equal(TOKEN_PROGRAM_ID.toString());
      
      console.log("✅ Test passed: Vault initialized successfully\n");
    });

    it("Fails to initialize vault twice", async () => {
      console.log("🧪 Test: Attempt duplicate initialization (should fail)");
      
      try {
        console.log("🔄 Attempting duplicate initialize...");
        await program.methods
          .initialize()
          .accounts({
            admin: admin.publicKey,
            mint: mint,
            vault: vaultPda,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          })
          .signers([admin])
          .rpc();
        
        console.log("❌ ERROR: Should have thrown but didn't!");
        expect.fail("Should have thrown an error");
      } catch (error: any) {
        console.log(`✅ Expected error caught: ${error.message}`);
        expect(error.message).to.include("already in use");
        console.log("✅ Test passed: Duplicate initialization prevented\n");
      }
    });
  });

  describe("💰 DEPOSIT OPERATIONS", () => {
    it("Executes a secure deposit from admin", async () => {
      console.log("🧪 Test: Admin deposit 300 USDC");
      
      const [obligationPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("obligation"), admin.publicKey.toBuffer()],
        program.programId
      );
      console.log(`🔐 Admin obligation PDA: ${obligationPda.toString()}`);

      const depositAmount = new anchor.BN(300_000_000); // 300 USDC
      console.log(`💰 Deposit amount: ${depositAmount.toNumber() / 1_000_000} USDC`);

      // Get balances before
      const adminBalanceBefore = await getAccount(provider.connection, adminTokenAccount);
      const vaultBalanceBefore = await getAccount(provider.connection, vaultPda);
      console.log(`📊 Admin balance before: ${Number(adminBalanceBefore.amount) / 1_000_000} USDC`);
      console.log(`📊 Vault balance before: ${Number(vaultBalanceBefore.amount) / 1_000_000} USDC`);

      console.log("🔄 Executing deposit transaction...");
      const tx = await program.methods
        .deposit(depositAmount)
        .accounts({
          user: admin.publicKey,
          mint: mint,
          userTokenAccount: adminTokenAccount,
          vault: vaultPda,
          obligation: obligationPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

      console.log(`✅ Transaction successful: ${tx}`);

      // Verify state
      const state = await program.account.userObligation.fetch(obligationPda);
      console.log(`📊 Obligation deposited: ${state.deposited.toNumber() / 1_000_000} USDC`);
      console.log(`📊 Obligation borrowed: ${state.borrowed.toNumber() / 1_000_000} USDC`);
      console.log(`📊 Obligation owner: ${state.owner.toString()}`);

      // Verify token transfers
      const adminBalanceAfter = await getAccount(provider.connection, adminTokenAccount);
      const vaultBalanceAfter = await getAccount(provider.connection, vaultPda);
      console.log(`📊 Admin balance after: ${Number(adminBalanceAfter.amount) / 1_000_000} USDC`);
      console.log(`📊 Vault balance after: ${Number(vaultBalanceAfter.amount) / 1_000_000} USDC`);

      // Assertions
      expect(state.deposited.toNumber()).to.equal(depositAmount.toNumber());
      expect(Number(adminBalanceAfter.amount)).to.equal(
        Number(adminBalanceBefore.amount) - depositAmount.toNumber()
      );
      expect(Number(vaultBalanceAfter.amount)).to.equal(
        Number(vaultBalanceBefore.amount) + depositAmount.toNumber()
      );
      
      console.log("✅ Test passed: Admin deposit successful\n");
    });

    it("Executes a deposit from another user", async () => {
      console.log("🧪 Test: User1 deposit 200 USDC");
      
      const [obligationPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("obligation"), user1.publicKey.toBuffer()],
        program.programId
      );
      console.log(`🔐 User1 obligation PDA: ${obligationPda.toString()}`);

      const depositAmount = new anchor.BN(200_000_000); // 200 USDC
      console.log(`💰 Deposit amount: ${depositAmount.toNumber() / 1_000_000} USDC`);

      console.log("🔄 Executing user1 deposit...");
      const tx = await program.methods
        .deposit(depositAmount)
        .accounts({
          user: user1.publicKey,
          mint: mint,
          userTokenAccount: user1TokenAccount,
          vault: vaultPda,
          obligation: obligationPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      console.log(`✅ Transaction successful: ${tx}`);

      const state = await program.account.userObligation.fetch(obligationPda);
      console.log(`📊 User1 obligation deposited: ${state.deposited.toNumber() / 1_000_000} USDC`);
      console.log(`📊 User1 obligation owner: ${state.owner.toString()}`);

      expect(state.deposited.toNumber()).to.equal(depositAmount.toNumber());
      expect(state.owner.toString()).to.equal(user1.publicKey.toString());
      
      console.log("✅ Test passed: User1 deposit successful\n");
    });

    it("Allows multiple deposits from same user", async () => {
      console.log("🧪 Test: Multiple deposits from admin");
      
      const [obligationPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("obligation"), admin.publicKey.toBuffer()],
        program.programId
      );

      const firstDeposit = new anchor.BN(100_000_000); // 100 USDC
      const secondDeposit = new anchor.BN(50_000_000); // 50 USDC

      console.log(`💰 First deposit: ${firstDeposit.toNumber() / 1_000_000} USDC`);
      console.log(`💰 Second deposit: ${secondDeposit.toNumber() / 1_000_000} USDC`);

      // Get initial state
      const initialState = await program.account.userObligation.fetch(obligationPda);
      console.log(`📊 Initial deposited: ${initialState.deposited.toNumber() / 1_000_000} USDC`);

      // First deposit
      console.log("🔄 Executing first deposit...");
      await program.methods
        .deposit(firstDeposit)
        .accounts({
          user: admin.publicKey,
          mint: mint,
          userTokenAccount: adminTokenAccount,
          vault: vaultPda,
          obligation: obligationPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

      const stateAfterFirst = await program.account.userObligation.fetch(obligationPda);
      console.log(`📊 After first deposit: ${stateAfterFirst.deposited.toNumber() / 1_000_000} USDC`);

      // Second deposit
      console.log("🔄 Executing second deposit...");
      await program.methods
        .deposit(secondDeposit)
        .accounts({
          user: admin.publicKey,
          mint: mint,
          userTokenAccount: adminTokenAccount,
          vault: vaultPda,
          obligation: obligationPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

      const finalState = await program.account.userObligation.fetch(obligationPda);
      console.log(`📊 After second deposit: ${finalState.deposited.toNumber() / 1_000_000} USDC`);

      const expectedTotal = initialState.deposited.toNumber() + firstDeposit.toNumber() + secondDeposit.toNumber();
      console.log(`📊 Expected total: ${expectedTotal / 1_000_000} USDC`);

      expect(finalState.deposited.toNumber()).to.equal(expectedTotal);
      console.log("✅ Test passed: Multiple deposits successful\n");
    });
  });

  describe("🏦 BORROW OPERATIONS", () => {
    it("Allows borrowing within collateral limits (80% rule)", async () => {
      console.log("🧪 Test: Borrow within 80% collateral limit");
      
      const [obligationPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("obligation"), admin.publicKey.toBuffer()],
        program.programId
      );

      // Get current deposited amount
      const stateBefore = await program.account.userObligation.fetch(obligationPda);
      const deposited = stateBefore.deposited.toNumber();
      const maxBorrow = Math.floor(deposited * 0.8); // 80% of collateral
      const borrowAmount = new anchor.BN(maxBorrow);

      console.log(`📊 Current deposited: ${deposited / 1_000_000} USDC`);
      console.log(`📊 Max borrow (80%): ${maxBorrow / 1_000_000} USDC`);
      console.log(`💰 Borrowing: ${borrowAmount.toNumber() / 1_000_000} USDC`);

      // Get balances before
      const vaultBalanceBefore = await getAccount(provider.connection, vaultPda);
      const adminBalanceBefore = await getAccount(provider.connection, adminTokenAccount);
      console.log(`📊 Vault balance before: ${Number(vaultBalanceBefore.amount) / 1_000_000} USDC`);
      console.log(`📊 Admin balance before: ${Number(adminBalanceBefore.amount) / 1_000_000} USDC`);

      console.log("🔄 Executing borrow transaction...");
      const tx = await program.methods
        .borrow(borrowAmount)
        .accounts({
          user: admin.publicKey,
          userTokenAccount: adminTokenAccount,
          vault: vaultPda,
          obligation: obligationPda,
          mint: mint,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([admin])
        .rpc();

      console.log(`✅ Transaction successful: ${tx}`);

      // Verify obligation updated
      const stateAfter = await program.account.userObligation.fetch(obligationPda);
      console.log(`📊 New borrowed amount: ${stateAfter.borrowed.toNumber() / 1_000_000} USDC`);
      console.log(`📊 Health factor: ${(deposited * 0.8 / (stateAfter.borrowed.toNumber() || 1)).toFixed(2)}x`);

      // Verify token transfers
      const vaultBalanceAfter = await getAccount(provider.connection, vaultPda);
      const adminBalanceAfter = await getAccount(provider.connection, adminTokenAccount);
      console.log(`📊 Vault balance after: ${Number(vaultBalanceAfter.amount) / 1_000_000} USDC`);
      console.log(`📊 Admin balance after: ${Number(adminBalanceAfter.amount) / 1_000_000} USDC`);

      expect(stateAfter.borrowed.toNumber()).to.equal(borrowAmount.toNumber());
      console.log("✅ Test passed: Borrow within limits successful\n");
    });

    it("Prevents borrowing beyond collateral limit", async () => {
      console.log("🧪 Test: Attempt to borrow beyond 80% limit (should fail)");
      
      const [obligationPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("obligation"), admin.publicKey.toBuffer()],
        program.programId
      );

      const state = await program.account.userObligation.fetch(obligationPda);
      const deposited = state.deposited.toNumber();
      const alreadyBorrowed = state.borrowed.toNumber();
      const remainingCollateral = deposited - alreadyBorrowed;
      const excessiveBorrow = new anchor.BN(Math.floor(remainingCollateral * 0.9)); // Try 90%

      console.log(`📊 Current deposited: ${deposited / 1_000_000} USDC`);
      console.log(`📊 Already borrowed: ${alreadyBorrowed / 1_000_000} USDC`);
      console.log(`📊 Remaining collateral: ${remainingCollateral / 1_000_000} USDC`);
      console.log(`💰 Attempting to borrow: ${excessiveBorrow.toNumber() / 1_000_000} USDC`);
      console.log(`⚠️  This exceeds the 80% limit!`);

      try {
        console.log("🔄 Attempting excessive borrow...");
        await program.methods
          .borrow(excessiveBorrow)
          .accounts({
            user: admin.publicKey,
            userTokenAccount: adminTokenAccount,
            vault: vaultPda,
            obligation: obligationPda,
            mint: mint,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([admin])
          .rpc();
        
        console.log("❌ ERROR: Should have thrown but didn't!");
        expect.fail("Should have thrown insufficient collateral error");
      } catch (error: any) {
        console.log(`✅ Expected error caught: ${error.message}`);
        expect(error.message).to.include("Insufficient collateral");
        console.log("✅ Test passed: Excessive borrow prevented\n");
      }
    });

    it("Prevents unauthorized user from borrowing", async () => {
      console.log("🧪 Test: User1 tries to borrow from admin's obligation (should fail)");
      
      const [adminObligationPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("obligation"), admin.publicKey.toBuffer()],
        program.programId
      );

      const borrowAmount = new anchor.BN(100_000_000); // 100 USDC

      console.log(`💰 Attempting to borrow: ${borrowAmount.toNumber() / 1_000_000} USDC`);
      console.log(`👤 User1: ${user1.publicKey.toString()}`);
      console.log(`🔐 Trying to access admin's obligation: ${adminObligationPda.toString()}`);
      console.log(`⚠️  This should fail - user doesn't own this obligation!`);

      try {
        console.log("🔄 Attempting unauthorized borrow...");
        await program.methods
          .borrow(borrowAmount)
          .accounts({
            user: user1.publicKey,
            userTokenAccount: user1TokenAccount,
            vault: vaultPda,
            obligation: adminObligationPda,
            mint: mint,
            tokenProgram: TOKEN_PROGRAM_ID,
            
          })
          .signers([user1])
          .rpc();
        
        console.log("❌ ERROR: Should have thrown but didn't!");
        expect.fail("Should have thrown unauthorized error");
      } catch (error: any) {
        console.log(`✅ Expected error caught: ${error.message}`);
        expect(error.message).to.include("Unauthorized");
        console.log("✅ Test passed: Unauthorized borrow prevented\n");
      }
    });
  });

  describe("🎯 EDGE CASES", () => {
    it("Handles zero amount deposit", async () => {
      console.log("🧪 Test: Deposit zero amount (should be allowed)");
      
      const [obligationPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("obligation"), user1.publicKey.toBuffer()],
        program.programId
      );

      const zeroAmount = new anchor.BN(0);
      console.log(`💰 Deposit amount: ${zeroAmount.toNumber()} USDC`);

      const stateBefore = await program.account.userObligation.fetch(obligationPda);
      console.log(`📊 Balance before: ${stateBefore.deposited.toNumber() / 1_000_000} USDC`);

      console.log("🔄 Executing zero deposit...");
      await program.methods
        .deposit(zeroAmount)
        .accounts({
          user: user1.publicKey,
          mint: mint,
          userTokenAccount: user1TokenAccount,
          vault: vaultPda,
          obligation: obligationPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      const stateAfter = await program.account.userObligation.fetch(obligationPda);
      console.log(`📊 Balance after: ${stateAfter.deposited.toNumber() / 1_000_000} USDC`);

      expect(stateAfter.deposited.toNumber()).to.equal(stateBefore.deposited.toNumber());
      console.log("✅ Test passed: Zero amount deposit handled correctly\n");
    });
  });

  after(async () => {
    console.log("📊 === FINAL BALANCES ===");
    
    const vaultBalance = await getAccount(provider.connection, vaultPda);
    const adminBalance = await getAccount(provider.connection, adminTokenAccount);
    const user1Balance = await getAccount(provider.connection, user1TokenAccount);
    
    console.log(`🏦 Vault balance: ${Number(vaultBalance.amount) / 1_000_000} USDC`);
    console.log(`👤 Admin balance: ${Number(adminBalance.amount) / 1_000_000} USDC`);
    console.log(`👤 User1 balance: ${Number(user1Balance.amount) / 1_000_000} USDC`);
    
    const [adminObligationPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("obligation"), admin.publicKey.toBuffer()],
      program.programId
    );
    const [user1ObligationPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("obligation"), user1.publicKey.toBuffer()],
      program.programId
    );
    
    try {
      const adminObligation = await program.account.userObligation.fetch(adminObligationPda);
      console.log(`📋 Admin obligation - Deposited: ${adminObligation.deposited.toNumber() / 1_000_000} USDC, Borrowed: ${adminObligation.borrowed.toNumber() / 1_000_000} USDC`);
    } catch (e) {
      console.log("📋 Admin obligation: Not found");
    }
    
    try {
      const user1Obligation = await program.account.userObligation.fetch(user1ObligationPda);
      console.log(`📋 User1 obligation - Deposited: ${user1Obligation.deposited.toNumber() / 1_000_000} USDC, Borrowed: ${user1Obligation.borrowed.toNumber() / 1_000_000} USDC`);
    } catch (e) {
      console.log("📋 User1 obligation: Not found");
    }
    
    console.log("✅ === ALL TESTS COMPLETED ===\n");
  });
});
