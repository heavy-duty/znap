import {
  Keypair,
  LAMPORTS_PER_SOL,
  SystemInstruction,
  Transaction
} from "@solana/web3.js";
import { assert } from "chai";

interface Action {
  label: string;
  href: string;
  parameters: {
    label: string;
    name: string;
  }[];
}

interface Metadata {
  icon: string;
  title: string;
  description: string;
  label: string;
  disabled: boolean;
  error: null;
  links: { actions: Action[] };
}

describe("My actions", () => {
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

  const baseUrl = "http://localhost:3000";

  const bobKeypair = Keypair.generate();

  it("should fetch the metadata of the send donation action", async () => {
    const url = new URL(`${baseUrl}/api/send_donation`);
    const response = await fetch(url.toString(), {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });
    const responseJson = (await response.json()) as Metadata;

    assert.equal(responseJson.title, actionMetadata.title);
    assert.equal(responseJson.description, actionMetadata.description);
    assert.equal(responseJson.icon, actionMetadata.icon);
    assert.equal(responseJson.label, actionMetadata.label);
    assert.equal(responseJson.disabled, actionMetadata.disabled);
    assert.equal(responseJson.error, actionMetadata.error);

    responseJson.links.actions.forEach((link, linkIndex) => {
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
    const url = new URL(`${baseUrl}/api/send_donation`);

    url.searchParams.set("amount", amount.toString());

    const response = await fetch(url.toString(), {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ account: bobKeypair.publicKey.toBase58() }),
    });
    const responseJson = (await response.json()) as { transaction: string };

    const transaction = Transaction.from(
      Buffer.from(responseJson.transaction, "base64")
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
