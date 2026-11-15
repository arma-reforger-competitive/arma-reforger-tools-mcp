---
title: Arma Reforger:From SQF to Enforce Script
url: https://community.bistudio.com/wiki/Arma_Reforger:From_SQF_to_Enforce_Script
---

Welcome to **Enfusion**!

While **Arma 3** is powered by **[SQF](https://community.bistudio.com/wiki/SQF_Syntax)**, **Arma Reforger** uses **Enforce Script** as language, which is syntactically close to C#.

**SQF** is a succession of [Scripting Commands](https://community.bistudio.com/wiki/Category:Scripting_Commands) processed in a certain order to give you a certain access or reference to the engine objects themselves (e.g [player](https://community.bistudio.com/wiki/Player)).

It is easy to access, read and process at the cost of performance.

**Enforce Script** is an **Object-Oriented Programming** **language** (OOP language) which as this designation suggests is based on objects.

**Note:** See [Object Oriented Programming Basics](https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Basics) for an introduction to the OOP approach.

The language is way closer to the engine and is inspired by C++, which allows scripters **more possibilities**, but also **more responsibilities** - a script can make or break one's game experience.

It is a stricter language that allows for a wider, stronger interaction with the engine.

## Similarities

Let's start with the similarities between the languages… because they are very different, the list will actually be a short one.

| SQF | Enforce Script |
| --- | --- |
| `// inline comment`<br>`/*`<br>`comment block`<br>`*/`<br>`private _booleanValue = true;`<br>`private _floatValue1 = 5;`<br>`private _floatValue2 = 5.5;`<br>`private _stringValue = "Hello there";` | `// inline comment`<br>`/*`<br>`comment block`<br>`*/`<br>`bool booleanValue = true;`<br>`float floatValue1 = 5;`<br>`float floatValue2 = 5.5;`<br>`string stringValue = "Hello there";` |

And that's it! As this example already shows, even the variable declaration syntax has changed. Now, let's see the main differences on first sight!

## Main Differences

### If-Then-Else

Definitely the most important point of this document. SQF being keyword-based, it needs to "bridge" the "if" condition to the "then" value, using the `then` keyword. Enforce Script uses if-else structure like any recent language.

| SQF | Enforce Script |
| --- | --- |
| `if (alive player) then`<br>`{`<br>`hint "I am alive!";`<br>`};` | `if (myPlayer.GetHealth() > 0)	// no more 'then'!`<br>`{`<br>`Print("I am alive!");`<br>`}`<br>`if (myPlayer.GetHealth() > 0)`<br>`Print("I am alive!");		// a short version is possible` |

The same applies for other code structures (while-do, switch-do, for-do, etc), see Code Flow below.

The **else** part supports `else if` (note: not `elseif`) statements:

| SQF | Enforce Script |
| --- | --- |
| `if (alive player) then`<br>`{`<br>`hint "I am alive!";`<br>`}`<br>`else`<br>`{`<br>`if (damage player > 0.9) then`<br>`{`<br>`hint "That's a lot of damage";`<br>`};`<br>`};` | `if (myPlayer.GetHealth() > 0)`<br>`{`<br>`Print("I am alive!");`<br>`}`<br>`else if (myPlayer.GetHealth() < 10) // no more "else { if }" structure`<br>`{`<br>`Print("That's a lot of damage");`<br>`}` |

### Code Flow

#### for

| SQF | Enforce Script |
| --- | --- |
| `// There are two ways to write a for-loop in SQF:`<br>`for "_i" from 0 to 10 do`<br>`{`<br>`};`<br>`for [{ _i = 0 }, { _i < 10 }, { _i = _i + 1 }] do`<br>`{`<br>`};` | `// Enforce Script has only one type of for loop`<br>`// statement, condition and step are separated by semicolons`<br>`for (int i; i < 5; i++) // i is automatically initialised to 0, but "int i = 0" could also be used`<br>`{`<br>`}` |

#### foreach

| SQF | Enforce Script |
| --- | --- |
| `// forEach loop in SQF is very convenient by selecting the current index and element automatically`<br>`{`<br>`hint str _forEachIndex;	// _forEachIndex is the current array index`<br>`hint str _x;			// _x represents the current element`<br>`} forEach _items;` | `// foreach's syntax is changed and follows today's standards`<br>`// the code to be run is placed after the keyword`<br>`// beware of the syntax: currentIndex and currentElement are separated with ","`<br>`// whereas currentElement and the array being iterated over are separated with ":"`<br>`foreach (int currentIndex, string currentElement : items)`<br>`{`<br>`Print(currentIndex);`<br>`Print(currentElement);`<br>`}`<br>`// or, if the index is not required`<br>`foreach (string currentElement : items)`<br>`{`<br>`Print(currentElement);`<br>`}` |

#### while

| SQF | Enforce Script |
| --- | --- |
| `// Unlike other flow structures, the "while" condition is written between {} brackets`<br>`while { _i < 10 } do`<br>`{`<br>`_i = _i + 1; // only way to increment i by 1 as i++ does not exist in SQF`<br>`};` | `// Condition check in while loop is done in regular () brackets`<br>`while (i < 10)`<br>`{`<br>`i++; // this will increment i by 1`<br>`}` |

#### switch

| SQF | Enforce Script |
| --- | --- |
| `switch (i) do`<br>`{`<br>`case 0: { systemChat "case 0" }; // if i is 0, code in brackets is executed and switch is exited automatically`<br>`case 1: { systemChat "case 1" };`<br>`default { systemChat "default" }; // default does -not- use ":" like case does`<br>`};` | `switch (i)`<br>`{`<br>`case 0:`<br>`Print("case 0");`<br>`break;`<br>`case 1:`<br>`Print("case 1");`<br>`break;`<br>`default: // default -does- use ":" just like case does`<br>`Print("default");`<br>`break;`<br>`}` |

#### Others

| SQF | Enforce Script |
| --- | --- |
| `// SPECIAL CASES`<br>`i = if (isScript) then { 1 } else { 0 }; // if-then-else returning value, not possible in Enforce Script`<br>`if (isScript) exitWith {};	// exitWith breaks from the current -scope-`<br>`// it will exit the script only if at its root`<br>`// lazy evaluation: by default, all statements in condition field are evaluated`<br>`// to prevent this behaviour the second statement should be wrapped in code brackets {}`<br>`// if (!isNil "_value" && _value > 10) then // wrong as "_value > 10" will be evaluated`<br>`if (!isNil "_value" && { _value > 10 }) then`<br>`{`<br>`hint "_value exists and is greater than 10";`<br>`};` | `// SPECIAL CASES`<br>`// an if statement can avoid brackets for only one line of code`<br>`if (isScript)`<br>`break;		// this will exit a while loop`<br>`if (isScript)`<br>`return 0;	// this will exit the current method and return 0`<br>`// "value.GetValue() > 10" will not be evaluated if "value != null" returns false`<br>`if (value != null && value.GetValue() > 10)`<br>`{`<br>`Print("value exists and is greater than 10");`<br>`}` |

### Case Sensitivity

Everything is case-sensitive in Enforce Script, unlike SQF:

| SQF | Enforce Script |
| --- | --- |
| `private _myValue = "Hello there";`<br>`hint _MYVALUE; // hints "Hello there"`<br>`IF (true) THEN`<br>`{`<br>`hint "it's true!";`<br>`};`<br>`if (_myValue == "HELLO THERE") then`<br>`{`<br>`// will work`<br>`};` | `string myValue = "Hello there";`<br>`Print(MYVALUE); // error: MYVALUE is undefined`<br>`IF (true) // error: "IF" is an unknown operator. "if" is the proper casing`<br>`{`<br>`Print("it's true!");`<br>`}`<br>`if (myValue == "HELLO THERE")`<br>`{`<br>`// will NOT work - "Hello there" is different from "HELLO THERE"`<br>`}` |

### Typed Variables

In SQF, a variable can change its type on the go according to value assignation.
In Enforce Script, a variable has one type and this type cannot be changed during the variable's lifetime.

| SQF | Enforce Script |
| --- | --- |
| `private _myValue = "Hello there";	// _myValue is a string`<br>`_myValue = 5;						// _myValue is now a floating point number`<br>`_myValue = true;					// _myValue is now a boolean, etc` | `string myValue = "Hello there";		// myValue is a string`<br>`myValue = 5;						// error: cannot convert an int to string`<br>`myValue = true;						// error: cannot convert a bool to string` |

### Position

A position is the location of an object. A position is composed of X, Y and Z values, X being the West → East axis, Y being the Floor → Sky axis, and Z being the South → North axis. A position is called `origin` in Enfusion.

Real Virtuality uses the `[X, Z, Y]` format (a vector pointing up being `[0, 0, 1]`) whereas Enfusion uses `{ X, Y, Z`} (a vector pointing up being `{ 0, 1, 0`}).

