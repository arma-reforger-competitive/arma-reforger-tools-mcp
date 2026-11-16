---
title: Arma Reforger:Scripting First Steps
url: https://community.bistudio.com/wiki/Arma_Reforger:Scripting_First_Steps
---

Welcome to Enforce Script! This guide will get you writing and testing your first lines of code in 5 minutes.

**Note:** This is a hands-on quick start. Skip the theory, start coding immediately!

## Requirements

- **Arma Reforger Tools** (Arma Reforger Workbench) installed via Steam
- That's it!

## Open Workbench and Remote Console

1. Launch **Enfusion Workbench** from Steam (Arma Reforger Tools)
1. Click on the **Script Editor** icon or select it from the menu<br>
1. If the **Remote Console** window is not visible, open it from the menu
1. Click on the **Remote Console** and **Output** tabs at the bottom to switch to the appropriate view<br>
1. You are ready to start coding!

**Note:** For these basic exercises, you do not need to load a world or run the game. The Remote Console can execute simple code directly.

## First Line of Code

Type the following in the **Remote Console** and click the **Run** button at the top of the remote console panel:

The `Print` method prints (as its name states) the provided arguments into the log console:

```enforce
Print("Hello there!"); // displays "Hello there!" in the log console
```

**You did it!** You just wrote your first Enforce Script code. The text appears in the console output.

`PrintFormat` allows for printing a string with arguments:

```enforce
PrintFormat("Hello %1, welcome to %2!", "there", "Arma Reforger");	// %1 is replaced by "there" (the first argument), %2 by "Arma Reforger
PrintFormat("Hello %1, welcome to %2!", "you", "Enfusion");			// result here is "Hello you, welcome to Enfusion!"
```

**Important:** A percent sign is printed without issue using `Print`, however it requires to be doubled with `PrintFormat`, being a special character to this function:

```enforce
Print("5%");			// prints "5%"
Print("%%");			// prints "%%"

PrintFormat("%1", 5);	// prints "5"
PrintFormat("5%");		// prints "5"
PrintFormat("5%%");		// prints "5%"
PrintFormat("%1%%", 5);	// prints "5%"
```

## Basic Maths

Try these commands one by one in the Remote Console:

```enforce
Print(10 + 5);
Print(20 * 3);
Print(100 / 4);
```

The console shows: `15`, `60`, `25`.

## Variables

Variables let you store and reuse values:

```enforce
int myAge = 25;
Print("I am " + myAge + " years old");
```

Try changing the number and run it again!

**Common variable types:**
- `int` - whole numbers (1, 42, -5)
- `float` - decimal numbers (3.14, 10.5)
- `string` - text ("Hello", "Arma")
- `bool` - true or false

**Note:** For a complete list of all data types, see [Scripting Values](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Values).

```enforce
float distance = 150.5;
string playerName = "Soldier";

PrintFormat("%1 is at %2m", playerName, distance);
```

## Decisions

```enforce
int health = 75;

if (health > 50)
	Print("Healthy!");
else
	Print("Need medical attention");
```

Try changing `health` to different numbers!

## Arrays

Arrays can be seen as lists that store naught to multiple values **of the same type**:

```enforce
array<string> soldiers = { "Alpha", "Bravo", "Charlie" };

Print(soldiers[0]); // Prints "Alpha"
Print(soldiers[1]); // Prints "Bravo"

soldiers.Insert("Delta"); // soldiers is now { "Alpha", "Bravo", "Charlie", "Delta" }
Print("Squad size: " + soldiers.Count()); // Squad size: 4
```

## Loops

### For

A `for` loop repeats instructions a specific number of times:

```enforce
for (int i; i < 5; i++)
{
	Print("Count: " + i); // prints ("Count: ") 0, 1, 2, 3, 4 then leaves (i < 5)
}
```

### Foreach

A `foreach` loop goes through all array items:

```enforce
array<string> weapons = { "Rifle", "Pistol", "Grenade" };

foreach (string weapon : weapons)
{
	Print("Weapon: " + weapon); // prints "Rifle", "Pistol", "Grenade"
}
```

The index can also be obtained this way:

```enforce
foreach (int i, string weapon : weapons)
```

## A First Mini-Program

Combine what you learned! Copy this entire block and run it:

