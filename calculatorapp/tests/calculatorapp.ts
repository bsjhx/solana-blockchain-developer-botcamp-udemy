import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculatorapp } from "../target/types/calculatorapp";
import { SystemProgram } from "@coral-xyz/anchor";
import { assert } from "chai";

describe("calculatorapp", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const calculator = anchor.web3.Keypair.generate();
  const program = anchor.workspace.Calculatorapp as Program<Calculatorapp>;

  it("Create calculator", async () => {
    await program.methods
      .create("hi")
      .accounts({
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([calculator])
      .rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.greeting === "hi");
    assert.ok(Number(account.result) === 0);
  });
});
