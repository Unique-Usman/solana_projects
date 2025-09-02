import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.counter as Program<Counter>;
  const signer = anchor.web3.Keypair.generate();
  const data_account = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.

    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        signer.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
    );

    const tx = await program.methods
      .initCounter()
      .accounts({
        signer: signer.publicKey,
        counter: data_account.publicKey,
      })
      .signers([signer, data_account])
      .rpc();
    console.log("Your transaction signature", tx);

    setTimeout(() => {
      console.log("After 2 seconds");
    }, 2000); // 2000 milliseconds = 2 second

    const tx2 = await program.methods
      .updateCounter()
      .accounts({ counter: data_account.publicKey })
      .rpc();

    console.log("Your increamentsignature", tx2);

    const data = await program.account.counter.fetch(data_account.publicKey);
    console.log(data);
  });
});
