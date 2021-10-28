const anchor = require('@project-serum/anchor');
const assert = require("assert");

describe('permissionless-candles', () => {

  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.PermissionlessCandles;
  const candleFrame = anchor.web3.Keypair.generate();


  it('Creates a container for candles', async () => {
    await program.rpc.create({
      accounts: {
        candleFrame: candleFrame.publicKey,
        updater: provider.wallet.payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      instructions: [
        await program.account.candleFrame.createInstruction(candleFrame), // Why do we need to do it this way? I assume because of loader?
      ],
      signers: [candleFrame],
    });

    const res = await program.account.candleFrame.fetch(candleFrame.publicKey);
    assert.ok(res.candles.length === 30);
    assert.ok(res.head.toNumber() === 0);

  });
});
