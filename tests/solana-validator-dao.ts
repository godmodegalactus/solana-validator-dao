import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaValidatorDao } from "../target/types/solana_validator_dao";

describe("solana-validator-dao", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaValidatorDao as Program<SolanaValidatorDao>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
