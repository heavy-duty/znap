# Deploy

The following tutorial will allow you to learn how to deploy your Solana Action API on Shuttle, so that it is available to everyone and not just in your local environment.

## Before you start

- You must have an account created on [Shuttle.rs](https://www.shuttle.rs/).
- You must have a project created on Shuttle.rs, which will be where we deploy our Solana Action API.

## Let's start

In this tutorial, we will assume that you have already created your Znap project with a collection. The command we will use to deploy our project will be `znap deploy <NAME> <PROJECT>`, where:

- `<NAME>`: The name of the collection
- `<PROJECT>`: The name of the project in shuttle

Let's suppose we have a collection called alice-donation and a project called alice-solana-action-api in Shuttle. Once we have tested our project in our local environment to ensure that everything works correctly, we use the following command: `znap deploy alice-donation alice-solana-action-api`.

If you used the correct name for your collection and the correct name for your project in Shuttle, the deployment of your project will start and, in the end, you will get an output similar to the following:

```bash
Service Name:  alice-solana-action-api
Deployment ID: 6f52214b-34cd-4010-8d53-073254f721b7
Status:        running
Last Updated:  2024-07-11T19:09:01Z
URI:           https://alice-solana-action-api.shuttleapp.rs
```

## Let's test our Solana Action API

Once our Solana Action API is deployed, we can test it from anywhere.

For example, in this case, it would be as follows and we would get the following outputs:

**GET REQUEST**

`https://alice-solana-action-api.shuttleapp.rs/api/send_donation?amount=1`

**GET RESPONSE**

```json
{
	"icon": "https://media.discordapp.net/attachments/1205590693041541181/1212566609202520065/icon.png?ex=667eb568&is=667d63e8&hm=0f247078545828c0a5cf8300a5601c56bbc9b59d3d87a0c74b082df0f3a6d6bd&=&format=webp&quality=lossless&width=660&height=660",
	"title": "Send a Donation to ",
	"description": "Send a donation to  using the Solana blockchain via a Blink.",
	"label": "Send",
	"links": {
		"actions": [
			{
				"label": "Send 1 SOL",
				"href": "/api/send_donation/?amount=1",
				"parameters": []
			},
			{
				"label": "Send 5 SOL",
				"href": "/api/send_donation/?amount=5",
				"parameters": []
			},
			{
				"label": "Send SOL",
				"href": "/api/send_donation/?amount={amount}",
				"parameters": [
					{
						"label": "Amount in SOL",
						"name": "amount",
						"required": false
					}
				]
			}
		]
	},
	"disabled": false,
	"error": null
}
```

**POST REQUEST**

`POST: https://alice-solana-action-api.shuttleapp.rs/api/send_donation?amount=1`

**POST RESPONSE**

```json
{
	"transaction": "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAQGcPdkuJpytnAJJPkZPmoN+AHwIoLTv85WOfeT7VJ+VQHWV5/VRU4sBfwjzVndavO5L5IxKP6guUf3b93KbXC/6gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABUpTWpkpIQZNJOhxYNo4fHw1td28kruB5B+oQEEFRI2Y26AfwWEGT+p+iAFVh86xILWAkHmMctnm4Cbg/dzDItPoMEP0Zt7vHw302NaG7VuP+ergq0lvUow2gAmwaRAZAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAgQAAQQFDAIAAAAAypo7AAAAAAMAwAFzb2xhbmEtYWN0aW9uOkZHQ05kYlAxZkNjcWF0Q3pERzdMS013eHNpQkVjemYyQ0x5OU1UTUt0cVoyOkJIaEQ0WXJaUjFUWVh2c0FmYnFBYjdiTlZmS1lwRVR6N3VVUjJEeXVtWjlzOjIyYnBEY0tDVG0yZ1VoSEVlakFOQWpRcTM5SDUxRmd1QmFkM3BWNnRZWFMyZzRuSkZ3R0NzVExpZkxOQ25DTVBZNHNOOG91RkE1Rnh2WVlYUTJaTXZReDc=",
	"message": "send donation to alice"
}
```