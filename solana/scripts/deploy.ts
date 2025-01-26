import {
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    BpfLoader,
    Transaction,
    SystemProgram,
    sendAndConfirmTransaction,
    PublicKey
} from '@solana/web3.js';
import { readFileSync, writeFileSync } from 'fs';
import path from 'path';

async function main() {
    // Connect to local cluster
    const connection = new Connection('http://localhost:8899', 'confirmed');
    
    // Load or create deployer keypair
    let deployer: Keypair;
    try {
        const deployerJson = readFileSync('./deployer-keypair.json', 'utf-8');
        deployer = Keypair.fromSecretKey(Buffer.from(JSON.parse(deployerJson)));
    } catch {
        deployer = Keypair.generate();
        writeFileSync(
            './deployer-keypair.json',
            JSON.stringify(Array.from(deployer.secretKey))
        );
    }

    // Request airdrop for deployer
    const signature = await connection.requestAirdrop(
        deployer.publicKey,
        2 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(signature);

    console.log('Deploying program...');
    
    // Read program data
    const programPath = path.join(__dirname, '../target/deploy/forai_solana.so');
    const program = readFileSync(programPath);

    // Create program account
    const programId = Keypair.generate();
    
    // Calculate program space
    const space = program.length;
    
    // Calculate rent
    const rentExemptionAmount = await connection.getMinimumBalanceForRentExemption(space);

    // Get BPF Loader program ID
    const bpfLoaderProgramId = new PublicKey('BPFLoader2111111111111111111111111111111111');

    // Create transaction to create program account
    const transaction = new Transaction().add(
        SystemProgram.createAccount({
            fromPubkey: deployer.publicKey,
            newAccountPubkey: programId.publicKey,
            lamports: rentExemptionAmount,
            space: space,
            programId: bpfLoaderProgramId,
        })
    );

    await sendAndConfirmTransaction(connection, transaction, [deployer, programId]);

    // Load program
    await BpfLoader.load(
        connection,
        deployer,
        programId,
        program,
        bpfLoaderProgramId
    );

    console.log('Program deployed successfully');
    console.log('Program ID:', programId.publicKey.toString());

    // Save program ID
    writeFileSync(
        './program-id.json',
        JSON.stringify(programId.publicKey.toString())
    );
}

main().catch(console.error); 