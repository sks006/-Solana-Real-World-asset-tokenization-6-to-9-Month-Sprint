// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// // This file is generated AFTER you run 'anchor build'
// import { AnchorBoilerplate } from "../target/types/anchor_boileplate";
// import { expect } from "chai";

// describe("Lending Drill", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   // RULE: Force the type cast to fix the "Property does not exist" error
//   const program = anchor.workspace.AnchorBoilerplate as Program<AnchorBoilerplate>;

//   it("Deposits collateral into a PDA vault", async () => {
//     // RULE: Precision Rule - Never use JS numbers for u64
//     const amount = new anchor.BN(5000000);

//     // Find the PDA (Mental Map of Solana)
//     const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("vault"), provider.wallet.publicKey.toBuffer()],
//       program.programId
//     );

//     // ACTION: The Handshake
//     await program.methods
//       .deposit(amount) // If this shows red, run 'anchor build' first
//       .accounts({
//         vaultAccount: vaultPDA,
//         user: provider.wallet.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .rpc();

//     // VERIFICATION
//     const vaultData = await program.account.userVault.fetch(vaultPDA);
//     expect(vaultData.collateral.toString()).to.equal(amount.toString());
//     console.log("Vault successfully updated with amount:", vaultData.collateral.toString());
//   });
// });

// ---------------------------- 15 ------------------------------

import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { AnchorBoileplate } from "../target/types/anchor_boileplate"
import { expect } from "chai"

describe("Lending Drill",()=>{
  const provider=anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const  program=anchor.workspace.AnchorBoileplate as Program<AnchorBoileplate>;
  it("Deposits collateral into a PDA vault",async()=>{
    const amount=new anchor.BN(5000000);
    const [vaultPDA]=anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    await program.methods.deposit(amount).accounts({
      vaultAccount:vaultPDA,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
      .rpc();
      const vaultData=await program.account.userVault.fetch(vaultPDA);
      expect(vaultData.collateral.toString());
      console.log("vault successfully update with amount:",vaultData.collateral.toString())
    })
  })
