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
//------------------------------------------ 15 -----------------------------------

// import * as anchor from "@coral-xyz/anchor"
// import { Program } from "@coral-xyz/anchor"

// //this file will generated After you run anchor build
// import {AnchorBoilerplate} from "../target/type/anchor_boileplate";

// describe("Lending Drill", ()=>{
//   const provider=anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

// //rule : Force the type cast to fix the "Property does not exist" error

// const program= anchor.workspace.AnchorBoilerplate as Program<AnchorBoilerplate>;

// it("Deposits collateral into a PDA vault",async()=>{
//   //rule: precision rule Never use js numbers for u64
//   const amount=new anchor.BN(5000000);
//   //find pda (mental map of solana)
//   cost [vaultPDA] =anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("vault"),provider.wallet.publicKey.toBuffer()],
//     program.programId
//   );
//   //The handshake 
//   await  program.methods.deposit(amount)
//   //if this shows red .run anchor build first
//   .accounts({
//     vaultAccount:anchor.web3.SystemProgram.programId,
//   })
//   .rpc();
//   //Verification
//   const vaultData=await program.account.userVault.fetch(vaultPDA);
//   export(vaultData.collateral.toString()).to.equal(amount.toString());
//   console.log("vault successfully update with amount:", vaultData.collateral.toString())

//     })
// })
//------------------------------------------ 14 -----------------------------------
// import  * as anchor from "@coral-xyz/anchor"
// import { Program } from "@coral-xyz/anchor"
// import { expect } from "chai";
// //this file will generated After you run anchor build 
// import {AnchorBoilerplate} from "../target/type/anchor_boileplate";


// describe("Lending Drill",()=>{
//   const provider=anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   //rule: Force the type cast to fix the "property does not exist" error
//   const  program=anchor.workspace.AnchorBoilerplate as Program<AnchorBoilerplate>;
//   it ("Deposit collateral into PDA vault",async()=>{
//     //rule: precision rule never use js number
//     const amount=new anchor.BN(5000000);
//     // find pda (mental map of solana)
//     const [vaultPDA]=anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("vault"),provider.wallet.publicKey.toBuffer()],
//       program.programId
//     );
//     // the handshake
//     await program.methods.deposit(amount)
//     //if this shows red .run anchor build first 
//     .accounts({
//       vaultAccount:anchor.web3.SystemProgram.programId,
//     }).rpc();
//     // Verification
//     const vaultData=await program.account.userVault.fetch(vaultPDA);
//     expect(vaultData.collateral.toString()).to.equal(amount.toString());
//     console.log("vault successfully update with amount:", vaultData.collateral.toString())
//   })
// })
//------------------------------------------ 13 -----------------------------------
// import * as anchor from "@coral-xyz/anchor"
// import { Program } from "@coral-xyz/anchor"
// //this is file will genetated after your anchor build
// import { expect } from "chai"
// import {AnchorBoilerplate} from "../target/type/anchor_boileplate"
// import { program } from "@coral-xyz/anchor/dist/cjs/native/system"

// describe("Lending Drill",()=>{
//   const provider=anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   //rule  Force the type cast to fix the property does not exist error
//   const  program=anchor.workspace.AnchorBoilerplate as Program<AnchorBoilerplate>;
//   it ("Deposit collateral into PDA vault",async()=>{
//     const amount=new anchor.BN(5000000);
//     //find pda(mental map of solana)
//     const [vaultPDA]=anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("vault"),provider.wallet.publicKey.toBuffer()],
//       program.programId
//     );
//     await program.methods.deposit(amount).accounts({
//       vaultAccount:anchor.web3.SystemProgram.programId,
//     }).rpc();
//     //verificaltion
//     const  vaultData=await program.account.userVault.fetch(vaultPDA);
//     expect(vaultData.collateral.toString().to.equal(amount.toString()));
//        console.log("vault successfully update with amount:", vaultData.collateral.toString())
//    })
//   })
//------------------------------------------ 12 -----------------------------------
// import *as anchor from "@coral-xyz/anchor"
// import { Program } from "@coral-xyz/anchor"
// import { expect } from "chai"

