import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemInstruction,
  Transaction,
} from "@solana/web3.js";
import { assert } from "chai";
import { Metadata, createActionClient } from "./utils";
import nacl from "tweetnacl";
import bs58 from "bs58";

describe("Prefix Tests", () => {
  const baseUrl = "http://localhost:3000";
  const aliceKeypair = Keypair.generate();
  const bobKeypair = Keypair.generate();
  const sendDonationClient = createActionClient(
    `${baseUrl}/v1-api/send_donation/${aliceKeypair.publicKey.toBase58()}`
  );
  const actionMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: `Send a Donation to ${aliceKeypair.publicKey.toBase58()}`,
    description: `Send a donation to ${aliceKeypair.publicKey.toBase58()} using the Solana blockchain via a Blink.`,
    label: "Send",
    links: {
      actions: [
        {
          label: "Send 1 SOL",
          href: `/v1-api/send_donation/${aliceKeypair.publicKey.toBase58()}?amount=1`,
          parameters: [],
        },
        {
          label: "Send 5 SOL",
          href: `/v1-api/send_donation/${aliceKeypair.publicKey.toBase58()}?amount=5`,
          parameters: [],
        },
        {
          label: "Send SOL",
          href: `/v1-api/send_donation/${aliceKeypair.publicKey.toBase58()}?amount={amount}`,
          parameters: [
            {
              label: "Amount in SOL",
              name: "amount",
            },
          ],
        },
      ],
    },
    disabled: false,
    error: null,
  };

  it("should fetch the metadata of the send donation action", async () => {
    const response = await sendDonationClient.getMetadata();

    assert.equal(response.title, actionMetadata.title);
    assert.equal(response.description, actionMetadata.description);
    assert.equal(response.icon, actionMetadata.icon);
    assert.equal(response.label, actionMetadata.label);
    assert.equal(response.disabled, actionMetadata.disabled);
    assert.equal(response.error, actionMetadata.error);

    response.links.actions.forEach((link, linkIndex) => {
      assert.equal(link.href, actionMetadata.links.actions[linkIndex].href);
      assert.equal(link.label, actionMetadata.links.actions[linkIndex].label);

      actionMetadata.links.actions[linkIndex].parameters.forEach(
        (parameter, parameterIndex) => {
          assert.equal(
            parameter.label,
            actionMetadata.links.actions[linkIndex].parameters[parameterIndex]
              .label
          );
          assert.equal(
            parameter.name,
            actionMetadata.links.actions[linkIndex].parameters[parameterIndex]
              .name
          );
        }
      );
    });
  });

  it("should create a valid send donation transaction", async () => {
    const amount = 5_000;
    const response = await sendDonationClient.getTransaction(
      bobKeypair.publicKey.toBase58(),
      { amount }
    );
    const transaction = Transaction.from(
      Buffer.from(response.transaction, "base64")
    );

    // check the transfer instruction
    const transferInstruction = SystemInstruction.decodeTransfer(
      transaction.instructions[0]
    );

    assert.isTrue(transferInstruction.fromPubkey.equals(bobKeypair.publicKey));
    assert.isTrue(transferInstruction.toPubkey.equals(aliceKeypair.publicKey));
    assert.equal(
      transferInstruction.lamports,
      BigInt(amount * LAMPORTS_PER_SOL)
    );

    // Verify there's a memo with a valid identity message
    assert.isTrue(
      transaction.instructions[1].programId.equals(
        new PublicKey("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")
      )
    );

    const [, identityAddress, referenceAddress, identitySignature] =
      transaction.instructions[1].data.toString("utf-8").split(":");

    const identityPublicKey = new PublicKey(identityAddress);
    const referencePublicKey = new PublicKey(referenceAddress);

    assert.isTrue(
      nacl.sign.detached.verify(
        referencePublicKey.toBytes(),
        bs58.decode(identitySignature),
        identityPublicKey.toBytes()
      )
    );

    // Verify the reference and identity are readonly non-signer keys of the transfer
    assert.isTrue(
      referencePublicKey.equals(transaction.instructions[0].keys[2].pubkey)
    );
    assert.isTrue(
      identityPublicKey.equals(transaction.instructions[0].keys[3].pubkey)
    );
  });
});
