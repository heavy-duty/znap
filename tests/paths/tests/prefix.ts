import { assert } from "chai";
import { Metadata, createActionClient } from "./utils";

describe("Prefix Tests", () => {
  const baseUrl = "http://localhost:3000";
  const customPrefixActionClient = createActionClient(
    `${baseUrl}/v1-api/custom_prefix`
  );
  const customPrefixActionMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: `Custom prefix`,
    description: `An action with a custom prefix.`,
    label: "Send",
    links: null,
    disabled: false,
    error: null,
  };
  const emptyPrefixActionClient = createActionClient(
    `${baseUrl}/empty_prefix`
  );
  const emptyPrefixActionMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: `Empty prefix`,
    description: `An action with an empty prefix.`,
    label: "Send",
    links: null,
    disabled: false,
    error: null,
  };
  const defaultPrefixActionClient = createActionClient(
    `${baseUrl}/api/default_prefix`
  );
  const defaultPrefixActionMetadata: Metadata = {
    icon: "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    title: `Default prefix`,
    description: `An action with the default prefix.`,
    label: "Send",
    links: null,
    disabled: false,
    error: null,
  };

  it("should fetch the metadata of the custom prefix action", async () => {
    const response = await customPrefixActionClient.getMetadata();

    assert.equal(response.title, customPrefixActionMetadata.title);
    assert.equal(response.description, customPrefixActionMetadata.description);
    assert.equal(response.icon, customPrefixActionMetadata.icon);
    assert.equal(response.label, customPrefixActionMetadata.label);
    assert.equal(response.disabled, customPrefixActionMetadata.disabled);
    assert.equal(response.error, customPrefixActionMetadata.error);
  });

  it("should fetch the metadata of the empty prefix action", async () => {
    const response = await emptyPrefixActionClient.getMetadata();

    assert.equal(response.title, emptyPrefixActionMetadata.title);
    assert.equal(response.description, emptyPrefixActionMetadata.description);
    assert.equal(response.icon, emptyPrefixActionMetadata.icon);
    assert.equal(response.label, emptyPrefixActionMetadata.label);
    assert.equal(response.disabled, emptyPrefixActionMetadata.disabled);
    assert.equal(response.error, emptyPrefixActionMetadata.error);
  });

  it("should fetch the metadata of the default prefix action", async () => {
    const response = await defaultPrefixActionClient.getMetadata();

    assert.equal(response.title, defaultPrefixActionMetadata.title);
    assert.equal(response.description, defaultPrefixActionMetadata.description);
    assert.equal(response.icon, defaultPrefixActionMetadata.icon);
    assert.equal(response.label, defaultPrefixActionMetadata.label);
    assert.equal(response.disabled, defaultPrefixActionMetadata.disabled);
    assert.equal(response.error, defaultPrefixActionMetadata.error);
  });
});
