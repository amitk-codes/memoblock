import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Memoblock } from "../target/types/memoblock";
import { assert } from "chai";

describe("memoblock", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Memoblock as Program<Memoblock>;
  const web3 = anchor.web3;
  const connection = program.provider.connection;

  const fundWallet = async (walletAddress: anchor.web3.PublicKey) => {
    const airdropSignature = await connection.requestAirdrop(
      walletAddress,
      2 * web3.LAMPORTS_PER_SOL
    );
    const { blockhash, lastValidBlockHeight } =
      await connection.getLatestBlockhash();

    await connection.confirmTransaction(
      {
        lastValidBlockHeight,
        blockhash,
        signature: airdropSignature,
      },
      "confirmed"
    );
  };

  it("Creates a memory", async () => {
    const demoKeypair = web3.Keypair.generate();
    await fundWallet(demoKeypair.publicKey);

    const memoryId = web3.Keypair.generate().publicKey;
    const title = "Memory Title";
    const description = "Memory Description";

    const [memoryAccount] = web3.PublicKey.findProgramAddressSync(
      [memoryId.toBuffer(), demoKeypair.publicKey.toBuffer()],
      program.programId
    );

    const accounts = {
      payer: demoKeypair.publicKey,
      memory_account: memoryAccount,
      system_program: web3.SystemProgram.programId,
    };

    await program.methods
      .createMemory(memoryId, title, description)
      .accounts(accounts)
      .signers([demoKeypair])
      .rpc();

    const memory = await program.account.memory.fetch(memoryAccount);
    assert.equal(memory.id.toString(), memoryId.toString());
    assert.equal(memory.title, title);
    assert.equal(memory.description, description);
    assert.equal(memory.owner.toString(), demoKeypair.publicKey.toString());
  });

  it("Updates a memory", async () => {
    const demoKeypair = web3.Keypair.generate();
    await fundWallet(demoKeypair.publicKey);

    const memoryId = web3.Keypair.generate().publicKey;
    const initialTitle = "Initial Title";
    const initialDescription = "Initial Description";

    const [memoryAccount] = web3.PublicKey.findProgramAddressSync(
      [memoryId.toBuffer(), demoKeypair.publicKey.toBuffer()],
      program.programId
    );

    const accounts = {
      payer: demoKeypair.publicKey,
      memory_account: memoryAccount,
      system_program: web3.SystemProgram.programId,
    };

    await program.methods
      .createMemory(memoryId, initialTitle, initialDescription)
      .accounts(accounts)
      .signers([demoKeypair])
      .rpc();

    const newTitle = "Updated Title";
    const newDescription = "Updated Description";

    await program.methods
      .updateMemory(memoryId, newTitle, newDescription)
      .accounts(accounts)
      .signers([demoKeypair])
      .rpc();

    const updatedMemory = await program.account.memory.fetch(memoryAccount);
    assert.equal(updatedMemory.title, newTitle);
    assert.equal(updatedMemory.description, newDescription);
  });

  it("Deletes a memory", async () => {
    const demoKeypair = web3.Keypair.generate();
    await fundWallet(demoKeypair.publicKey);

    const memoryId = web3.Keypair.generate().publicKey;
    const title = "Memory to Delete";
    const description = "Description of memory to delete";

    const [memoryAccount] = web3.PublicKey.findProgramAddressSync(
      [memoryId.toBuffer(), demoKeypair.publicKey.toBuffer()],
      program.programId
    );

    const accounts = {
      payer: demoKeypair.publicKey,
      memory_account: memoryAccount,
      system_program: web3.SystemProgram.programId,
    };

    await program.methods
      .createMemory(memoryId, title, description)
      .accounts(accounts)
      .signers([demoKeypair])
      .rpc();

    await program.methods
      .deleteMemory(memoryId)
      .accounts(accounts)
      .signers([demoKeypair])
      .rpc();

    try {
      await program.account.memory.fetch(memoryAccount);
      assert.fail("Memory account should have been closed");
    } catch (error) {
      assert.include(error.message, "Account does not exist");
    }
  });
});