**Important:** Note that in Enfusion the coordinate system is **left-handed**  - see [Wikipedia](https://en.wikipedia.org/wiki/Cartesian_coordinate_system#Orientation_and_handedness).

| SQF | Enforce Script |
| --- | --- |
| `private _northVector = [0, 1, 0];`<br>`private _westVector = [-1, 0, 0];`<br>`private _upVector = [0, 0, 1];` | `vector northVector = "0 0 1";`<br>`vector westVector = "-1 0 0";`<br>`vector upVector = "0 1 0";` |

### Array

In SQF, an array is a list that can contain any type of data including sub-arrays, and can be expanded at will.
In Enforce Script, there are two types of array: static (extremely fast but of static size) and dynamic (size can change); also, an array can only contain **one** type of data.

| SQF | Enforce Script |
| --- | --- |
| `private _array = [];`<br>`_array pushBack 0;		// _array is [0]`<br>`_array pushBack [1];	// _array is [0, [1]]`<br>`_array pushBack "2";	// _array is [0, [1], "2"], etc`<br>`_array set [0, 1];		// sets the first element to 1`<br>`// _array is [1, [1], "2"]`<br>`// array duplication`<br>`private _array2 = +_array;` | `array<int> myArray = {};	// dynamic array`<br>`myArray.Insert(0);			// array is { 0 }`<br>`myArray.Insert(true);		// array is { 0, 1 } - true is 1`<br>`myArray.Insert("2");		// error: an array cannot contain a different type`<br>`myArray[0] = 1;				// sets the first element to 1`<br>`// array duplication`<br>`array<int> duplicate = {};`<br>`duplicate.Copy(myArray);	// does not work for ref items`<br>`// ref array duplication`<br>`array<ref SCR_Class> refDuplicate = {};`<br>`foreach (SCR_Class item : myArray)`<br>`{`<br>`refDuplicate.Insert(item);`<br>`}`<br>`// a helper is provided`<br>`array<ref SCR_Class> refDuplicate = SCR_ArrayHelper<SCR_Class>.GetCopy(myArray);`<br>`// new array type: static (size) array`<br>`int myArray[3];				// automatically initialised to the type's default value (here { 0, 0, 0 })`<br>`myArray[0] = 1;				// { 1, 0, 0 }`<br>`myArray[1] = 2;				// { 1, 2, 0 }`<br>`myArray[2] = 3;				// { 1, 2, 3 }` |

### Data Types

Including **vectors**, **enums**, **sets**, **classes** of course and others! Find all the new data types on their [dedicated page](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Values).

| SQF | Enforce Script |
| --- | --- |
| `private _position = [50, 50, 0];	// an array`<br>`private _number1 = 42;				// a Number, a.k.a float`<br>`private _number2 = 5.5;				// a Number, a.k.a float`<br>`_number1 = _number1 + 1;`<br>`_number2 = _number2 - 1;`<br>`private _hashmap = createHashMap;`<br>`_hashmap set [1, "oops"];`<br>`_hashmap set [2, "two"];`<br>`_hashmap set ["two", true];`<br>`_hashmap set [1, "one"];`<br>`private _value = nil;`<br>`if (isNil "_value") then { hint "_value is not defined" };`<br>`private _value = objectParent player;`<br>`if (isNull _value) then { hint "player has vehicle" };` | `vector position = { 50, 0, 50 };`<br>`int number1 = 42;`<br>`float number2 = 5.5;`<br>`number1++;`<br>`number2--;`<br>`map<int, string> hashmap = new map<int, string>(); // types are set in stone`<br>`hashmap.Insert(1, "oops");`<br>`hashmap.Insert(2, "two");`<br>`// no Enforce Script equivalent`<br>`hashmap.Set(1, "one"); // Set() is to update a value - it can create it too, but Insert() is faster`<br>`set<string> stringSet = new set<string>();`<br>`stringSet.Insert("value1");`<br>`stringSet.Insert("value2");`<br>`stringSet.Insert("value1"); // returns false as value1 already exists`<br>`// there is no isNil equivalent in Enforce Script as a variable is either defined or not`<br>`// its content can still be null as seen below`<br>`MyClass myInstance;`<br>`if (myInstance == null)`<br>`Print("myInstance is null");`<br>`// alternatively`<br>`if (!myInstance)`<br>`Print("myInstance is null");` |

### Object-Oriented Programming

SQF is a scripting language based on sequences of expressions set in scripts. Enforce Script is Object-Oriented Programming (OOP) meaning that the code is placed inside declared objects.

**Note:** The basics of OOP can be found in [Object Oriented Programming Basics](https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Basics); see also [Wikipedia](https://en.wikipedia.org/wiki/Object-oriented_programming).

| SQF | Enforce Script |
| --- | --- |
| `// in-script code declaration - normally one would have to declare it in CfgFunctions`<br>`private _sum = {`<br>`params [`<br>`["_value1", 0, [0]],`<br>`["_value2", 0, [0]]`<br>`];`<br>`_value1 + _value2;`<br>`};`<br>`private _result = [5, 3] call _sum;` | `// a method is a class' function`<br>`class MyClass`<br>`{`<br>`int Sum(int value1, int value2)`<br>`{`<br>`return value1 + value2;`<br>`}`<br>`// a different method signature, yet named the same`<br>`string Sum(string value1, string value2)`<br>`{`<br>`return value1 + value2;`<br>`}`<br>`}`<br>`// further in code`<br>`MyClass myInstance = new MyClass();`<br>`int result = myInstance.Sum(5, 3);`<br>`string result = myInstance.Sum("Hello ", "there");`<br>`string result = myInstance.Sum("Hello ", 3); // does not work - no such string/int signature, only string/string` |

**Warning:** It is worth noting that Enforce Script does **not** benefit from previous titles' [Scheduler](https://community.bistudio.com/wiki/Scheduler), therefore wildly creating new threads is not recommended!
See Enforce Script's example below for `Callqueue`'s usage.

| SQF | Enforce Script |
| --- | --- |
| `hint "Thread A 1/2";`<br>`[] spawn {`<br>`hint "Thread B 1/2";`<br>`sleep 1;`<br>`hint "Thread B 2/2";`<br>`};`<br>`hint "Thread A 2/2";` | `class MyClass`<br>`{`<br>`void PrintMessage()`<br>`{`<br>`Print("Thread A 1/2");`<br>`GetGame().GetCallqueue().CallLater(PrintOtherMessage, 1000); // in milliseconds`<br>`// thread Thread_PrintOtherMessage(); // thread usage is not recommended`<br>`Print("Thread A 2/2");`<br>`}`<br>`void PrintOtherMessage()`<br>`{`<br>`Print("CallQueue later");`<br>`}`<br>`void Thread_PrintOtherMessage()`<br>`{`<br>`Print("Thread B 1/2");`<br>`Sleep(1000);`<br>`Print("Thread B 2/2");`<br>`}`<br>`}`<br>`// further in code`<br>`MyClass myInstance = new MyClass();`<br>`myInstance.PrintMessage();` |

## See Also

- [Scripting: Conventions](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Conventions)
- Scripting Guidelines
- Scripting Tutorials
