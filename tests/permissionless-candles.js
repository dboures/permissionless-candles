const anchor = require('@project-serum/anchor');

describe('permissionless-candles', () => {

  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.PermissionlessCandles;

    const candleFrameKeypair = anchor.web3.Keypair.generate();
    const tx = await program.rpc.create(
      {
      accounts: {
        candleFrame: candleFrameKeypair.publicKey,
        updater: provider.wallet.payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [provider.wallet.payer, candleFrameKeypair]
    });
    console.log("Your transaction signature", tx);
  });
});
