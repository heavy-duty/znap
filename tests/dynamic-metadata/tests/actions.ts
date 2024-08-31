import { assert } from "chai";
import { Metadata, createActionClient } from "./utils";
import { Keypair } from "@solana/web3.js";

describe("Dynamic Metadata Tests", () => {
  const baseUrl = "http://localhost:3000";
  const actionClient = createActionClient(`${baseUrl}/api/dynamic_metadata`);
  const actionMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: "Dynamic Metadata Action",
    description: "An action with dynamic metadata",
    label: "Send",
    links: null,
    disabled: false,
    error: null,
  };

  it("should fetch the metadata of the get action", async () => {
    const response = await actionClient.getMetadata();

    assert.equal(response.title, actionMetadata.title);
    assert.equal(response.description, actionMetadata.description);
    assert.equal(response.icon, actionMetadata.icon);
    assert.equal(response.label, actionMetadata.label);
    assert.equal(response.disabled, actionMetadata.disabled);
    assert.equal(response.error, actionMetadata.error);
  });
});
