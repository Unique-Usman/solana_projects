import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PdaCounter } from "../target/types/pda_counter";

describe("pda-counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const wallet = provider.wallet;
  const signer = (wallet as anchor.Wallet).payer;

  const program = anchor.workspace.pdaCounter as Program<PdaCounter>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initializeCounter()
      .accounts({
        signer: signer.publicKey,
      })
      .signers([signer])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("is counter updated", async () => {
    // Add your test here.
    const tx = await program.methods
      .updateCounter()
      .accounts({
        signer: signer.publicKey,
      })
      .signers([signer])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
