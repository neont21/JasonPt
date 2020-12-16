Jason Bot JasonPt#2964
---
Pt-customized Discord Bot
Hosting for Kuding

### Environment Variables
- `DISCORD_TOKEN` : Bot API Token

### Features
- PREFIX: Bot Mention

#### `ping`
`ping` -> "Pong!"

#### `about`
`about` -> introduce itself

also called as `자기소개`

#### `send`
sends the embed message to the channel    
requires argument of JSON data    
- `content` : (string) non-embed text (if you don't want it, just send empty string)
- `title` : (string) title of the embed
- `description` : (string) description of the embed
- `colour` : (integer) the left-side color of the embed
- `fields` : (array of [String, String, Boolean]) the fields of the embed
- `bind` : (string) the channel to send message. default or channel tag (requires quotation marks)

also called by `임베드`    

sample JSON data
```json
{
	"content" : "the JSON data for the modify test",
	"title" : "The Test",
	"description": "This is a test data for embedding",
	"colour": 14501908,
	"fields": [
		["title1", "content1", true], ["title2", "content2", true], ["title3", "content3", false]
	],
	"bind": "default"
}
```

if you want to modify the message, use `send modify`    
which requires message ID
- `m_id` : (integer) message ID to modify

also called by `임베드 수정`

sample JSON data
```json
{
	"m_id" : 787729895257931837,
	"content" : "the JSON data for the test",
	"title" : "The Test is modified!",
	"description": "This is a test data for modifying",
	"colour": 14501908,
	"fields": [
		["title1", "content1", false], ["title2", "content2", false]
	],
	"bind": "default"
}
```

#### `say`
sends the text messaeg to the channel    
requires argument of JSON data
- `content` : (string) non-embed text (if you don't want it, just send empty string)
- `bind` : (string) the channel to send message. default or channel tag (requires quotation marks)

also called by `텍스트`

sample JSON data
```json
{
	"content" : "the JSON data for the test",
	"bind": "default"
}
```

if you want to modify the message, use `say modify`    
which requires message ID
- `m_id` : (integer) message ID to modify

also called by `텍스트 수정`

sample JSON data
```json
{
	"m_id" : 787729895257931837,
	"content" : "the JSON data for the modify test",
	"bind": "default"
}
```

#### `react`
reacts to the message by emoji
requires argument of JSON data
- `bind` : (string) the channel in which the message be
- `m_id` : (integer) message ID
- `reactions` : (array of String) Emoji

also called by `반응`

sample JSON data
```json
{
	"bind" : "default",
	"m_id" : 787729895257931837,
	"reactions" : [":new_moon:", ":last_quarter_moon:", ":full_moon:", ":boom:"]
}
```

if you want to remove the reaction, use `react remove`    
emoji that isn't used will be ignored    
use same JSON format as `react`

also called by `반응 해제`

sample JSON data
```json
{
	"bind" : "default",
	"m_id" : 787729895257931837,
	"reactions" : [":last_quarter_moon:", ":full_moon:"]
}
```

#### `delete`
deletes the message    
requires argument of JSON data
- `bind` : (string) the channel in which the message be
- `m_id` : (integer) message ID

also called by `삭제`

sample JSON data
```json
{
	"bind" : "default",
	"m_id" : 787729895257931837
}
```
