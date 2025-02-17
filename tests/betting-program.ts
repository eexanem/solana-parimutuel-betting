import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BettingProgram } from "../target/types/betting_program";
import { assert } from "chai";

describe("betting-program", () => {
  // Set up provider and program
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BettingProgram as Program<BettingProgram>;

  it("Initializes a pool with PDA", async () => {
    const poolId = new anchor.BN(1);

    // Don't manually specify systemProgram anymore
    await program.methods
      .initializePool(poolId)
      .accounts({
        admin: provider.wallet.publicKey,  // Keep only necessary accounts
      })
      .rpc();

    // Fetch the PDA automatically using findProgramAddressSync
    const [poolPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("pool"),
        provider.wallet.publicKey.toBuffer(),
        poolId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    const pool = await program.account.pool.fetch(poolPDA);
    assert.equal(pool.admin.toBase58(), provider.wallet.publicKey.toBase58());
  });
});
