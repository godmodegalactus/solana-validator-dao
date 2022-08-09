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
import { assert } from "chai";
import {TextDecoder} from 'text-encoding'
import _ from "lodash";

describe("register-validator-provider", () => {
    const provider = anchor.AnchorProvider.env();
    const connection = provider.connection;
    anchor.setProvider(anchor.AnchorProvider.env());
    const governanceProgramId = new web3.PublicKey('GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw');
    const program = anchor.workspace.SolanaValidatorDao as Program<SolanaValidatorDao>;

    // solana logger
    // let logsCallback = (logs: anchor.web3.Logs, context: anchor.web3.Context) => {
    //     mlog.log( logs.logs.join("\n") )
    // };
    // const listner = connection.onLogs(program.programId, logsCallback)


    const validatorProvider = web3.Keypair.generate();
    let paymentMint: web3.PublicKey = null;
    let providerPDA : web3.PublicKey = null;
    
    it("Register a provider", async () => {
        const owner = web3.Keypair.generate();
        await connection.confirmTransaction( await connection.requestAirdrop( owner.publicKey, 2*web3.LAMPORTS_PER_SOL));
        await connection.confirmTransaction( await connection.requestAirdrop( validatorProvider.publicKey, 2*web3.LAMPORTS_PER_SOL));
        
        paymentMint = await createMint(
            connection,
            owner,
            owner.publicKey,
            null,
            6,
            undefined,
            { commitment: 'confirmed' }
          );
        let _bump = 0;
        [providerPDA, _bump] = await web3.PublicKey.findProgramAddress([Buffer.from("validator_provider"), validatorProvider.publicKey.toBuffer()], program.programId)
        
        const tx = await program.methods.registerValidatorProvider( 
            new anchor.BN(15), 
            "Orion validators", 
            "We can build and optimise a validator for solana mainnet, our validators are million times faster than any other provider"
        )
        .accounts({
            owner: validatorProvider.publicKey,
            paymentMint,
            providerData: providerPDA,
            systemProgram: web3.SystemProgram.programId,
        })
        .signers([validatorProvider]).rpc();
        connection.confirmTransaction(tx);
    });

    it( "Verify data", async () => {

        type DataType = {validatorProvider:{}} | {governanceProvider:{}} | {contract:{}};
        type ValidatorProvider = anchor.IdlAccounts<SolanaValidatorDao>["validatorProvider"];
        type Metadata = Omit<anchor.IdlTypes<SolanaValidatorDao>["Metadata"], "dataType"> & { dataType: DataType };
    
        const textdecoder = new TextDecoder('utf-8')
        const instance : ValidatorProvider = await program.account.validatorProvider.fetch(providerPDA);
        //const metaData : Metadata = instance.metaData;
        const name = instance.name.splice(0, instance.name.indexOf(0)).map(x=> String.fromCharCode(x)).join("");
        const desc = instance.description.splice(0, instance.description.indexOf(0)).map(x=> String.fromCharCode(x)).join("")
        mlog.log(instance.paymentMint);
        mlog.log(paymentMint);
        assert(instance.owner.equals(validatorProvider.publicKey), "owner should be set");
        assert(instance.reviewCount == 0, "review count should be 0")
        assert(instance.rating == 0.0, "rating should be 0")
        assert(instance.paymentMint.equals(paymentMint), "payment mint should be set")
        //assert(metaData.isInitialized = true, "metadata should be initialized")
        //assert(_.isEqual(metaData.dataType, {validatorProvider:{}}))
        assert( name == "Orion validators");
        assert( desc == "We can build and optimise a validator for solana mainnet, our validators are million times faster than any other provider");
    } )
})