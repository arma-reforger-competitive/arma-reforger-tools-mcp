---
title: Syntax: Values
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Enforce_Syntax_Values.html
---

A **value** is either a **variable** (that can be changed) or a **constant** (that cannot be changed).

Values are declared in Enfusion with a type. Enfusion Script uses strong-types, which means a value's type cannot be changed along its lifetime.

## Identifier

An **identifier** is the actual name of a value. The naming rules are:

- can be composed of ASCII letters, numbers and underscores
- **must** start with an underscore or a letter (**cannot** start with a number)
- is **case-sensitive**
- **cannot be identical to a keyword**

The regular expression: ^[a-zA-Z_][a-zA-Z0-9_]*$

```
int myNumber = 10;
int _myNumber = 10;
int myNUMBER = 10;      // different from myNumber
 
int 1number = 1;        // wrong - starts with a number
int my#variable = 1;    // wrong - contains invalid character
int auto = 1;           // wrong - "auto" keyword exists
```

## Declaring a value

```
int myNumber;       // declared and auto-initialised to 0
myNumber = 10;      // myNumber is now 10
 
int myNumber = 10;  // declared with a value directly
```

## Passing a value

### By content

```
int myVar1 = 33;
int myVar2 = myVar1;    // myVar2 is 33 - copied content
myVar2 = 42;            // myVar2 is 42, myVar1 is still 33
```

### By reference

```
array<int> myArray1 = { 0, 1, 2, 3 };
array<int> myArray2 = myArray1;  // references same object
myArray2[0] = 99;                // myArray1 is now { 99, 1, 2, 3 }
```

## Types

### Primitive Types

| Type name | C++ equivalent | Range | Default value | Size (Bytes) |
| --- | --- | --- | --- | --- |
| int | int32 | -2,147,483,648 through +2,147,483,647 | 0 | 4 |
| float | float | ±1.18E-38 through ±3.402823E+38 | 0.0 | 4 |
| bool | bool | true or false | false | 4 |
| string | char* | - | "" (empty string) | 8 (pointer) + (length × 1 B) |
| vector | float[3] | like float | { 0, 0, 0 } | 8 (pointer) + 12 |
| void | void | - | - | - |
| class | Instance* | pointer to any script object | null | 8 |
| typename | VarType* | pointer to type structure | null | 8 |

### Boolean

Passed by: **value** — Prefix: **b** — Default: **false**

```
bool myValue;          // false
myValue = 10 > 0;     // true
```

### Integer

Passed by: **value** — Prefix: **i** — Default: **0**

```
int myValue;       // 0
myValue = 20;      // 20
myValue /= 3;     // 6 (truncated, not rounded)
```

### Float

Passed by: **value** — Prefix: **f** — Default: **0.0**

**Remarks:** The further away from 0 the float value is, the less precision it has. For "almost equal" comparison, see float.AlmostEqual; default epsilon is 0.0001.

```
float myValue;         // 0.0
myValue = 1;           // 1.0
myValue /= 3;         // 0.333333…
 
float originalValue = 1;
float divisionResult = originalValue / 3333;
float result = divisionResult * 3333;
originalValue == result;                     // returns false due to floating point precision
float.AlmostEqual(originalValue, result);    // returns true
```

### String

Passed by: **value** — Prefix: **s** — Default: **""**

**Remarks:** A string **cannot** be null. String comparison is **case-sensitive**.

```
Escape characters: `\n` `\r` `\t` `\\` `\"`
```

```
string username;           // "" (empty string)
username = "Player 1";
```

### Enum

Passed by: **value** — Prefix: **e** — Default: **0** (which may not exist in the enum)

```
enum EHealthState
{
    ALIVE,
    INJURED,
    UNCONSCIOUS,
    DEAD,
}
 
EHealthState healthState;                   // ALIVE (0)
healthState = EHealthState.UNCONSCIOUS;     // UNCONSCIOUS (2)
int myValue = EHealthState.ALIVE;           // valid
```

Custom values:

```
enum EHealthState
{
    ALIVE       = 42,
    INJURED,              // 43
    UNCONSCIOUS = 50,
    DEAD,                 // 51
}
 
// bit-shifted enum for flag usage
enum ELifeStatus
{
    HEALTHY   = 1 << 0,   // 1
    HAS_HOUSE = 1 << 1,   // 2
    HAS_FOOD  = 1 << 2,   // 4
}
```

### Vector

Passed by: **value** — Prefix: **v** — Default: **{ 0, 0, 0 }**

Three ways to create: vector v;, vector v = { 0, 1.5, 2 };, vector v = "0 1.5 2";

Two vectors can be compared with == for **value** comparison.

```
vector myVector = "0 1.5 2";
vector myVector2 = myVector;
myVector[1] = 42;                 // myVector is now { 0, 42, 2 }
myVector == myVector2;            // false
 
