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
import { sleep } from '@blockworks-foundation/mango-client'

describe("solana-validator-dao", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(anchor.AnchorProvider.env());
  const governanceProgramId = new web3.PublicKey('GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw');
  const program = anchor.workspace.SolanaValidatorDao as Program<SolanaValidatorDao>;

  // solana logger
  // let logsCallback = (logs: anchor.web3.Logs, context: anchor.web3.Context) => {
  //   mlog.log( logs.logs.join("\n") )
  // };
  // const listner = connection.onLogs(program.programId, logsCallback)

  const owner = web3.Keypair.generate();
  mlog.log("owner : " + owner.publicKey)
  let communityMint: web3.PublicKey = null;
  let ownerCommunityAccount: web3.PublicKey = null;
  const programVersion = 3.0;

  //governance data
  let realmAddress: web3.PublicKey = null;
  let tokenOwnerRecord: web3.PublicKey = null;
  let governanceAddress: web3.PublicKey = null;
  let nativeTreasury: web3.PublicKey = null;

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
      { commitment: 'confirmed' }
    );
    mlog.log("created community mint")
    const mintData = await spl_token.getMint(connection, communityMint, 'confirmed');
    ownerCommunityAccount = await spl_token.createAccount(
      connection,
      owner,
      communityMint,
      owner.publicKey,
      undefined,
      { commitment: 'confirmed' }
    );
    await spl_token.mintTo(connection, owner, communityMint, ownerCommunityAccount, owner, 100_000_000, undefined, { commitment: 'confirmed' });
    
    const communityMintData = await spl_token.getMint(connection, communityMint);
    mlog.log(communityMintData.supply)

    const councilMint = await createMint(
      connection,
      owner,
      owner.publicKey,
      null,
      6,
      undefined,
      { commitment: 'confirmed' }
    );

    signers.push(owner);
    mlog.log('program-version: ' + programVersion);
    let blockHash = await connection.getRecentBlockhash();
    mlog.log('block-hash: ' + blockHash.blockhash);

    let mintMaxVoteWeightSource = new governance.MintMaxVoteWeightSource({ value: new anchor.BN(100_000_000) });
    mintMaxVoteWeightSource.type = governance.MintMaxVoteWeightSourceType.SupplyFraction;

    realmAddress = await governance.withCreateRealm(
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

    tokenOwnerRecord = await governance.withCreateTokenOwnerRecord(instructions,
      governanceProgramId,
      realmAddress,
      owner.publicKey,
      communityMint,
      owner.publicKey);
    mlog.log('token owner record: ' + tokenOwnerRecord);

    let votePercentage = new governance.VoteThresholdPercentage({ value: 1 });
    votePercentage.type = governance.VoteThresholdPercentageType.YesVote;
    votePercentage.value = 10;

    const config = new governance.GovernanceConfig({
      voteThresholdPercentage: votePercentage,
      minCommunityTokensToCreateProposal: new anchor.BN(1_000_000),
      minInstructionHoldUpTime: 0,
      maxVotingTime: 2,
      voteTipping: governance.VoteTipping.Early,
      proposalCoolOffTime: null,
      minCouncilTokensToCreateProposal: new anchor.BN(1_000_000),
    });

    governanceAddress = await governance.withCreateGovernance(
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

    nativeTreasury = await governance.withCreateNativeTreasury(
      instructions,
      governanceProgramId,
      governanceAddress,
      owner.publicKey
    )
    mlog.log('native treasury address: ' + nativeTreasury);

    const transaction = new web3.Transaction()
    transaction.add(...instructions)
    await web3.sendAndConfirmTransaction(connection,
      transaction,
      signers,
    )
    connection.confirmTransaction(await connection.requestAirdrop(nativeTreasury, 2000 * web3.LAMPORTS_PER_SOL));
  });

  let proposalAddress :web3.PublicKey = null;
  let transactionAddress: web3.PublicKey = null;
  let instructionData: governance.InstructionData = null;

  it("Create proposals", async () => {
    const instructions: web3.TransactionInstruction[] = []
    const signers: web3.Keypair[] = []
    signers.push(owner);

    mlog.log('deposit some community tokens')
    await governance.withDepositGoverningTokens(
      instructions,
      governanceProgramId,
      programVersion,
      realmAddress,
      ownerCommunityAccount,
      communityMint,
      owner.publicKey,
      owner.publicKey,
      owner.publicKey,
      new anchor.BN(100_000_000),
    )

    mlog.log('creating proposal instruction ');
    proposalAddress = await governance.withCreateProposal(
      instructions,
      governanceProgramId,
      programVersion,
      realmAddress,
      governanceAddress,
      tokenOwnerRecord,
      "Test Proposal",
      "Test Proposal",
      communityMint,
      owner.publicKey,
      0,
      governance.VoteType.SINGLE_CHOICE,
      ['yes', 'no'],
      true,
      owner.publicKey,
    );

    mlog.log('building tranction');
    const [daoStakeAccount, _bump] = await web3.PublicKey.findProgramAddress([Buffer.from("validator_dao_stake_account"), governanceAddress.toBuffer(), nativeTreasury.toBuffer(), governanceProgramId.toBuffer()], program.programId);
    const instruction = await program
    .methods
    .initialize(new anchor.BN(1000_000_000), new anchor.BN(10), web3.PublicKey.default)
    .accounts(
      {
        governanceId : governanceAddress,
        governanceNativeTreasuryAccount : nativeTreasury,
        daoStakeAccount: daoStakeAccount,
        payer:owner.publicKey,
        governanceProgram: governanceProgramId,
        stakeProgram: web3.StakeProgram.programId,
        systemProgram: web3.SystemProgram.programId,
        rentProgram: web3.SYSVAR_RENT_PUBKEY,
      }
    ).instruction();

    const createInstructionData = (instruction: web3.TransactionInstruction) => {
      return new governance.InstructionData({
        programId: instruction.programId,
        data: instruction.data,
        accounts: instruction.keys.map(
          k =>
            new governance.AccountMetaData({
              pubkey: k.pubkey,
              isSigner: k.isSigner,
              isWritable: k.isWritable,
            }),
        ),
      });
    };

    instructionData = createInstructionData(instruction);
    mlog.log('adding transaction');
    transactionAddress = await governance.withInsertTransaction(
      instructions,
      governanceProgramId,
      programVersion,
      governanceAddress,
      proposalAddress,
      tokenOwnerRecord,
      owner.publicKey,
      0,
      0,
      0,
      [instructionData],
      owner.publicKey,
    )

    const transaction = new web3.Transaction()
    transaction.add(...instructions)
    await web3.sendAndConfirmTransaction(connection,
      transaction,
      signers,
    )
  });

  it("passing the proposal", async () => {
    const instructions: web3.TransactionInstruction[] = [];
    const signers: web3.Keypair[] = []
    signers.push(owner);

    mlog.log('signoff proposal')
    await governance.withSignOffProposal(
      instructions,
      governanceProgramId,
      programVersion,
      realmAddress,
      governanceAddress,
      proposalAddress,
      owner.publicKey,
      tokenOwnerRecord,
      tokenOwnerRecord,
    );
    await mlog.log('voting for proposal')
    governance.withCastVote(
      instructions,
      governanceProgramId,
      programVersion,
      realmAddress,
      governanceAddress,
      proposalAddress,
      tokenOwnerRecord,
      tokenOwnerRecord,
      owner.publicKey,
      owner.publicKey,
      new governance.Vote({
        voteType: governance.VoteKind.Approve,
        approveChoices: [new governance.VoteChoice({rank: 0,weightPercentage:100})],
        deny: false,
      }),
      owner.publicKey,
    )

    const transaction = new web3.Transaction()
    transaction.add(...instructions)
    await web3.sendAndConfirmTransaction(connection,
      transaction,
      signers,
    )

    mlog.log('wait for proposal voting to be finished');
    await sleep(3000);
  });

  it("Finalizing the proposal", async () => {
    const instructions: web3.TransactionInstruction[] = [];
    const signers: web3.Keypair[] = []
    signers.push(owner);

    await governance.withFinalizeVote(
      instructions,
      governanceProgramId,
      programVersion,
      realmAddress,
      governanceAddress,
      proposalAddress,
      tokenOwnerRecord,
      communityMint,
    );

    const transaction = new web3.Transaction()
    transaction.add(...instructions)
    await web3.sendAndConfirmTransaction(connection,
      transaction,
      signers,
    );
    mlog.log('proposal finalized');
    const proposal = await governance.getGovernanceAccount(connection, proposalAddress, governance.Proposal);
    const governanceData = await governance.getGovernance(connection, governanceAddress)
    const proposalAccount = proposal.account;
    mlog.log('---------proposal details----------------');
    mlog.log('has time ended ' + proposalAccount.hasVoteTimeEnded(governanceData.account))
    mlog.log('is finalized ' + proposalAccount.isVoteFinalized())
    mlog.log('state ' + proposalAccount.state)
  });
  // executing the proposal
  it("executing the proposal", async () => {
    const instructions: web3.TransactionInstruction[] = [];
    const signers: web3.Keypair[] = []
    signers.push(owner);

    await governance.withExecuteTransaction(
      instructions,
      governanceProgramId,
      programVersion,
      governanceAddress,
      proposalAddress,
      transactionAddress,
      [instructionData]
    );

    const transaction = new web3.Transaction()
    transaction.add(...instructions)
    await web3.sendAndConfirmTransaction(connection,
      transaction,
      signers,
    )
  });

  it("checking stake account", async () => {
    web3.SOLANA_SCHEMA
  });
});
