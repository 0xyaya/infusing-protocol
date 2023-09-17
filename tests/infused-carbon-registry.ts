import * as anchor from '@coral-xyz/anchor';
import {Program, utils, BN, AnchorProvider} from '@coral-xyz/anchor';
import {Keypair, PublicKey} from '@solana/web3.js';
import {
    createMint,
    getOrCreateAssociatedTokenAccount,
    mintTo
} from '@solana/spl-token';
import {InfusedCarbonRegistry} from '../target/types/infused_carbon_registry';
import {publicKey} from '@coral-xyz/anchor/dist/cjs/utils';
import {
    AggregatorAccount,
    SwitchboardProgram
} from '@switchboard-xyz/solana.js';
import Big from 'big.js';

const requestAirdrop = async (connection, wallet, amount) => {
    console.log(`Requesting airdrop for ${wallet}`);
    const signature = await connection.requestAirdrop(
        new PublicKey(wallet),
        amount * 1000000000
    );
    const {blockhash, lastValidBlockHeight} =
        await connection.getLatestBlockhash();
    await connection.confirmTransaction(
        {
            blockhash,
            lastValidBlockHeight,
            signature
        },
        'finalized'
    );
    console.log(
        `Tx Complete: https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
};

describe('infused-carbon-registry', () => {
    // Configure the client to use the local cluster.
    const provider = AnchorProvider.env();
    const program = anchor.workspace
        .InfusedCarbonRegistry as Program<InfusedCarbonRegistry>;

    const [state] = PublicKey.findProgramAddressSync(
        [utils.bytes.utf8.encode('global-registry')],
        program.programId
    );
    const holdingAccount = Keypair.generate();
    const feesAccount = Keypair.generate();
    const feedStalenessThreshold = new BN(10000);

    let switchboard: SwitchboardProgram;
    let aggregatorAccount: AggregatorAccount;
    let aggregatorAccountNctUsd: AggregatorAccount;

    const nctUsdPriceFeed = new PublicKey(
        '4YL36VBtFkD2zfNGWdGFSc5suvskjrHnx3Asuksyek1J'
    );
    const solUsdPriceFeed = new PublicKey(
        'GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR'
    );
    before(async () => {
        switchboard = await SwitchboardProgram.fromProvider(provider);
        aggregatorAccount = new AggregatorAccount(switchboard, solUsdPriceFeed);
        // aggregatorAccountNctUsd = new AggregatorAccount(
        //     switchboard,
        //     nctUsdPriceFeed
        // );
    });

    it('Is initialized!', async () => {
        // Add your test here.
        const tx = await program.methods
            .initialize({
                holdingAccount: holdingAccount.publicKey,
                feesAccount: feesAccount.publicKey,
                feedStalenessThreshold
            })
            .accounts({state: state})
            .rpc();
        console.log('Your transaction signature', tx);
    });

    it('Infused an account!', async () => {
        const myKeypair = Keypair.generate();
        const nftMint = Keypair.generate();
        const signer = Keypair.generate();
        const mintAuthority = Keypair.generate();
        const holdingAuthority = Keypair.generate();

        console.log('BEFORE LOAD');

        const result: Big | null = await aggregatorAccount.fetchLatestValue();
        if (result === null) {
            throw new Error('Aggregator holds no value');
        }
        console.log('sol/usd data feed: ', result.toString());
        const resultNctUsd = 1.44;
        // const resultNctUsd: Big | null =
        //     await aggregatorAccountNctUsd.fetchLatestValue();
        // if (result === null) {
        //     throw new Error('Aggregator holds no value');
        // }
        console.log('nct/usd data feed: ', resultNctUsd.toString());

        await requestAirdrop(provider.connection, signer.publicKey, 100);

        const [infusedAccount] = PublicKey.findProgramAddressSync(
            [
                utils.bytes.utf8.encode('infused-account'),
                nftMint.publicKey.toBytes()
            ],
            program.programId
        );
        let mint: PublicKey;
        try {
            mint = await createMint(
                provider.connection,
                signer,
                mintAuthority.publicKey,
                mintAuthority.publicKey,
                9
            );
            console.log('Token Mint created: ', mint.toString());
        } catch (e) {}

        // Create Token Account for the owner
        const tokenAccount = await getOrCreateAssociatedTokenAccount(
            provider.connection,
            signer, // rent payer for this account
            mint, // mint of the token
            signer.publicKey // owner of this token account
        );
        const holdingAccountPda = await getOrCreateAssociatedTokenAccount(
            provider.connection,
            signer, // rent payer for this account
            mint, // mint of the token
            holdingAuthority.publicKey // owner of this token account
        );
        console.log('Token Account created: ', tokenAccount);

        let decimals = 1_000_000_000;
        const sig = await mintTo(
            provider.connection,
            signer,
            mint,
            tokenAccount.address,
            mintAuthority,
            1000000 * decimals
        );

        try {
            console.log('DEBUG');
            // Add your test here.
            const tx = await program.methods
                .infuse(new BN(16), resultNctUsd)
                .accounts({
                    globalRegistry: state,
                    nftMint: nftMint.publicKey,
                    infusedAccount,
                    holdingAccount: holdingAccountPda.address,
                    mint,
                    solUsdPriceFeed: aggregatorAccount.publicKey
                    // nctUsdPriceFeed: aggregatorAccountNctUsd.publicKey
                })
                .rpc();
            console.log('Your transaction signature', tx);
        } catch (e) {
            console.log(e);
        }
    });
});
