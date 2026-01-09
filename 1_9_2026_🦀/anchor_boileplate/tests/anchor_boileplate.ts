import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorBoilerplate } from "../target/types/anchor_boilerplate";
import { 
  createMint, 
  getOrCreateAssociatedTokenAccount, 
  mintTo 
} from "@solana/spl-token";
import { Keypair, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

describe("Lending Protocol Security Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorBoilerplate as Program<AnchorBoilerplate>;
  const user = anchor.web3.Keypair.generate();
  
  let mint: PublicKey;
  let userTokenAccount: PublicKey;
  let poolVault: PublicKey;
  let obligation: PublicKey;

  // RULE OF PREPARATION: The before() block is your "Genesis"
  before(async () => {
    // 1. Airdrop to user
    const signature = await provider.connection.requestAirdrop(user.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature);

    // 2. Create USDC Mint (Mock)
    mint = await createMint(provider.connection, user, user.publicKey, null, 6);

    // 3. Setup User Token Account with some balance
    const ata = await getOrCreateAssociatedTokenAccount(provider.connection, user, mint, user.publicKey);
    userTokenAccount = ata.address;
    await mintTo(provider.connection, user, mint, userTokenAccount, user, 1000_000_000); // 1000 tokens

    // 4. RULE: Deriving PDAs (Must match Rust b"pool_vault" and b"obligation")
    [poolVault] = anchor.web3.Pubkey.findProgramAddressSync(
      [Buffer.from("pool_vault"), program.programId.toBuffer()],
      program.programId
    );

    [obligation] = anchor.web3.Pubkey.findProgramAddressSync(
      [Buffer.from("obligation"), user.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Deposits collateral successfully", async () => {
    const depositAmount = new anchor.BN(500_000_000); // 500 tokens

    await program.methods
      .deposit(depositAmount)
      .accounts({
        // Note: Anchor 0.30+ might auto-fill some, but explicit is better for learning
        poolVault: poolVault,
        obligation: obligation,
        userTokenAccount: userTokenAccount,
        user: user.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    // Verification Rule
    const account = await program.account.userObligation.fetch(obligation);
    expect(account.depositedAmount.toNumber()).to.equal(depositAmount.toNumber());
  });
  it("Fails when borrowing beyond 80% LTV (Security Check)", async () => {
    const badBorrowAmount = new anchor.BN(450_000_000); // 90% of 500 deposit

    try {
      await program.methods
        .borrow(badBorrowAmount)
        .accounts({
          poolVault: poolVault,
          obligation: obligation,
          userTokenAccount: userTokenAccount,
          owner: user.publicKey,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();
      
      expect.fail("Protocol allowed over-collateralized borrow!");
    } catch (err) {
      // RULE: Check for your custom error code from Rust
      expect(err.toString()).to.include("InsufficientCollateral");
    }
  })
});