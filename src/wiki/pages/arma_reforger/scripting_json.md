---
title: Arma Reforger:Scripting: JSON
url: https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_JSON
---

## JSON Format

**JSON** stands for **J**ava**S**cript **O**bject **N**otation. It is a way to present data in a simple form.

**Note:** See:
- https://en.wikipedia.org/wiki/JSON for a more informative Wikipedia article
- [RFC7159](https://datatracker.ietf.org/doc/html/rfc7159) for the RFC
- [ECMA-404](https://www.ecma-international.org/publications-and-standards/standards/ecma-404/) for the standard
- https://www.json.org/ for general details and libraries
- [Mozilla's article](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/JSON) for more information.

The parent class is always an object (the **O** from JSON).

```json
{
}
```

A property is defined with its name on the left of the colon, and its value on the right. **Properties and strings** are defined by **double-quotes** `"`.<br>
If another property follows, a comma is required. If no other property follows, **there must be no comma**.

**Important:** There can be **no comments** in a JSON file.

```json
{
	"property": 42
}
```

```json
{
	"property1": 41,
	"property2": "fourty-two"
}
```

Spacing does not matter outside of the quotes.

```json
{
	"property1": "name",
	"property2": 42
}
```

is identical to

```json
{"property1":"name","property2":42}
```

but for obvious readability reasons, tabs and spaces are welcome.

### Boolean

A boolean can be either `true` or `false` and nothing else.

```json
{
	"boolean1": true,
	"boolean2": false
}
```

### Number

A number can be either an integer or a float. There are no quotes around it. It cannot start with a period `.`. Scientific notations are accepted.

```json
{
	"number1": 42,
	"number2": 4.2,
	"number3": -33,
	"number4": 10e10
}
```

### String

Line returns cannot happen in JSON strings. The following character escape sequences are accepted:

- `\b`
- `\f`
- `\r`
- `\n`
- `\t`
- `\"`
- `\\`
- `\/`
- `\u####` - see [Unicode chars](https://en.wikipedia.org/wiki/List_of_Unicode_characters)

```json
{
	"string1": "This is a string",
	"string2": "Is \"this\" a string?\tYes, Sir\u0021"
}
```

### Object

```json
{
	"object1": {
		"property1": "sub-object"
	}
}
```

The null value is covered.

```json
{
	"object1": null
}
```

### Array

An array can contain anything: direct values, or other objects with named properties.

```json
{
	"array1": [
		4,
		5,
		6
	],
	"array2": ["This", "is", "Sparta!"],
	"array3": [
		{
			"name": "object's name",
			"value": 33
		},
		{ "name": "another object, inlined", "value": 42 }
	]
}
```

## See Also

- [Arma Reforger:JsonApiStruct Usage](https://community.bistudio.com/wiki/Arma_Reforger:JsonApiStruct_Usage) - covers JSON serialisation, deserialisation, validation
- [Arma Reforger:Serialisation#JSON](https://community.bistudio.com/wiki/Arma_Reforger:Serialisation#JSON)
