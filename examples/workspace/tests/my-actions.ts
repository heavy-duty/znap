import {
  Keypair,
  LAMPORTS_PER_SOL,
  SystemInstruction,
  Transaction,
} from "@solana/web3.js";
import { assert } from "chai";
import { Metadata, createClient } from "./utils";

describe("My actions", () => {
  const baseUrl = "http://localhost:3000";
  const znapClient = createClient(baseUrl);
  const bobKeypair = Keypair.generate();
  const actionMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: "Alice's website",
    description: "Website to make a donation to Alice",
    label: "Send",
    links: {
      actions: [
        {
          label: "Send 1 SOL",
          href: "/api/send_donation?amount=1",
          parameters: [],
        },
        {
          label: "Send 5 SOL",
          href: "/api/send_donation?amount=5",
          parameters: [],
        },
        {
          label: "Send SOL",
          href: "/api/send_donation?amount={amount}",
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
    const response = await znapClient.getMetadata("send_donation");

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
    const response = await znapClient.getTransaction(
      "send_donation",
      bobKeypair.publicKey.toBase58(),
      { amount }
    );
    const transaction = Transaction.from(
      Buffer.from(response.transaction, "base64")
    );
    const transferInstruction = SystemInstruction.decodeTransfer(
      transaction.instructions[0]
    );

    assert.isTrue(transferInstruction.fromPubkey.equals(bobKeypair.publicKey));
    assert.isTrue(transferInstruction.fromPubkey.equals(bobKeypair.publicKey));
    assert.equal(
      transferInstruction.lamports,
      BigInt(amount * LAMPORTS_PER_SOL)
    );
  });
});