myVector2 = { 0, 42, 2 };
myVector == myVector2;            // true
```

### Array

Passed by: **reference** (dynamic) or **value** (static) — Prefix: **a** — Default: **null**

```
array<string> dynamicArray = {};
string staticArray[2] = { "Hello", "" };
 
dynamicArray.Insert("Hello");
dynamicArray.Insert("");
 
dynamicArray[1] = "there";
staticArray[1]  = "there";
 
// dynamic/static arrays cannot be directly compared for values
// == compares pointers (is it the same array, not the same values)
dynamicArray == staticArray;            // false
dynamicArray[0] == staticArray[0];      // true
dynamicArray[1] == staticArray[1];      // true
```

### Set

Passed by: **reference** — Default: **null**

Ensures uniqueness. Insertion is slower but containment check is faster.

```
set<string> setInstance = new set<string>();
 
setInstance.Insert("Hello there");
setInstance.Insert("General Kenobi");
setInstance.Insert("Hello there");      // still only contains 2 items
 
setInstance.Get(0); // values order is never guaranteed!
```

### Map

Passed by: **reference** — Prefix: **m** — Default: **null**

Key→value pairs. Keys are unique. A float **cannot** be a map key.

```
map<int, string> mapInstance = new map<int, string>();
 
mapInstance.Set(5712, "Hello there");
mapInstance.Set(5716, "General Kenobi");
 
mapInstance.Get(5712);                       // returns "Hello there"
mapInstance.Get(5715);                       // returns "" (default string)
mapInstance.Get(5716);                       // returns "General Kenobi"
 
mapInstance.GetKeyByValue("Hello there");    // returns 5712
mapInstance.GetKeyByValue("test");           // returns 0 (default int)
 
mapInstance.Contains(5716);                  // true
mapInstance.ReplaceKey(5716, 5715);          // replaces "General Kenobi" key
mapInstance.Contains(5716);                  // false
 
mapInstance.Contains(5715);                  // true
mapInstance.Remove(5715);                    // removes "General Kenobi"
mapInstance.Contains(5715);                  // false
```

### Class

Passed by: **reference** — Default: **null**

```
class ObjectClass
{
    private int m_iHealth = 100;
 
    int function getHealth()
    {
        return m_iHealth;
    }
 
    bool function setHealth(int health)
    {
        if (health < 0 || health > 100)
            return false;
 
        m_iHealth = health;
        return true;
    }
}
 
void HealthMethod()
{
    ObjectClass myObjectInstance;             // null
    myObjectInstance = new ObjectClass();
    int objectHealth = myObjectInstance.getHealth();
    Print(objectHealth);
}
```

### Typename

Passed by: **reference** — Default: **null**

```
class ObjectClass
{
    int Health = 100;
}
 
typename t;                         // null
t = ObjectClass;
string classname = TypeName(t);     // returns "ObjectClass"
t = Type(classname);                // returns ObjectClass typename too
```

## Scope

A value has a scope that defines its existence and accessibility.

```
void SetDammage(int dammage)
{
    int newHealth;
    newHealth = m_iHealth - dammage;
    if (newHealth < 0)
    {
        newHealth = 0;
    }
    else if (newHealth > 100)
    {
        int difference = newHealth - 100;     // difference is declared
        Print("health overflow: " + difference);
        newHealth = 100;                      // difference is destroyed after leaving this scope
    }
    // difference does not exist here
 
    m_iHealth = newHealth;
    Print("Health has been set");             // newHealth is destroyed after this line
}
```

**Incorrect**

```
if (soldiersHealth > 75)
{
    string message = "I'm doing well";
}
else
{
    string message = "I don't feel so good";
}
Print(message); // error: message variable is undefined here
```

**Correct**

```
string message = "I don't feel so good";
if (soldiersHealth > 75)
{
    message = "I'm doing well";
}
Print(message); // OK
```

### Casting

A value can be **casted** ("forced" into a type) if the underlying type is known. A wrong cast will return a null value.

```
Soldier_Base aSoldier = new B_Soldier_F();   // valid
aSoldier.SayHello();                         // invalid - Soldier_Base has no SayHello
 
B_Soldier_F mySoldierF = B_Soldier_F.Cast(aSoldier);
mySoldierF.SayHello();                       // valid
```
