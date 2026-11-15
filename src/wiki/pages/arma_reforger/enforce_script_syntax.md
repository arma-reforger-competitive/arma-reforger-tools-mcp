---
title: Arma Reforger:Enforce Script Syntax
url: https://community.bistudio.com/wiki/Arma_Reforger:Enforce_Script_Syntax
---

**Enforce Script** is the language that is used by the [Enfusion](https://community.bistudio.com/wiki/Enfusion) engine first introduced in DayZ Standalone.
It is an Object-Oriented scripting language that works with objects and classes and is similar to the [C#](https://en.wikipedia.org/wiki/C_Sharp_(programming_language)) programming language.

**DayZ:** For DayZ, see [DayZ:Enforce Script Syntax](https://community.bistudio.com/wiki/DayZ:Enforce_Script_Syntax).

## Data Types

**Note:** See [Arma Reforger:Scripting: Values#Types](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Values#Types).

There are many types of data, the most common being:

| Type | Description | Example | Wikipedia |
| --- | --- | --- | --- |
| **Native types** |  |  |  |
| `bool` | a bool, a.k.a `true` or `false` (and nothing else!) | `bool value = true;` | see [Bool](https://en.wikipedia.org/wiki/Boolean_data_type) |
| `int` | an integer, a.k.a a whole number e.g 1, 42, -10, etc. | `int value = 42;` | see [Integer](https://en.wikipedia.org/wiki/Integer_(computer_science)) |
| `float` | a floating point value, a.k.a a partial number e.g 1.0, 4.2, -0.1, etc | `float value = 0.333;` | see [Float](https://en.wikipedia.org/wiki/Floating-point_arithmetic) |
| `string` | a text value, a.k.a a sequence of characters, e.g "Hello there" | `string owk = "Hello" + " " + "there";` | see [String](https://en.wikipedia.org/wiki/String_(computer_science)) |
| `vector` | an array of three float values e.g `{ 1.0, 42.2, 66.6 }` | `vector position = { 5, 1.5, 10 };` | see [Vector](https://en.wikipedia.org/wiki/Euclidean_vector) |
| **Objects** |  |  |  |
| `class` | an object able to hold properties and methods. | see [Arma Reforger:Object Oriented Programming Basics](https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Basics) | see [Class](https://en.wikipedia.org/wiki/Class_(programming)) |
| `enum` | a structure having static values "listed" as its properties.<br>**Note:** An enum is not an object *per se*. | `SCR_EBloodType myBloodType = SCR_EBloodType.O_POSITIVE;` | see [Enum](https://en.wikipedia.org/wiki/Enumerated_type) |
| static array | a **static** array of elements (of *a* type) - it cannot contain a mix of types.<br>**Important:** Arrays (dynamic or static) are the only list type that can be initialised in line. | `string helloThereWords[] = { "Hello", "there" };` | see [Array](https://en.wikipedia.org/wiki/Array_(data_type)) |
| `array<x>` | a **dynamic** array of elements (of *a* type) - it cannot contain a mix of types.<br>**Important:** Arrays (dynamic or static) are the only list type that can be initialised in line. | `array<string> helloThereWords = { "Hello", "there" };` | see [Array](https://en.wikipedia.org/wiki/Array_(data_type)) |
| `set<x>` | a set of ***unique*** elements, it can only contain one type of elements.<br>**Important:** Values are **not** stored in insertion order. | `set<string> helloThereWords = new set<string>();`<br>`helloThereWords.Insert("Hello");`<br>`helloThereWords.Insert("there");`<br>`helloThereWords.Insert("Hello");	// ignored`<br>`Print(helloThereWords.Count());		// prints '2'` | see [Set](https://en.wikipedia.org/wiki/Set_(abstract_data_type)) |
| `map<x, y>` | a map constituted of key-value pairs, where the key aspect is used to obtain the value.<br>**Important:** Values are **not** stored in insertion order. | `map<string, int> wordsAndLength = new map<string, int>();`<br>`wordsAndLength.Insert("General", 7);`<br>`wordsAndLength.Insert("Kenobi", 6);`<br>`Print(replyWordsAndLength.Get("Kenobi")); // prints '6'` | see [Map](https://en.wikipedia.org/wiki/Map_%28higher-order_function%29) |

## Object-Oriented Programming

See [Arma Reforger:Object Oriented Programming Basics](https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Basics) and [Arma Reforger:Object Oriented Programming Advanced Usage](https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Advanced_Usage).

## Operations

### Assignation

Assignation operations convert the value to the expected type; see below:

```enforce
// adding int and int
int result = 1 + 2;			// result == 3

// adding float and float
float result = 1.25 + 2.75;	// result == 4

// adding int and float
int result = 1 + 0.9;			// result == 1 - int ''truncates'' a float result
int result = 0.9 + 0.9 + 0.9;	// result == 2 - 2.7 truncated to 2
float result = 0.9 + 0.9 + 0.9;	// result == 2.7
float result = 1 + 1.9;			// result == 2.9
float result = 1.9 + 1;			// result == 2.9

bool invalid = new SCR_Ray();	// error: Types 'SCR_Ray' and 'bool' are unrelated
SCR_Ray instance = new SCR_Ray();
bool valid = instance;			// valid == true
bool result = 42;				// result == true
```

### Logic

```enforce
SCR_Ray instance = new SCR_Ray();
if (instance)	// identical to "instance != null"
	Print("Instance exists");
else
	Print("Instance does not exists");

string text;
if (text)		// identical to "!text.IsEmpty()"
	Print("Text is not empty");
else
	Print("Text is empty");

int val = 42;	// same with float
if (val)		// identical to "val != 0"
	Print("Val is not zero");
else
	Print("Val is zero");
```

**Important:** `if (intValue)` and `if (stringValue)` are **not** recommended by [Arma Reforger:Scripting: Best Practices](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Best_Practices).
