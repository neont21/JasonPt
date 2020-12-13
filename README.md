Jason Bot JasonPt#2964
---
### Environment Variables
- `DISCORD_TOKEN` : Bot API Token

### Features
- PREFIX: Bot Mention

#### `ping` is work!    
`ping` -> "Ping!"
#### `about` is work!    
`about` -> introduce itself
#### `send` is work!    
requires argument of JSON data    
- `content` : (string) non-embed text (if you don't want it, just send empty string)
- `title` : (string) title of the embed
- `description` : (string) description of the embed
- `colour` : (integer) the left-side color of the embed
- `fields` : (array of [String, String, Boolean]) the fields of the embed

sample JSON data
```json
{
	"content" : "the JSON data for the test",
	"title" : "The Test",
	"description": "This is a test data for embedding",
	"colour": 14501908,
	"fields": [
		["title1", "content1", true], ["title2", "content2", true], ["title3", "content3", false]
	]
}
```

#### `bind` has to implemented.    
