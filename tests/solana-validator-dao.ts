import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { SolanaValidatorDao } from "../target/types/solana_validator_dao";
import * as governance from "@solana/spl-governance"
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
} from "@solana/spl-token";
import * as spl_token from "@solana/spl-token";
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
  // let logsCallback = (logs: anchor.web3.Logs, context: anchor.web3.Context) => {
  //   mlog.log( logs.logs.join("\n") )
  // };
  // const listner = connection.onLogs('all', logsCallback)

  const program = anchor.workspace.SolanaValidatorDao as Program<SolanaValidatorDao>;
  const owner = web3.Keypair.generate();
  mlog.log("owner : " + owner.publicKey)
  let communityMint : web3.PublicKey = null;

  it("Create a governance!", async () => {
    
    await connection.confirmTransaction(await connection.requestAirdrop(owner.publicKey, 2 * web3.LAMPORTS_PER_SOL));
    
    const instructions: web3.TransactionInstruction[] = []
    const signers: web3.Keypair[] = []
    const _tokenAccount = web3.Keypair.generate();
    
    mlog.log("creating community mint")
    communityMint = await createMint(
      connection,
      owner,
      owner.publicKey,
      null,
      6,
      undefined,
      {commitment:'confirmed'}
    );
    mlog.log("created community mint")
    const mintData = await spl_token.getMint(connection, communityMint, 'confirmed');
    
    const councilMint = await createMint(
      connection,
      owner,
      owner.publicKey,
      null,
      6,
      undefined,
      {commitment:'confirmed'}
    );

    signers.push(owner);
    const programVersion = 2;
    mlog.log('program-version: ' + programVersion);
    let blockHash = await connection.getRecentBlockhash();
    mlog.log('block-hash: ' + blockHash.blockhash);

    let mintMaxVoteWeightSource = new governance.MintMaxVoteWeightSource({ value: new anchor.BN(100_000_000)});
    mintMaxVoteWeightSource.type = governance.MintMaxVoteWeightSourceType.SupplyFraction;
    
    const realmAddress = await governance.withCreateRealm(
      instructions,
      governanceProgramId,
      programVersion,
      "Test",
      owner.publicKey,
      communityMint,
      owner.publicKey,
      councilMint,
      mintMaxVoteWeightSource,
      new anchor.BN(1_000_000),
      undefined,
      undefined,
    );
    mlog.log('realm address: ' + realmAddress);

    const tokenOwnerRecord = await governance.withCreateTokenOwnerRecord(instructions,
      governanceProgramId,
      realmAddress,
      owner.publicKey,
      communityMint,
      owner.publicKey);
    mlog.log('token owner record: ' + tokenOwnerRecord);

    let votePercentage = new governance.VoteThresholdPercentage({value: 10});
    votePercentage.type = governance.VoteThresholdPercentageType.YesVote;
    votePercentage.value = 10;

    const config = new governance.GovernanceConfig({
      voteThresholdPercentage: votePercentage,
      minCommunityTokensToCreateProposal: new anchor.BN(1_000_000),
      minInstructionHoldUpTime: 0,
      maxVotingTime: 10,
      voteTipping: governance.VoteTipping.Early,
      proposalCoolOffTime: null,
      minCouncilTokensToCreateProposal: new anchor.BN(1_000_000),
    });

    const governanceAddress = await governance.withCreateGovernance(
          instructions,
          governanceProgramId,
          programVersion,
          realmAddress,
          undefined,
          config,
          tokenOwnerRecord,
          owner.publicKey,
          owner.publicKey,
          null,
        );
    mlog.log('governance address: ' + governanceAddress);

    const native_treasury =  await governance.withCreateNativeTreasury(
        instructions,
        governanceProgramId,
        governanceAddress,
        owner.publicKey
      )
    mlog.log('native treasury address: ' + native_treasury);

    const transaction = new web3.Transaction()
    transaction.add(...instructions)
    await web3.sendAndConfirmTransaction(connection,
      transaction,
      signers,
    )
    //connection.confirmTransaction(await connection.requestAirdrop(native_treasury, 2 * web3.LAMPORTS_PER_SOL));
  });
});
