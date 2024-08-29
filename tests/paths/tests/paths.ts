import { Keypair } from "@solana/web3.js";
import { assert } from "chai";
import { Metadata, createActionClient } from "./utils";

describe("Paths Tests", () => {
  const baseUrl = "http://localhost:3001";
  const aliceKeypair = Keypair.generate();
  const mintKeypair = Keypair.generate();
  const customPathSingleParameterClient = createActionClient(
    `${baseUrl}/api/v1/test/custom_path_single_parameter/${aliceKeypair.publicKey.toBase58()}`
  );
  const customPathSingleParameterMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: "Custom Path with Single Paramater",
    description: "Use a custom path configuration with a single parameter",
    label: "Send",
    links: null,
    disabled: false,
    error: null,
  };
  const defaultPathMultiParameterClient = createActionClient(
    `${baseUrl}/api/default_path_multi_parameter/${mintKeypair.publicKey.toBase58()}/${aliceKeypair.publicKey.toBase58()}`
  );
  const defaultPathMultiParameterMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: "Default Path with Multiple Paramaters",
    description: "Use the default path configuration with multiple parameters",
    label: "Send",
    links: null,
    disabled: false,
    error: null,
  };

  it("should fetch the metadata of the custom path and single parameter action", async () => {
    const response = await customPathSingleParameterClient.getMetadata();

    assert.equal(response.title, customPathSingleParameterMetadata.title);
    assert.equal(
      response.description,
      customPathSingleParameterMetadata.description
    );
    assert.equal(response.icon, customPathSingleParameterMetadata.icon);
    assert.equal(response.label, customPathSingleParameterMetadata.label);
    assert.equal(response.disabled, customPathSingleParameterMetadata.disabled);
    assert.equal(response.error, customPathSingleParameterMetadata.error);
    assert.equal(response.links, customPathSingleParameterMetadata.links);
  });

  it("should fetch the metadata of the default path and multiple parameters action", async () => {
    const response = await defaultPathMultiParameterClient.getMetadata();

    assert.equal(response.title, defaultPathMultiParameterMetadata.title);
    assert.equal(
      response.description,
      defaultPathMultiParameterMetadata.description
    );
    assert.equal(response.icon, defaultPathMultiParameterMetadata.icon);
    assert.equal(response.label, defaultPathMultiParameterMetadata.label);
    assert.equal(response.disabled, defaultPathMultiParameterMetadata.disabled);
    assert.equal(response.error, defaultPathMultiParameterMetadata.error);
    assert.equal(response.links, defaultPathMultiParameterMetadata.links);
  });
});
