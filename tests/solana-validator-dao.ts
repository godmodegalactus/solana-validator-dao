import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { SolanaValidatorDao } from "../target/types/solana_validator_dao";
import * as governance from "@solana/spl-governance"
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
} from "@solana/spl-token";
import { BN } from "bn.js";
import mlog from "mocha-logger"; 
import { assert } from "chai";

describe("solana-validator-dao", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(anchor.AnchorProvider.env());
  const governanceProgramId = new web3.PublicKey('GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw');

    // solana logger
  let logsCallback = (logs: anchor.web3.Logs, context: anchor.web3.Context) => {
    mlog.log( logs.logs.join("\n") )
  };
  const listner = connection.onLogs('all', logsCallback)


  const program = anchor.workspace.SolanaValidatorDao as Program<SolanaValidatorDao>;
  const owner = web3.Keypair.generate();
  let mint : web3.PublicKey = null;

  it("Create a governance!", async () => {
    
    await connection.confirmTransaction(await connection.requestAirdrop(owner.publicKey, 1000 * web3.LAMPORTS_PER_SOL));
    
    const instructions: web3.TransactionInstruction[] = []
    const signers: web3.Keypair[] = []
    const _tokenAccount = web3.Keypair.generate();

    mint = await createMint(
      connection,
      owner,
      owner.publicKey,
      null,
      6
    );

    signers.push(owner);

    const councilMint = await createMint(
      connection,
      owner,
      owner.publicKey,
      null,
      6
    );

    // Explicitly request the version before making RPC calls to work around race conditions in resolving
    // the version for RealmInfo
    const programVersion = await governance.getGovernanceProgramVersion(
      connection,
      governanceProgramId,
    )
    mlog.log('program-version: ' + programVersion);
    let blockHash = await connection.getRecentBlockhash();
    mlog.log('block-hash: ' + blockHash.blockhash);

    const tokenAccount = await createAccount(
          connection,
          owner,
          mint,
          owner.publicKey,
          _tokenAccount,
        );

    const realmsArgs = new governance.CreateRealmArgs({
      name: "TestRealms",
      configArgs: new governance.RealmConfigArgs({
        useCouncilMint: false,
        minCommunityTokensToCreateGovernance: new anchor.BN(1000000),
        communityMintMaxVoteWeightSource: new governance.MintMaxVoteWeightSource({ value: new anchor.BN(1000000)}),
        useCommunityVoterWeightAddin: false,
        useMaxCommunityVoterWeightAddin: false,
      })
    });

    const realmAddress = await governance.withCreateRealm(
      instructions,
      governanceProgramId,
      programVersion,
      "Test",
      owner.publicKey,
      mint,
      owner.publicKey,
      undefined,
      new governance.MintMaxVoteWeightSource({ value: new anchor.BN(1000000)}),
      new anchor.BN(1000000),
      undefined,
      undefined,
    );
    mlog.log('realm address: ' + realmAddress);
    mlog.log(instructions[0].programId);
    mlog.log(instructions[0].keys.map(x=> x.pubkey).join(", "));
    mlog.log(instructions[0].data.length)

    
    // const tokenOwnerRecord = await governance.withCreateTokenOwnerRecord(instructions,
    //   governanceProgramId,
    //   realmAddress,
    //   owner.publicKey,
    //   mint,
    //   owner.publicKey);

    // const config = new governance.GovernanceConfig({
    //   voteThresholdPercentage: new governance.VoteThresholdPercentage({value: 1}),
    //   minCommunityTokensToCreateProposal: new anchor.BN(100_000_000),
    //   minInstructionHoldUpTime: 0,
    //   maxVotingTime: 10,
    //   voteTipping: null,
    //   proposalCoolOffTime: null,
    //   minCouncilTokensToCreateProposal: new anchor.BN(0),
    // });

    // const governanceAddress = await governance.withCreateGovernance(
    //       instructions,
    //       governanceProgramId,
    //       programVersion,
    //       realmAddress,
    //       undefined,
    //       config,
    //       tokenOwnerRecord,
    //       owner.publicKey,
    //       owner.publicKey,
    //       null,
    //     );
    // mlog.log('governance address: ' + governanceAddress);

    // const native_treasury =  await governance.withCreateNativeTreasury(
    //     instructions,
    //     governanceProgramId,
    //     governanceAddress,
    //     owner.publicKey
    //   )
    // mlog.log('native treasury address: ' + native_treasury);

    const transaction = new web3.Transaction()
    transaction.add(...instructions)
    await web3.sendAndConfirmTransaction(connection,
      transaction,
      signers,
    )
    //connection.confirmTransaction(await connection.requestAirdrop(native_treasury, 1000 * web3.LAMPORTS_PER_SOL));
  });
});
