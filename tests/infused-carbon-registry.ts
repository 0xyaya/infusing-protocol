import * as anchor from '@coral-xyz/anchor';
import {
  Program,
  utils,
  BN,
  AnchorProvider,
} from '@coral-xyz/anchor';
import {
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from '@solana/spl-token';
import { InfusedCarbonRegistry } from '../target/types/infused_carbon_registry';
import { publicKey } from '@coral-xyz/anchor/dist/cjs/utils';
import {
  AggregatorAccount,
  SwitchboardProgram,
} from '@switchboard-xyz/solana.js';
import Big from 'big.js';
import * as token from '@solana/spl-token';
import { assert, expect } from 'chai';

const requestAirdrop = async (connection, wallet, amount) => {
  console.log(`Requesting airdrop for ${wallet}`);
  const signature = await connection.requestAirdrop(
    new PublicKey(wallet),
    amount * 1000000000
  );
  const { blockhash, lastValidBlockHeight } =
    await connection.getLatestBlockhash();
  await connection.confirmTransaction(
    {
      blockhash,
      lastValidBlockHeight,
      signature,
    },
    'finalized'
  );
  console.log(
    `Tx Complete: https://explorer.solana.com/tx/${signature}?cluster=devnet`
  );
};

describe('infused-carbon-registry', () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = AnchorProvider.env();
  // Configure the client to use the local cluster.
  const program = anchor.workspace
    .InfusedCarbonRegistry as Program<InfusedCarbonRegistry>;

  const [state] = PublicKey.findProgramAddressSync(
    [utils.bytes.utf8.encode('global-registry')],
    program.programId
  );

  // const holdingAccount = new anchor.web3.Keypair();
  const holdingAccount = new PublicKey(
    '6ACx2p98pF7m58GYZViCtv4sxYED9Yj5HDcMZk6BR1FK'
  );
  // const feesAccount = new anchor.web3.Keypair();
  const feesAccount = new PublicKey(
    '735WcMTFNG3qXQat7VP2uxMpSvts969xg5vnKPiDpsp9'
  ); //735WcMTFNG3qXQat7VP2uxMpSvts969xg5vnKPiDpsp9
  const feedStalenessThreshold = new BN(10000);
  const nftMint = Keypair.generate();

  let switchboard: SwitchboardProgram;
  let aggregatorAccount: AggregatorAccount;
  let aggregatorAccountNctUsd: AggregatorAccount;

  const nctUsdPriceFeed = new PublicKey(
    '4YL36VBtFkD2zfNGWdGFSc5suvskjrHnx3Asuksyek1J'
  );
  const solUsdPriceFeed = new PublicKey(
    'GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR'
  );

  // before(async () => {
  //   switchboard = await SwitchboardProgram.fromProvider(provider);
  //   aggregatorAccount = new AggregatorAccount(
  //     switchboard,
  //     solUsdPriceFeed
  //   );
  //   aggregatorAccountNctUsd = new AggregatorAccount(
  //     switchboard,
  //     nctUsdPriceFeed
  //   );
  // });

  it('send sol', async () => {
    console.log('Start test...');
    const holdingAccountBalance =
      await provider.connection.getBalance(holdingAccount);
    console.log('Before: ', holdingAccountBalance);
    const tx = await program.methods
      .sendSol(new BN(1 * LAMPORTS_PER_SOL))
      .accounts({
        from: provider.publicKey,
        to: holdingAccount,
      })
      .rpc();

    console.log('Your transaction signature', tx);
    const holdingAccountBalanceAfter =
      await provider.connection.getBalance(holdingAccount);
    console.log('balance after: ', holdingAccountBalanceAfter);
  });

  it('Is initialized!', async () => {
    console.log('Start test...');
    const tx = await program.methods
      .initialize()
      .accounts({
        state,
        holdingAccount: holdingAccount,
        feesAccount: feesAccount,
      })
      .rpc();

    console.log('Your transaction signature', tx);
  });

  it('Infused an account!', async () => {
    // const result: Big | null =
    //   await aggregatorAccount.fetchLatestValue();
    // if (result === null) {
    //   throw new Error('Aggregator holds no value');
    // }

    // const resultNctUsd: Big | null =
    //   await aggregatorAccountNctUsd.fetchLatestValue();
    // if (result === null) {
    //   throw new Error('Aggregator holds no value');
    // }

    // await requestAirdrop(provider.connection, signer.publicKey, 100);

    const [infusedAccount] = PublicKey.findProgramAddressSync(
      [
        utils.bytes.utf8.encode('infused-account'),
        nftMint.publicKey.toBytes(),
      ],
      program.programId
    );
    const signerAccountBalanceBefore =
      await provider.connection.getBalance(provider.wallet.publicKey);
    try {
      // Add your test here.
      const tx = await program.methods
        .infuse(new BN(1))
        .accounts({
          globalRegistry: state,
          nftMint: nftMint.publicKey,
          infusedAccount,
          holdingAccount: holdingAccount,
          feesAccount: feesAccount,
        })
        .rpc();
      console.log('Your transaction signature', tx);
    } catch (e) {
      console.log(e);
    }

    const holdingAccountBalance =
      await provider.connection.getBalance(holdingAccount);
    const newFeesAccountBalance =
      await provider.connection.getBalance(feesAccount);
    const signerAccountBalance = await provider.connection.getBalance(
      provider.wallet.publicKey
    );

    const [infusedAccountAddress] = PublicKey.findProgramAddressSync(
      [
        utils.bytes.utf8.encode('infused-account'),
        nftMint.publicKey.toBytes(),
      ],
      program.programId
    );
    const infusedAccountState =
      await program.account.infusedAccount.fetch(infusedAccount);

    expect(
      holdingAccountBalance,
      'The holding account should have more than 1 lamports'
    ).to.be.greaterThan(new BN(1).toNumber());
    expect(
      newFeesAccountBalance,
      'The fees account should have more than 1 lamports'
    ).to.be.greaterThan(new BN(1).toNumber());
    expect(
      signerAccountBalance,
      'The signer account should have less lamports'
    ).to.be.lessThan(signerAccountBalanceBefore);
  });

  it('increase the carbon score', async () => {
    const [infusedAccountAddress] = PublicKey.findProgramAddressSync(
      [
        utils.bytes.utf8.encode('infused-account'),
        nftMint.publicKey.toBytes(),
      ],
      program.programId
    );
    const infusedAccount = await program.account.infusedAccount.fetch(
      infusedAccountAddress
    );

    console.log(
      'infusedAccount carbonScore: ',
      infusedAccount.carbonScore.toNumber()
    );

    expect(
      infusedAccount.carbonScore.toNumber(),
      'The infused carbon score is greater than 0'
    ).to.be.greaterThan(0);
  });
});
