import { assert } from "chai";
import { Metadata, createActionClient } from "./utils";

describe("Advanced Inputt Types Tests", () => {
  const baseUrl = "http://localhost:3000";
  const actionClient = createActionClient(`${baseUrl}/api/types`);
  const actionMetadata: Metadata = {
    "icon": "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
    "title": "Advanced Input Types",
    "description": "An action with advanced input types.",
    "label": "Send",
    "links": {
      "actions": [
        {
          "label": "Send",
          "href": "/api/types",
          "parameters": [
            {
              "label": "Text",
              "name": "text",
              "required": false,
              "type": "text",
              "options": []
            },
            {
              "label": "Email",
              "name": "email",
              "required": false,
              "type": "email",
              "options": []
            },
            {
              "label": "Url",
              "name": "url",
              "required": false,
              "type": "url",
              "options": []
            },
            {
              "label": "Number",
              "name": "number",
              "required": false,
              "type": "number",
              "options": []
            },
            {
              "label": "Date",
              "name": "date",
              "required": false,
              "type": "date",
              "options": []
            },
            {
              "label": "Datetime local",
              "name": "datetime-local",
              "required": false,
              "type": "datetime-local",
              "options": []
            },
            {
              "label": "Checkbox",
              "name": "checkbox",
              "required": false,
              "type": "checkbox",
              "options": []
            },
            {
              "label": "Radio",
              "name": "radio",
              "required": false,
              "type": "radio",
              "options": [
                {
                  "label": "Radio 1",
                  "value": "radio-1"
                },
                {
                  "label": "Radio 2",
                  "value": "radio-2"
                }
              ]
            },
            {
              "label": "Radio empty",
              "name": "radio-empty",
              "required": false,
              "type": "radio",
              "options": []
            },
            {
              "label": "Textarea",
              "name": "textarea",
              "required": false,
              "type": "textarea",
              "options": []
            },
            {
              "label": "Select",
              "name": "select",
              "required": false,
              "type": "select",
              "options": [
                {
                  "label": "Select 1",
                  "value": "select-1"
                },
                {
                  "label": "Select 2",
                  "value": "select-2"
                }
              ]
            },
            {
              "label": "Select empty",
              "name": "select-empty",
              "required": false,
              "type": "select",
              "options": []
            }
          ]
        }
      ]
    },
    "disabled": false,
    "error": null
  };

  const defaultTextParameter = {
    "label": "Text",
    "name": "text",
    "required": false,
    "type": "text",
    "options": []
  };

  const defaultEmailParameter = {
    "label": "Email",
    "name": "email",
    "required": false,
    "type": "email",
    "options": []
  };

  const defaultUrlParameter = {
    "label": "Url",
    "name": "url",
    "required": false,
    "type": "url",
    "options": []
  }

  const defaultNumberParameter =             {
    "label": "Number",
    "name": "number",
    "required": false,
    "type": "number",
    "options": []
  }

  const defaultDateParameter =             {
    "label": "Date",
    "name": "date",
    "required": false,
    "type": "date",
    "options": []
  }

  const defaultDatetimeLocalParameter =                 {
    "label": "Datetime local",
    "name": "datetime-local",
    "required": false,
    "type": "datetime-local",
    "options": []
  }

  const defaultCheckboxParameter =              {
    "label": "Checkbox",
    "name": "checkbox",
    "required": false,
    "type": "checkbox",
    "options": []
  }

  const defaultRadioParameter = {
    "label": "Radio",
    "name": "radio",
    "required": false,
    "type": "radio",
    "options": [
      {
        "label": "Radio 1",
        "value": "radio-1"
      },
      {
        "label": "Radio 2",
        "value": "radio-2"
      }
    ]
  }

  const defaultRadioEmptyParameter =             {
    "label": "Radio empty",
    "name": "radio-empty",
    "required": false,
    "type": "radio",
    "options": []
  }

  const defaultTextareaParameter =            {
    "label": "Textarea",
    "name": "textarea",
    "required": false,
    "type": "textarea",
    "options": []
  }

  const defaultSelectParameter ={
    "label": "Select",
    "name": "select",
    "required": false,
    "type": "select",
    "options": [
      {
        "label": "Select 1",
        "value": "select-1"
      },
      {
        "label": "Select 2",
        "value": "select-2"
      }
    ]
  }

  const defaultSelectEmptyParameter =            {
    "label": "Select empty",
    "name": "select-empty",
    "required": false,
    "type": "select",
    "options": []
  }

  const defaultRadioOptions = [
    {
      "label": "Radio 1",
      "value": "radio-1"
    },
    {
      "label": "Radio 2",
      "value": "radio-2"
    }
  ]

  const defaultSelectOptions = [
    {
      "label": "Select 1",
      "value": "select-1"
    },
    {
      "label": "Select 2",
      "value": "select-2"
    }
  ]

  it("should fetch the metadata of the get action", async () => {
    const response = await actionClient.getMetadata();

    assert.equal(response.title, actionMetadata.title);
    assert.equal(response.description, actionMetadata.description);
    assert.equal(response.icon, actionMetadata.icon);
    assert.equal(response.label, actionMetadata.label);
    assert.equal(response.disabled, actionMetadata.disabled);
    assert.equal(response.error, actionMetadata.error);
  });

  it("should fetch the metadata of the parameter: text", async () => {
    const response = await actionClient.getMetadata();
    const textParameter = response.links.actions[0].parameters[0];

    assert.equal(textParameter.label, defaultTextParameter.label);
    assert.equal(textParameter.name, defaultTextParameter.name);
    assert.equal(textParameter.required, defaultTextParameter.required);
    assert.equal(textParameter.type, defaultTextParameter.type);
  });

  it("should fetch the metadata of the parameter: email", async () => {
    const response = await actionClient.getMetadata();
    const emailParameter = response.links.actions[0].parameters[1];

    assert.equal(emailParameter.label, defaultEmailParameter.label);
    assert.equal(emailParameter.name, defaultEmailParameter.name);
    assert.equal(emailParameter.required, defaultEmailParameter.required);
    assert.equal(emailParameter.type, defaultEmailParameter.type);
  });

  it("should fetch the metadata of the parameter: url", async () => {
    const response = await actionClient.getMetadata();
    const urlParameter = response.links.actions[0].parameters[2];

    assert.equal(urlParameter.label, defaultUrlParameter.label);
    assert.equal(urlParameter.name, defaultUrlParameter.name);
    assert.equal(urlParameter.required, defaultUrlParameter.required);
    assert.equal(urlParameter.type, defaultUrlParameter.type);
  });

  it("should fetch the metadata of the parameter: number", async () => {
    const response = await actionClient.getMetadata();
    const numberParameter = response.links.actions[0].parameters[3];

    assert.equal(numberParameter.label, defaultNumberParameter.label);
    assert.equal(numberParameter.name, defaultNumberParameter.name);
    assert.equal(numberParameter.required, defaultNumberParameter.required);
    assert.equal(numberParameter.type, defaultNumberParameter.type);
  });

  it("should fetch the metadata of the parameter: date", async () => {
    const response = await actionClient.getMetadata();
    const dateParameter = response.links.actions[0].parameters[4];

    assert.equal(dateParameter.label, defaultDateParameter.label);
    assert.equal(dateParameter.name, defaultDateParameter.name);
    assert.equal(dateParameter.required, defaultDateParameter.required);
    assert.equal(dateParameter.type, defaultDateParameter.type);
  });

  it("should fetch the metadata of the parameter: datetime-local", async () => {
    const response = await actionClient.getMetadata();
    const datetimeLocalParameter = response.links.actions[0].parameters[5];

    assert.equal(datetimeLocalParameter.label, defaultDatetimeLocalParameter.label);
    assert.equal(datetimeLocalParameter.name, defaultDatetimeLocalParameter.name);
    assert.equal(datetimeLocalParameter.required, defaultDatetimeLocalParameter.required);
    assert.equal(datetimeLocalParameter.type, defaultDatetimeLocalParameter.type);
  });

  it("should fetch the metadata of the parameter: checkbox", async () => {
    const response = await actionClient.getMetadata();
    const checkboxParameter = response.links.actions[0].parameters[6];

    assert.equal(checkboxParameter.label, defaultCheckboxParameter.label);
    assert.equal(checkboxParameter.name, defaultCheckboxParameter.name);
    assert.equal(checkboxParameter.required, defaultCheckboxParameter.required);
    assert.equal(checkboxParameter.type, defaultCheckboxParameter.type);
  });

  it("should fetch the metadata of the parameter: radio", async () => {
    const response = await actionClient.getMetadata();
    const radioParameter = response.links.actions[0].parameters[7];

    assert.equal(radioParameter.label, defaultRadioParameter.label);
    assert.equal(radioParameter.name, defaultRadioParameter.name);
    assert.equal(radioParameter.required, defaultRadioParameter.required);
    assert.equal(radioParameter.type, defaultRadioParameter.type);
  });

  it("should fetch the metadata of the parameter: radio-empty", async () => {
    const response = await actionClient.getMetadata();
    const radioEmptyParameter = response.links.actions[0].parameters[8];

    assert.equal(radioEmptyParameter.label, defaultRadioEmptyParameter.label);
    assert.equal(radioEmptyParameter.name, defaultRadioEmptyParameter.name);
    assert.equal(radioEmptyParameter.required, defaultRadioEmptyParameter.required);
    assert.equal(radioEmptyParameter.type, defaultRadioEmptyParameter.type);
  });

  it("should fetch the metadata of the parameter: textarea", async () => {
    const response = await actionClient.getMetadata();
    const textareaParameter = response.links.actions[0].parameters[9];

    assert.equal(textareaParameter.label, defaultTextareaParameter.label);
    assert.equal(textareaParameter.name, defaultTextareaParameter.name);
    assert.equal(textareaParameter.required, defaultTextareaParameter.required);
    assert.equal(textareaParameter.type, defaultTextareaParameter.type);
  });

  it("should fetch the metadata of the parameter: select", async () => {
    const response = await actionClient.getMetadata();
    const selectParameter = response.links.actions[0].parameters[10];

    assert.equal(selectParameter.label, defaultSelectParameter.label);
    assert.equal(selectParameter.name, defaultSelectParameter.name);
    assert.equal(selectParameter.required, defaultSelectParameter.required);
    assert.equal(selectParameter.type, defaultSelectParameter.type);
  });

  it("should fetch the metadata of the parameter: select-empty", async () => {
    const response = await actionClient.getMetadata();
    const selectEmptyParameter = response.links.actions[0].parameters[11];

    assert.equal(selectEmptyParameter.label, defaultSelectEmptyParameter.label);
    assert.equal(selectEmptyParameter.name, defaultSelectEmptyParameter.name);
    assert.equal(selectEmptyParameter.required, defaultSelectEmptyParameter.required);
    assert.equal(selectEmptyParameter.type, defaultSelectEmptyParameter.type);
  });

  it("should fetch the metadata of the radio options", async () => {
    const response = await actionClient.getMetadata();
    const option1 = response.links.actions[0].parameters[7].options[0];
    const option2 = response.links.actions[0].parameters[7].options[1];

    assert.equal(option1.label, defaultRadioOptions[0].label);
    assert.equal(option1.value, defaultRadioOptions[0].value);

    assert.equal(option2.label, defaultRadioOptions[1].label);
    assert.equal(option2.value, defaultRadioOptions[1].value);
  });

  it("should fetch the metadata of the select options", async () => {
    const response = await actionClient.getMetadata();
    const option1 = response.links.actions[0].parameters[10].options[0];
    const option2 = response.links.actions[0].parameters[10].options[1];

    assert.equal(option1.label, defaultSelectOptions[0].label);
    assert.equal(option1.value, defaultSelectOptions[0].value);

    assert.equal(option2.label, defaultSelectOptions[1].label);
    assert.equal(option2.value, defaultSelectOptions[1].value);
  });
});