// import { AnchorBoilerplate } from "../target/type/anchor_boileplate";


// describe(
//   "Lending Drill",()=> {
//     const provider=anchor.AnchorProvider.env();
//     anchor.setProvider(provider);
//     const program=anchor.workspace.AnchorBoilerplate as Program<AnchorBoilerplate>;
//     it("Deposit collateral into a PDA vault",async()=>{
//       const amount=new anchor.BN(5000000);
//       const [vaultPDA]=anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("vault"),provider.wallet.publicKey.toBuffer()],
//         program.programId
//       )
  
//     await program.methods.deposit(amount)
//     .accounts({
//       vaultAccount:vaultPDA,
//       user:provider.wallet.publicKey,
//       systemProgram:anchor.web3.SystemProgram.programId
//     })
//     .rpc();
//     const vaultData= await program.account.userVault.fetch(vaultPDA);
//     expect(vaultData.collateral.toString()).to.equal(amount.toString());
//     console.log("Vault successfully updated with amount:", vaultData.collateral.toString());
//   }
// });
//------------------------------------------ 11 -----------------------------------
// import * as anchor from "@coral-xyz/anchor"
// import {Program} from "@coral-xyz/anchor"
// // after anchor build

// import{AnchorBoilerplate} from "../target/types/anchor_boileplate"
// import { expect, use } from "chai"

// describe(
//     "Lending Drill", ()=>{
//       const provider= anchor.AnchorProvider.env();
//       anchor.setProvider(provider);
//       //Rule Froce the type script cast to fix the property dose not exist error
//       const program=anchor.workspace.AnchorBoilerplate as Program<AnchorBoilerplate>;
//       it ("Deposit collateral into a PDA vault", async()=>{

//         const amount=new anchor.BN(5000000);
//         // Find the PDA (Mental Map of Solana)
//         const [vaultPDA]=anchor.web3.PublicKey.findProgramAddressSync(
//           [
//             Buffer.from("vault"),provider.wallet.publicKey.toBuffer()
//           ],
//           program.programId
//         )
//         await program.methods.deposit(amount).accounts({
//           vaultAccount:vaultPDA,
//           user:provider.wallet.publicKey,
//           systemProgram:anchor.web3.SystemProgram.programId,
//         }).rpc()
//         const vaultData= await program.account.userVault.fetch(vaultPDA);
//         expect(vaultData.collateral.toString()).to.equal(amount.toString());
//         console.log("Vault successfully updated with amount:", vaultData.collateral.toString())
//       })
//     } 
// )
//------------------------------------------ 10 -----------------------------------
// import * as anchor from "@coral-xyz/anchor"
// import {Program} from "@coral-xyz/anchor"

// // after anchor build

// import{AnchorBoilerplate} from "../target/types/anchor_boileplate"
// import { expect, use } from "chai"


// describe(
// "Lending Drill",() =>{
//   const provider=anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program= anchor.workspace.AnchorBoilerplate as Program<AnchorBoilerplate>;
//   it(

//     async()=>{
//       const amount=new anchor.BN(5000000);
//       const [vaultPDA]=anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("vault"),provider.wallet.publicKey.toBuffer()],
//         program.programId
//       )

//       await program.methods.deposit(amount)
//       .accounts({
//         vaultAccount:vaultPDA,
//         user:provider.wallet.publicKey,
//         systemProgram:anchor.web3.SystemProgram.programId,
//       })
//       .rpc();
//           const vaultData = await program.account.userVault.fetch(vaultPDA);
//     expect(vaultData.collateral.toString()).to.equal(amount.toString());
//     console.log("Vault successfully updated with amount:", vaultData.collateral.toString());
//   });
//     }
//   )
























