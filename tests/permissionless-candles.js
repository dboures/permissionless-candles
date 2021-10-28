const anchor = require('@project-serum/anchor');
const assert = require("assert");
const { BN } = require('bn.js');

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

  it('Can add a candle to the container', async () => {
    await program.rpc.update(
      new BN(1),
      new BN(2),
      new BN(3),
      new BN(4),
      new BN(5),
      new BN(6),
      {
      accounts: {
        candleFrame: candleFrame.publicKey,
        updater: provider.wallet.payer.publicKey,
      }
    });

    const res = await program.account.candleFrame.fetch(candleFrame.publicKey);
    assert.ok(res.candles.length === 30);
    assert.ok(res.head.toNumber() === 1);
    assert.ok(res.candles[0].open.toNumber() === 1);
    assert.ok(res.candles[0].high.toNumber() === 2);
    assert.ok(res.candles[0].low.toNumber() === 3);
    assert.ok(res.candles[0].close.toNumber() === 4);
    assert.ok(res.candles[0].volume.toNumber() === 5);
    assert.ok(res.candles[0].unixTime.toNumber() === 6);
  });

});
