import { Keypair } from "@solana/web3.js";
import { assert } from "chai";
import { Metadata, createActionClient } from "./utils";

describe("Paths Tests", () => {
  const baseUrl = "http://localhost:3001";
  const aliceKeypair = Keypair.generate();
  const customPathClient = createActionClient(
    `${baseUrl}/api/v1/test/custom_path/${aliceKeypair.publicKey.toBase58()}`
  );
  const customPathMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: "Custom Path",
    description: "Use a custom path configuration",
    label: "Send",
    links: null,
    disabled: false,
    error: null,
  };
  const defaultPathClient = createActionClient(
    `${baseUrl}/api/default_path/${aliceKeypair.publicKey.toBase58()}`
  );
  const defaultPathMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: "Default Path",
    description: "Use the default path configuration",
    label: "Send",
    links: null,
    disabled: false,
    error: null,
  };
  const customPathWithDynamicMetadataClient = createActionClient(
    `${baseUrl}/api/super_custom/custom_path_with_dynamic_metadata`
  );
  const customPathWithDynamicMetadataMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: "Custom Path with Dynamic Metadata",
    description: "Use a custom path configuration with dynamic metadata",
    label: "Send",
    links: null,
    disabled: false,
    error: null,
  };
  const defaultPathWithDynamicMetadataClient = createActionClient(
    `${baseUrl}/api/default_path_with_dynamic_metadata`
  );
  const defaultPathWithDynamicMetadataMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: "Default Path with Dynamic Metadata",
    description: "Use a default path configuration with dynamic metadata",
    label: "Send",
    links: null,
    disabled: false,
    error: null,
  };

  it("should fetch the metadata of the custom path action", async () => {
    const response = await customPathClient.getMetadata();

    assert.equal(response.title, customPathMetadata.title);
    assert.equal(response.description, customPathMetadata.description);
    assert.equal(response.icon, customPathMetadata.icon);
    assert.equal(response.label, customPathMetadata.label);
    assert.equal(response.disabled, customPathMetadata.disabled);
    assert.equal(response.error, customPathMetadata.error);
    assert.equal(response.links, customPathMetadata.links);
  });

  it("should fetch the metadata of the default path action", async () => {
    const response = await defaultPathClient.getMetadata();

    assert.equal(response.title, defaultPathMetadata.title);
    assert.equal(response.description, defaultPathMetadata.description);
    assert.equal(response.icon, defaultPathMetadata.icon);
    assert.equal(response.label, defaultPathMetadata.label);
    assert.equal(response.disabled, defaultPathMetadata.disabled);
    assert.equal(response.error, defaultPathMetadata.error);
    assert.equal(response.links, defaultPathMetadata.links);
  });

  it("should fetch the metadata of the custom path with dynamic metadata action", async () => {
    const response = await customPathWithDynamicMetadataClient.getMetadata();

    assert.equal(response.title, customPathWithDynamicMetadataMetadata.title);
    assert.equal(
      response.description,
      customPathWithDynamicMetadataMetadata.description
    );
    assert.equal(response.icon, customPathWithDynamicMetadataMetadata.icon);
    assert.equal(response.label, customPathWithDynamicMetadataMetadata.label);
    assert.equal(
      response.disabled,
      customPathWithDynamicMetadataMetadata.disabled
    );
    assert.equal(response.error, customPathWithDynamicMetadataMetadata.error);
    assert.equal(response.links, customPathWithDynamicMetadataMetadata.links);
  });

  it("should fetch the metadata of the default path with dynamic metadata action", async () => {
    const response = await defaultPathWithDynamicMetadataClient.getMetadata();

    assert.equal(response.title, defaultPathWithDynamicMetadataMetadata.title);
    assert.equal(
      response.description,
      defaultPathWithDynamicMetadataMetadata.description
    );
    assert.equal(response.icon, defaultPathWithDynamicMetadataMetadata.icon);
    assert.equal(response.label, defaultPathWithDynamicMetadataMetadata.label);
    assert.equal(
      response.disabled,
      defaultPathWithDynamicMetadataMetadata.disabled
    );
    assert.equal(response.error, defaultPathWithDynamicMetadataMetadata.error);
    assert.equal(response.links, defaultPathWithDynamicMetadataMetadata.links);
  });
});
