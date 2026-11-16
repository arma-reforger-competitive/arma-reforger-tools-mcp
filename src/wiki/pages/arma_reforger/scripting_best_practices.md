---
title: Arma Reforger:Scripting: Best Practices
url: https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Best_Practices
---

## Getting Started

In the domain of development, any rule is a rule of thumb.
If a rule states for example that it is better that a line of code doesn't go over 80 characters, it doesn't mean that any line ***must not*** go over 80 characters; sometimes, the situation needs it.

If the code has a good structure, **do not** change it to enforce a single arbitrary rule. If many of them are not implemented/not respected, changes should be applied; again, this is according to one's judgement.

With that being said, let's go!

## Best Practices

**Note:** See [Scripting Conventions](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Conventions) for all the conventions to date.

### Code Format

- **Reminder:** chosen indentation for Enfusion is [Allman style](https://en.wikipedia.org/wiki/Indentation_style#Allman_style)
- **Reminder:** indentation is done with **tabulations**
- Use empty space. Line return, spaces before and after brackets, if this improves readability, use it: space is free
- *One-lining* (putting everything in one statement) memory improvement is most of the time not worth the headache it gives when trying to read it: don't overuse it
  - it also hinders debugger's usage, e.g in the event of an inlined `if`

### Variable Format

- Name variables and functions properly: code must be readable by a human being, e.g variables like **u** instead of **uniform** should not exist.
  - **`i`** is an accepted iteration variable name (e.g in `for` loops).
- Prefix any public content (classes, global methods, global variables) with a [Creator Tag](https://community.bistudio.com/wiki/Scripting_Tags) in order to prevent conflicts with other mods.
- Use the closest value type whenever possible; using `auto` for a known variable type makes code more obscure and prevents autocompletion.

### Code Structuration

#### SOLID

A series of development principles to follow in order to ensure an easy code maintenance and lifetime.

**Note:** See https://en.wikipedia.org/wiki/SOLID.

#### DRY

**D**on't **R**epeat **Y**ourself. If within the same class, the same code or the same pattern is written in various places, write a protected method and use appropriate parameters.

**Note:** See https://en.wikipedia.org/wiki/Don't_repeat_yourself.

#### Logical Simplifications

If the code has too many repetitions, make a common method as stated above.

If the code has too many levels, it is time to split it and rethink it.

#### Examples

| Improvable | Good |
| --- | --- |
| `auto number = 42;`<br>`Animal cutePet = new Dog();` | `int number = 42;`<br>`Dog cutePet = new Dog();` |
| `int i = 0;`<br>`string result = "";`<br>`SCR_MyClass obj = null;` | `int i;				// default value = 0 - see {{Link\|Arma Reforger:Scripting: Values#Integer\|Values - integer}}`<br>`string result;		// default value = "" (a string cannot be null)`<br>`SCR_MyClass obj;	// default value = null` |
| `// a method call is more expensive than a bool check`<br>`if (obj.MustBeTreated() \|\| m_bTreatAllObjects)`<br>`Print(obj);`<br>`if (obj.MustBeTreated() && m_bTreatAllObjects)`<br>`Print(obj);` | `// cheap checks go first, expensive checks (method calls) go after`<br>`if (m_bTreatAllObjects \|\| obj.MustBeTreated())`<br>`Print(obj);`<br>`if (m_bTreatAllObjects && obj.MustBeTreated())`<br>`Print(obj);` |
| `// many identical method calls`<br>`if (obj.MustBeTreated() && obj.GetObject())`<br>`Print("Result: " + obj.GetObject().m_sValue1 + " " + obj.GetObject().m_sValue2);` | `// "bigger", non-repetitive code can be beneficial for performance and readability`<br>`if (obj.MustBeTreated())`<br>`{`<br>`SCR_Object subObj = obj.GetObject();`<br>`if (subObj)`<br>`Print("Result: " + subObj.m_sValue1 + " " + subObj.m_sValue2);`<br>`}` |
| `foreach (SCR_Object obj : list)`<br>`{`<br>`Method(obj); // one method call per iteration`<br>`}`<br>`void Method(SCR_Object obj)`<br>`{`<br>`if (!obj)`<br>`return;`<br>`Print(obj.m_sName + " has a value of " + obj.m_sValue);`<br>`}` | `foreach (SCR_Object obj : list) // the least method calls, the better`<br>`{`<br>`if (!obj)`<br>`continue;`<br>`Print(obj.m_sName + " has a value of " + obj.m_sValue);`<br>`}` |
| `bool IsObjectAlive(SCR_Object obj)`<br>`{`<br>`if (!obj)`<br>`return false;`<br>`if (obj.m_Health > 0) // keep this structure for complex code`<br>`return true;`<br>`else`<br>`return false;`<br>`}` | `bool IsObjectAlive(SCR_Object obj)`<br>`{`<br>`return obj && obj.m_Health > 0;`<br>`}` |
| `bool IsObjectValid(SCR_Object obj)`<br>`{`<br>`if (!obj)`<br>`return false;`<br>`return true;`<br>`}` | `bool IsObjectValid(SCR_Object obj)`<br>`{`<br>`return obj != null; // for readability`<br>`}` |
| `for (int i; i < list.Count(); i++) // list.Count() is called on every iteration`<br>`{`<br>`// ...`<br>`}` | `for (int i, count = list.Count(); i < count; i++) // only one list.Count() call`<br>`{`<br>`// ...`<br>`}` |
| `for (int i, count = list.Count(); i < count; i++)`<br>`{`<br>`if (list[i])		// first`<br>`Print(list[i]);	// and second .Get(i) method calls`<br>`}` | `foreach (SCR_Object obj : list) // foreach is faster for start-to-end iterating`<br>`{`<br>`if (obj)`<br>`Print(obj); // no additional method call`<br>`}` |
| `for (int i, count = list.Count(); i < count; i++)`<br>`{`<br>`PrintFormat("Object #%1 = %2", i, list[i]);`<br>`}` | `foreach (int i, SCR_Object obj : list) // iteration index is available this way too`<br>`{`<br>`PrintFormat("Object #%1 = %2", i, obj);`<br>`}` |
| `// declaring an 'obj' every loop generates a pointer release each time`<br>`foreach (SCR_ParentObject parent : list)`<br>`{`<br>`SCR_Object obj = parent.m_Object;`<br>`if (obj)`<br>`Print(obj.m_sName);`<br>`}` | `SCR_Object obj; // external declaration = only one release at the end of the scope`<br>`foreach (SCR_ParentObject parent : list)`<br>`{`<br>`obj = parent.m_Object;`<br>`if (obj)`<br>`Print(obj.m_sName);`<br>`}` |
| `array<SCR_Object> toRemove = {};`<br>`foreach (SCR_Object obj : bigArray)`<br>`{`<br>`if (obj.m_bShouldBeRemoved)`<br>`toRemove.Insert(obj);`<br>`}`<br>`foreach (SCR_Object obj : toRemove)`<br>`{`<br>`bigArray.RemoveItem(obj); // or RemoveItemOrdered if order is important`<br>`}` | `for (int i = bigArray.Count() - 1; i >= 0; i--) // reverse iterating`<br>`{`<br>`if (bigArray[i].m_bShouldBeRemoved)`<br>`bigArray.Remove(i); // or RemoveItemOrdered if order is important`<br>`}` |
| `if (a)`<br>`{`<br>`if (b)`<br>`{`<br>`if (c)				// also known as Hadouken code`<br>`Method(true);`<br>`else`<br>`Method(false);`<br>`}`<br>`}` | `if (a && b)`<br>`Method(c);` |
| `if (a)`<br>`{`<br>`Method(a);`<br>`if (b)`<br>`{`<br>`Method(b);`<br>`if (c)				// another Hadouken code, with complications`<br>`{`<br>`Method(c);`<br>`return 42;`<br>`}`<br>`else`<br>`{`<br>`return -1;`<br>`}`<br>`}`<br>`else`<br>`{`<br>`return -1;`<br>`}`<br>`}`<br>`else`<br>`{`<br>`return -1;`<br>`}` | `if (!a)`<br>`return -1; // this is called early return and helps funnel down the code`<br>`Method(a);`<br>`if (!b)`<br>`return -1;`<br>`Method(b);`<br>`if (!c)`<br>`return -1;`<br>`Method(c);`<br>`return 42;` |
| `int i;`<br>`if (a)`<br>`i++;`<br>`if (b)`<br>`i++;`<br>`if (c)`<br>`i++;`<br>`// etc` | `int i;`<br>`array<bool> conditions = { a, b, c, /* etc */ };`<br>`foreach (bool condition : conditions)`<br>`{`<br>`if (condition)`<br>`i++;`<br>`}` |
| `Initialise(player1, 1);`<br>`Initialise(player2, 2);`<br>`Initialise(player3, 3);`<br>`Initialise(player4, 4);`<br>`Initialise(player5, 5);`<br>`Initialise(player6, 6);` | `array<IEntity> list = { player1, player2, player3, player4, player5, player6 };`<br>`foreach (int i, IEntity item : list)`<br>`{`<br>`Initialise(item, i + 1);`<br>`}`<br>`// or, better, one method call that initialises all of them`<br>`Initialise(list);		// numbering is then done inside the method, if possible`<br>`Initialise(list, 1);	// otherwise the starting number can be provided` |

#### Code Comments

Code comments are surprisingly **not** a must-have for inside code; code organisation combined to variable names should be enough to be read by a human, **then** comment can be used:
- a comment should explain ***why*** the code is written this way
- a comment should not tell ***what*** the code does; code should be self-explanatory
- as a last resort in the event of a complex piece of code, a comment can be used to describe what the code actually does - or at least its intention

On the other hand, *documentation* is more than welcome as it provides information from the outside without having to read the code. Enfusion uses [Doxygen](https://community.bistudio.com/wiki/Doxygen).

#### Files Organisation

**Note:** See [Arma Reforger:Directory Structure](https://community.bistudio.com/wiki/Arma_Reforger:Directory_Structure) to know how/where to organise script files (Scripts\GameCode).

- Have one class/enum per file
  - Small classes/enums can always be grouped together in the same file, provided they are part of the same system or only used there
- Use (sub-)directories to group related classes together
