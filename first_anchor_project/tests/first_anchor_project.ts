import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FirstAnchorProject } from "../target/types/first_anchor_project";

describe("first_anchor_project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .firstAnchorProject as Program<FirstAnchorProject>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
