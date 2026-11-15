---
title: Syntax: Operators
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Enforce_Syntax_Operators.html
---

Operators are symbols between values that allow for operations between values.

## Types

### Assignment

#### =

The "assign" operator sets the righthand value to the lefthand variable. It is **NOT** an equality check.

```
int five = 5;
```

#### ++

Adds 1 to the provided numerical variable (int or float only).

```
int result = 5;
result++; // result is 6
```

#### –

Subtracts 1 from the provided numerical variable (int or float only).

```
int result = 5;
result--; // result is 4
```

#### +=

Adds the righthand value to the lefthand variable.

```
int result = 5;
result += 3; // result is 8
```

#### -=

Subtracts the righthand value from the lefthand variable.

```
int result = 5;
result -= 3; // result is 2
```

#### *=

Multiplies the lefthand variable with the righthand value.

```
int result = 5;
result *= 3; // result is 15
```

#### /=

Divides the lefthand variable by the righthand value.

```
int result = 5;
result /= 3; // result is 1 (floored int value)
```

### Arithmetic

#### +

Addition between two numeric values.

```
5 + 3;       // result is 8 (int)
5 + 2.5;     // result is 7.5 (float)
3.25 + 1.75; // result is 5.0 (float)
```

#### -

Subtraction between two numerical values.

```
5 - 3;       // result is 2 (int)
5 - 2.5;     // result is 2.5 (float)
3.25 - 1.25; // result is 2.0 (float)
```

#### *

Multiplication between two numerical values.

```
5 * 3;       // result is 15
5 * 2.5;     // result is 12.5 (float)
1.5 * 3.25;  // result is 4.875 (float)
```

#### /

Division. If both values are int, the return value will be a floored integer.

```
5 / 3;       // result is 1 (int, floored)
5 / 2.5;     // result is 2.0 (float)
3.5 / 0.5;   // result is 7.0 (float)
```

#### %

Modulo — gives the remainder of a division. Only applies to integers.

```
 5 % 3;  // result is 2: 5 is one time 3, remaining 2
10 % 2;  // result is 0: 10 is five times 2, remaining 0
```

### Relational

#### >

Returns **true** if the left argument is strictly greater than the right argument.

```
10 > 11;   // false
10 > 10;   // false
10 >  9;   // true
```

#### <

Returns **true** if the left argument is strictly smaller than the right argument.

```
10 < 11;   // true
10 < 10;   // false
10 <  9;   // false
```

#### >=

Returns **true** if the left argument is greater than or equal to the right argument.

```
10 >= 11;  // false
10 >= 10;  // true
10 >=  9;  // true
```

#### <=

Returns **true** if the left argument is smaller than or equal to the right argument.

```
10 <= 11;  // true
10 <= 10;  // true
10 <=  9;  // false
```

#### ==

Returns **true** if the left argument is strictly equal to the right argument.

Applies to: boolean, int, float, int/float, string (case-sensitive), vector (value), array (reference), object (reference).

```
true == true;   // true
true == false;  // false
 
10 == 11;       // false
10 == 10;       // true
10 == 10.0;     // true
 
MyClass object1 = new MyClass();
MyClass object2 = object1;
object1 == object2;  // true - same instance
 
MyClass object3 = new MyClass();
object1 == object3;  // false - different instance
```

#### !=

Returns **true** if the left argument is different from the right argument.

Applies to: boolean, int, float, int/float, string (case-sensitive), vector (value), array (reference), object (reference).

```
10 != 11;   // true
10 != 10;   // false
10 != 10.0; // false
```

### Logical

#### &&

Returns **true** if both conditions are true.

```
boolean a = true;
boolean b = true;
boolean result = a && b; // result is true
```

#### ||

Returns **true** if left or right condition is true (or both).

```
boolean a = true;
boolean b = false;
boolean result = a || b; // result is true
```

#### !

Inverts a boolean value. Can also be used on int/float (0 comparison), string (empty comparison), object (null comparison).

```
boolean a = true;
boolean result = !a; // result is false
 
int health = 0;
if (!health) // same as if (health == 0)
{
    Print("dead");
}
 
string data = "";
if (!data) // same as if (data == "")
{
    Print("empty string");
}
 
MyClass instance = null;
if (!instance) // same as if (instance == null)
{
    Print("instance is null!");
}
 
// note that if (value) to check the opposite works too
```

### Bitwise

#### &

Bitwise AND — both bits must be 1 to return 1.

```
int a = 13;          // 00001101 in binary
int b = 11;          // 00001011 in binary
int result = a & b;  // 00001001 in binary = 9
```

#### |

Bitwise OR — one of the bits must be 1 to return 1.

```
int a = 13;          // 00001101 in binary
int b = 11;          // 00001011 in binary
int result = a | b;  // 00001111 in binary = 15
```

#### ~

Bitwise NOT — reverts all bits.

```
int a = 7;           // 0000...0111 in binary
int result = ~a;     // 1111...1000 in binary = -8
```

#### <<

Left shift — moves bits to the left.

```
int a = 1;              // 00000001 in binary
int result = a << 2;    // 00000100 in binary = 4
```

#### >>

Right shift — moves bits to the right.

```
int a = 6;              // 00000110 in binary
int result = a >> 2;    // 00000001 in binary = 1
```

### String

#### +

Used with strings, concatenates them. When used with string and another type, it will stringify the other value (calling .ToString()). The left argument **must** be string.

```
string a = "Hello";
string b = "there";
string result = a + " " + b;  // "Hello there"
 
string stringifiedBoolean = "string " + true;       // "string true"
string stringifiedInt     = "string " + 42;          // "string 42"
string stringifiedFloat   = "string " + 4.2;         // "string 4.2"
string stringifiedObject  = "string " + myObject;    // "string MyClass<0x00000282360ECFD0>"
string stringifiedType    = "string " + MyClass;     // "string MyClass"
 
string ok    = "test" + 3;       // "test3"
string error = 3 + "test";       // parsing error!
```

### Indexing

#### [ ]

Used on arrays/strings to get an element by its **zero-based** index.

```
array<int> numbers = { 1, 2, 3, 4, 5 };
int thirdNumber = numbers[2]; // result is 3
 
array<string> letters = { "A", "B", "C", "D", "E" };
string thirdLetter = letters[2]; // result is "C"
 
string text = "ABCDE";
string thirdLetter = text[2]; // result is "C"
```

## Unsupported Operators

The following operators known from C/C++ are **not supported** in Enforce and will cause a compilation error:

| Operator | C/C++ name | Enforce alternative |
| --- | --- | --- |
| ? : | Ternary (conditional) | Use an if/else block |

```
// WRONG - ternary operator is not supported
int result = (a > b) ? a : b;
 
// CORRECT - use if/else instead
int result;
if (a > b)
    result = a;
else
    result = b;
```

## Precedence

Operators precedence in Enforce is identical to the C language: see the [Wikipedia article section](https://en.wikipedia.org/wiki/Operators_in_C_and_C%2B%2B#Operator_precedence).
