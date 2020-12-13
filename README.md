Jason Bot JasonPt#2964
---
### Environment Variables
- `DISCORD_TOKEN` : Bot API Token

### Features
- PREFIX: Bot Mention

#### `ping`
`ping` -> "Ping!"

#### `about`
`about` -> introduce itself

#### `send`
requires argument of JSON data    
- `content` : (string) non-embed text (if you don't want it, just send empty string)
- `title` : (string) title of the embed
- `description` : (string) description of the embed
- `colour` : (integer) the left-side color of the embed
- `fields` : (array of [String, String, Boolean]) the fields of the embed
- `bind` : (string) the channel to send message. default or channel tag (requires quotation marks)

sample JSON data
```json
{
	"content" : "the JSON data for the test",
	"title" : "The Test",
	"description": "This is a test data for embedding",
	"colour": 14501908,
	"fields": [
		["title1", "content1", true], ["title2", "content2", true], ["title3", "content3", false]
	],
	"bind": "default"
}
```

#### `say`
requires argument of JSON data
- `content` : (string) non-embed text (if you don't want it, just send empty string)
- `bind` : (string) the channel to send message. default or channel tag (requires quotation marks)

sample JSON data
```json
{
	"content" : "the JSON data for the test",
	"bind": "default"
}
```

#### `react`
requires argument of JSON data
- `c_id` : (integer) channel ID
- `m_id` : (integer) message ID
- `reactions` : (array of String) Emoji

sample JSON data
```json
{
"c_id" : 773540505266421812,
"m_id" : 787729895257931837,
"reactions" : [":new_moon:", ":last_quarter_moon:", ":full_moon:", ":boom:"]
}
```
