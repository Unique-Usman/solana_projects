import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StudentRegister } from "../target/types/student_register";
import { expect } from "chai";

describe("student-register", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.studentRegister as Program<StudentRegister>;
  const student = {
    name: "Usman Akinyemi",
    message: "Grind Bro",
  };

  const [studentPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [provider.wallet.publicKey.toBuffer(), Buffer.from(student.name)],
    program.programId
  );

  it("Is Created", async () => {
    // Add your test here.
    const tx = await program.methods
      .createStudent(student.name, student.message)
      .rpc();

    const student_account = await program.account.student.fetch(studentPDA);

    expect(student_account.name === student.name);
    expect(student_account.message === student.message);
    expect(student_account.owner === provider.wallet.publicKey);

    console.log("Your transaction signature", tx);
  });

  it("Is Updated", async () => {
    // Add your test here.
    let message = "What's up bro";
    const tx = await program.methods.updateStudent(student.name, message).rpc();

    const student_account = await program.account.student.fetch(studentPDA);

    expect(student_account.name === student.name);
    expect(student_account.message === message);
    expect(student_account.owner === provider.wallet.publicKey);

    console.log("Your transaction signature", tx);
  });

  it("Is Deleted", async () => {
    // Add your test here.
    const tx = await program.methods.deleteStudent(student.name).rpc();
    console.log("Your transaction signature", tx);
  });
});
