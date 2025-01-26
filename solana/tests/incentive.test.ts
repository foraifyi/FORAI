import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import { assert } from 'chai';
import { readFileSync } from 'fs';

describe('Incentive Program', () => {
    const connection = new Connection('http://localhost:8899', 'confirmed');
    let programId: PublicKey;
    let authority: Keypair;
    let treasury: Keypair;
    
    before(async () => {
        // Load program ID from the deployed program
        programId = new PublicKey(JSON.parse(readFileSync('program-id.json', 'utf-8')));
        
        // Create test accounts
        authority = Keypair.generate();
        treasury = Keypair.generate();
        
        // Airdrop SOL to authority
        const signature = await connection.requestAirdrop(
            authority.publicKey,
            1000000000 // 1 SOL
        );
        await connection.confirmTransaction(signature);
    });

    it('should initialize agent account', async () => {
        const agent = Keypair.generate();
        const tx = new Transaction().add(
            SystemProgram.createAccount({
                fromPubkey: authority.publicKey,
                newAccountPubkey: agent.publicKey,
                lamports: await connection.getMinimumBalanceForRentExemption(65),
                space: 65,
                programId,
            })
        );

        await sendAndConfirmTransaction(
            connection,
            tx,
            [authority, agent]
        );

        const accountInfo = await connection.getAccountInfo(agent.publicKey);
        assert(accountInfo !== null);
        assert.equal(accountInfo.owner.toBase58(), programId.toBase58());
    });

    it('should reward agent', async () => {
        // Test implementation will be added after program deployment
    });
}); 