```enforce
// Soldier health checker
array<string> soldierNames = { "Alpha", "Bravo", "Charlie", "Delta" };
array<int> healthValues = { 100, 45, 80, 20 };

Print("=== Squad Status Report ===");

foreach (int i, string soldierName : soldierNames)
{
	int health = healthValues[i];

	if (health > 70)
		PrintFormat("%1: Healthy (%2%%)", soldierName, health);
	else if (health > 30)
		PrintFormat("%1: Injured (%2%%)", soldierName, health);
	else
		PrintFormat("%1: Critical! (%2%%)", soldierName, health);
}

Print("=== End of Report ===");
```

**Note:** Lines starting with `//` are **comments** - they are ignored by the code but help explain what is happening.

## Quick Reference Card

| Category | Code | Description |
| --- | --- | --- |
| **Output** | `Print("text");` | Shows text in console |
| **Variables** | `int number = 42;` | Stores a whole number |
| **Variables** | `float decimal = 3.14;` | Stores a decimal number |
| **Variables** | `string text = "Hi";` | Stores text |
| **Variables** | `bool flag = true;` | Stores true/false |
| **Arrays** | `array<int> numbers = { 1, 2, 3 };` | Creates a list |
| **Arrays** | `myArray.Insert(value);` | Adds item to list |
| **Arrays** | `myArray.Count();` | Gets list size |
| **If/Else** | `if (condition)`<br>`{`<br>`}` | Do something if true |
| **If/Else** | `else`<br>`{`<br>`}` | Do something if false |
| **Loops** | `for (int i; i < 10; i++)`<br>`{`<br>`}` | Repeat 10 times |
| **Loops** | `foreach (int item : myArray)`<br>`{`<br>`}` | Execute for each item |
| **Math** | `+` | Add |
| **Math** | `-` | Subtract |
| **Math** | `*` | Multiply |
| **Math** | `/` | Divide |
| **Compare** | `==` | Equal to |
| **Compare** | `!=` | Not equal to |
| **Compare** | `>` | Greater than |
| **Compare** | `<` | Less than |
| **Compare** | `>=` | Greater or equal |
| **Compare** | `<=` | Less or equal |

## Common Mistakes

| Incorrect | Correct | Explanation |
| --- | --- | --- |
| `print("hello")` | `Print("hello")` | Capital P in Print |
| `int x = 5` | `int x = 5;` | Missing semicolon |
| `if (x = 5)` | `if (x == 5)` | Use == for comparison, = for assignment |
| `int firstElement = myArray[1];` | `int firstElement = myArray[0];` | Arrays start at 0, not 1 |
| `string name = John` | `string name = "John"` | Text needs quotes |

**Note:** Follow professional coding standards: [Scripting Conventions](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Conventions).

## Exercises

The following challenges help in practicing and understanding scripting.

### Temperature Converter

Create a program that converts 25°C to Fahrenheit.

```enforce
// Convert Celsius to Fahrenheit
float celsius = 25;
float fahrenheit = (celsius * 9 / 5) + 32;
PrintFormat("%1°C = %2°F", celsius, fahrenheit);
```

### Countdown

Create a countdown from 10 to 0, then print "Launch!".

```enforce
// Count down from 10 to 0
for (int i = 10; i >= 0; i--)
{
	Print(i);
}

Print("Launch!");
```

### Squad Filter

Print only the soldiers with health above 50 from two arrays (names and health values).

```enforce
// Only print soldiers with health above 50
array<string> names = { "Alpha", "Bravo", "Charlie" };
array<int> health = { 80, 30, 90 };

foreach (int i, string name : names)
{
	if (health[i] > 50)
		Print(name + " is combat ready");
}
```

## Troubleshooting

**Console will not open?**
- Make sure the **Script Editor** is opened in Workbench
- Look for the **Remote Console** panel - if hidden, open it from the menu
- Ensure Workbench is properly installed via Steam (Arma Reforger Tools)

**Code does not work?**
- Check for missing semicolons `;`
- Make sure quotes match: `"text"`
- Check spelling and capitalisation (casing matters: `print("ok");` is different from `Print("ok");`)
- Look for error messages in red

**Getting errors?**
- Read the error message - it often tells you what is wrong
- Click the error message and check reported line number(s)
- Make sure all open brackets `{ }` are closed

## See Also

**Reference Documentation:**
- [Scripting Values](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Values) - All data types in detail
- [Scripting Conventions](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Conventions) - Bohemia Interactive coding standards
- [OOP Basics](https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Basics) - Object-oriented programming fundamentals
