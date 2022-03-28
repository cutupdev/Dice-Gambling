import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { DdrDice } from '../target/types/ddr_dice';

describe('ddr_dice', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.DdrDice as Program<DdrDice>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
