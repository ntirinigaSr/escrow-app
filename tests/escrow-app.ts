import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { EscrowApp } from '../target/types/escrow_app';

describe('escrow-app', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.EscrowApp as Program<EscrowApp>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
