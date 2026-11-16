---
title: Enforce script language: Syntax
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Enforce_Syntax.html
---

| Pages |
| --- |
| Keywords |
| Operators |
| Values |

**Todo:** default parameters, cast rules, comments, better Pointers explanation, better multithreading/timer explanation.

Enfusion Script is strong typed language with syntax similar to C# programming language.

## Basics

### Blocks

Code block is bordered by curly brackets. Code block also defines a scope. Variables defined inside scope are accessible only inside the scope.

```
void Hello()
{
    if (true)
    {
        int x = 2; // First declaration of x
    }
    else
    {
        int x = 23; // Different x not related to the first one
    }
    Print(x); // <-- error x is out of scope now
}
```

### Program structure

Enforce consists of classes and functions. All code must be declared inside a function.

```
class MyClass
{
    int Hello()
    {
        Print("Hello World"); // ok
    }
}
 
int Hello()
{
    Print("Hello World"); // ok
}
 
Print("Hello World"); // wrong, this code will be never executed
```

### Variables and Constants

See the Values page for more information.

### Keywords

**Remarks:** See the Keywords page for more information.

|  | Class | Method | Value | Code |
| --- | --- | --- | --- | --- |
| Keywords | class, modded, sealed, extends, typedef | proto, native, volatile, private, protected, static, event, override, sealed | owned, out, inout, const, reference, notnull, private, protected, static, autoptr, ref | new, delete, thread, null, this, super |

### Operators

**Remarks:** See the Operators page for detailed usage.

| Category | Operator |
| --- | --- |
| Arithmetic | +, -, *, /, % |
| Logical | &&, \|\| |
| Bitwise | &, \|, ~, ^ |
| String | + |
| Increment, decrement | ++, – |
| Shift | <<, >> |
| Relational (conditional) | ==, !=, <, >, <=, >= |
| Assignment | = |
| Indexing | [ ] |
| Negation | ! |

**Remarks:** **Operator Priority:** Priority of operators is similar to C language. See [Wikipedia's C Operators page](https://en.wikipedia.org/wiki/Operators_in_C_and_C%2B%2B#Operator_precedence) for more information.

### Preprocessor Directives

| Directive | Description |
| --- | --- |
| #define | Define a flag. A flag is either defined or not. |
| #ifdef | Open a preprocessor scope considered if the provided flag is defined. Must be ended by #endif. |
| #ifndef | Open a preprocessor scope considered if the provided flag is **not** defined. Must be ended by #endif. |
| #else | Add a preprocessor scope of the opposite condition. |
| #endif | Close a preprocessor scope. |
| #include | Include another file. The effect is as if the other file's content was copy-pasted at this exact location. |

**Remarks:** A flag can be determined outside of the code; see Script and Virtual machine.

### Functions

Functions are basic feature of Enfusion script. Function declaration consist of return value type, function name and list of parameters.

**Remarks:** Function can be declared in global scope or inside class declaration.

**Remarks:** Function parameters are fixed and typed (no variadic parameters).

**Remarks:** Function declaration and implementation can be on different places in code.

**Remarks:** Keyword 'out' before parameter ensures it is passed by reference.

**Remarks:** Enfusion script supports default parameters.

**Remarks:** Named arguments can be used in format argName: value and can be in any order after order-based arguments.

```
void MethodA() {} // no parameters, no return value
 
int GiveMeTen() // no parameters, returns integer
{
    return 10;
}
 
void GiveMeElevenAndTwelve(out int val1, out int val2, int val3)
{
    val1 = 11;
    val2 = 12;
    val3 = 13; // val3 is local only, not passed back
}
 
void PrintNum(int a = 0) // default parameter
{
    Print(a);
}
```

Named arguments example:

```
void Example()
{
    int num = 123;
    string s1 = num.ToString(prefix: "#", hex: true);   // '#7B'
    string s2 = num.ToString(5, prefix: "#", hex: true); // '#0007B'
}
```

### Comments

```
/*
Multi
line
comment
*/
 
int Test()
{
    Print("Hello"); // single line comment
}
```

### Attributes

Attributes represent **meta information** for methods, variables, enums and classes. They are used **mainly by compiler or engine framework** for various functionality.

**Remarks:** Attribute itself is a **regular script class**.

**Remarks:** Attributes are instanced right **after compilation** and destroyed during script module cleanup.

**Remarks:** Attributes are bound to declaration — they are instanced **just once**.

```
// custom attribute declaration
class MyAttribute
{
    string m_Info;
    void MyAttribute(string metaInfo)
    {
        m_Info = metaInfo;
    }
};
 
[MyAttribute("This is an attribute on SomeClass class")]
class SomeClass
{
    [MyAttribute("This is attribute on variable")]
    int m_SomeVariable;
 
    [MyAttribute("This is attribute on method")]
    void SomeMethod() {}
}
```

#### Obsolete

Marks method as obsolete. Compiler throws a compile-time warning, but method is called normally.

```
[Obsolete("Use different method!")]
void Hello() {}
 
void Test()
{
    Hello(); // throws compile warning
}
```

#### Friend

Allows other classes to access protected methods.

```
[Friend(ClassB)]
class ClassA
{
    static protected void MethodA() {}
 
    [Friend(ClassC)]
    static protected void MethodB() {}
}
 
class ClassB
{
    static void Do()
    {
        ClassA.MethodA();
        ClassA.MethodB();
    }
}
 
class ClassC
{
    static void Do()
    {
        ClassA.MethodB();
    }
}
```

## Types

### Primitive Types

| Type name | C++ equivalent | Range | Default value | Size (Bytes) |
| --- | --- | --- | --- | --- |
| int | int32 | −2,147,483,648 through +2,147,483,647 | 0 | 4 |
| float | float | ±1.401298E−45 through ±3.402823E+38 | 0.0 | 4 |
| bool | bool | true or false | false | 4 |
| string | char* | - | "" (empty string) | 8 (pointer) + size of string |
| vector | float[3] | like float | (0.0, 0.0, 0.0) | 8 (pointer) + 12 |
| void | void | - | - | - |
| class | Instance* | pointer to any script object | null | 8 |
| typename | VarType* | pointer to type structure | null | 8 |

### Strings

Strings are **passed by value**, can be concatenated by + operator, are **initialized and destroyed automatically**.

```
Supported escape sequences: `\n \r \t \\ \"`
```

```
void Method()
{
    string a = "Hello";
    string b = " world!";
    string c = a + b;
    Print(c); // prints "Hello world!"
}
```

### Vectors

Vector is a 3D vector representation. Vectors are **passed by value** and **initialized and destroyed automatically**.

```
void Method()
{
    vector up = "0 1 0";
    vector down;        // default value <0, 0, 0>
    down = up;
    down[1] = -1;       // change Y value
    Print(up);           // <0, 1, 0>
    Print(down);         // <0, -1, 0>
}
```

### Objects

Objects are references and are passed by reference. Default value is **null**. All member functions and variables are *public* by default.

```
class MyClass
{
    void Say() { Print("Hello world"); }
}
 
void Method()
{
    MyClass o;          // o == null
    o = new MyClass;    // creates instance
    o.Say();            // calls Say()
}
```

this and super:

```
class Animal
{
    void Hello() { Print("Animal::Hello()"); }
};
 
class Deer : Animal
{
    void Hello() { Print("Deer::Hello()"); }
 
    void Test()
    {
        Hello();        // prints "Deer::Hello()"
        this.Hello();   // same as above
        super.Hello();  // prints "Animal::Hello()"
    }
}
```

**Remarks:** For more info about managed classes see Enforce Automatic Reference Counting.

> **⚠️** Accessing not initialised objects (null) causes Virtual Machine Exception.

### Enums

Enumerators are set of named constant identifiers with int type. Enums can inherit from another enum.

```
enum MojEnumBase
{
    Alfa = 5,   // has value 5
    Beta,       // has value 6
    Gama        // has value 7
};
 
enum MojEnum : MojEnumBase
{
    Modra,        // has value 8
    Zlta,         // has value 9
    Zelena = 20,  // has value 20
    Oranzova      // has value 21
};
 
void Test()
{
    int a = MojEnum.Beta;
    MojEnum b = MojEnum.Zelena;
    int c = b;
 
    Print(a); // prints '6'
    Print(b); // prints '20'
    Print(c); // prints '20'
}
```

### Typenames

Typename is primitive type that contains type definition, used mostly by RTTI API functions.

```
class MyType
{
    int number;
};
 
class MyExtendedType: MyType
{
    string name;
};
 
void Test()
{
    typename t = MyType;
    Print(t); // Prints "typename t = MyType"
 
    t = "MyType".ToType();
    Print(t); // Prints "typename t = MyType"
 
    MyType obj1 = new MyType;
    MyType obj2 = MyType.Cast(t.Spawn());
 
    Print(obj1.IsInherited(t));
    Print(obj1.IsInherited(MyType));
    Print(MyType.IsInherited(MyExtendedType));
}
```

### Casting

#### Upcasting

```
void SomeMethod(Man player)
{
    Cocker cocker = new Cocker();
    Animal animal = cocker;      // OK, as a cocker is an animal
    string sentence = cocker;    // error: cocker does not inherit from string
}
```

#### Downcasting

```
void Method(Dog dog)
{
    Cocker cocker1 = dog;                  // error: "Dog" is too generic
    Cocker cocker2 = Cocker.Cast(dog);     // OK: manual casting
}
```

#### Manual Casting

```
void Method()
{
    float value1 = 4.9;
    int value2 = value1;                         // value2 = 4 (truncated, not rounded)
    string value3 = "result = " + (bool)value2;  // "result = true"
}
```

### Templates

Enfusion script has template feature similar to C++ Templates.

```
class Item<Class T>
{
    T m_data;
    void Item(T data) { m_data = data; }
    void SetData(T data) { m_data = data; }
    T GetData() { return m_data; }
    void PrintData() { Print(m_data); }
};
 
void Method()
{
    Item<string> string_item = new Item<string>("Hello!");
    Item<int> int_item = new Item<int>(72);
 
    string_item.PrintData(); // prints "m_data = 'Hello!'"
    int_item.PrintData();    // prints "m_data = 72"
}
```

### Lists

#### Static Array

Size is fixed at compile time. Static arrays are **passed by value** and **initialized and destroyed automatically**.

```
void MethodA()
{
    int numbersArray[3];
    numbersArray[0] = 54;
    numbersArray[1] = 82;
    numbersArray[2] = 7;
 
    int anotherArray[3] = {53, 90, 7};
}
 
const int ARRAY_SIZE = 5;
 
void MethodB()
{
    int numbersArray[ARRAY_SIZE]; // OK: size from constant
}
 
void MethodC()
{
    int size = 3;
    int numbersArray[size]; // wrong! size cannot be a variable!
}
```

#### Dynamic Array

Dynamic arrays support change of size at runtime. They are **passed by reference** and must be maintained like objects. Typedefs: TStringArray, TFloatArray, TIntArray, TClassArray, TVectorArray.

```
void Method()
{
    TStringArray nameArray = new TStringArray; // or array<string>
 
    nameArray.Insert("Peter");
    nameArray.Insert("Michal");
    nameArray.Insert("David");
 
    string name;
    name = nameArray.Get(1);    // "Michal"
 
    nameArray.Remove(1);
    name = nameArray.Get(1);    // "David"
 
    int nameCount = nameArray.Count(); // 2
}
```

#### Set

A Set is a list of **unique** values. Insertion is slower but containment check is faster.

```
set<string> mySet = new set<string>();
 
mySet.Insert("value1");
mySet.Insert("value2");
mySet.Insert("value1"); // still only contains "value1" and "value2"
```

> **⚠️** Values order is **never** guaranteed.

#### Map

A Map is a list of key→value pairs. Keys are unique. A float **cannot** be a map key.

```
map<string, string> myMap = new map<string, string>();
 
myMap.Insert("key1", "value1");
myMap.Insert("key2", "value1");
myMap.Insert("key1", "value2");     // replaces "key1" value
 
myMap.Get("key1");                  // returns "value2"
myMap["key1"];                      // alternative syntax
myMap.GetKeyByValue("value1");      // returns "key2"
```

## Control Structures

### Conditional structures

#### If statement

```
void Method()
{
    int a = 4;
 
    if (a > 10)
    {
        Print("a is bigger then 10");
    }
    else if (a > 5)
    {
        Print("a is bigger then 5 but smaller than 10");
    }
    else
    {
        Print("a is smaller then 5");
    }
}
```

#### Switch statement

Supports switching by numbers, constants and strings.

```
string name = "peter";
switch(name)
{
    case "john":
        Print("Hello John!");
        break;
 
    case "michal":
        Print("Hello Michal!");
        break;
 
    case "peter":
        Print("Hello Peter!"); // this one is called
        break;
}
```

### Iteration structures

#### For

```
for (int i = 0; i < 3; i++)
{
    Print(i);
}
```

#### Foreach

```
void TestFn()
{
    int pole1[] = {7,3,6,8};
    array<string> pole2 = {"a", "b", "c"};
    auto mapa = new map<string, int>();
    mapa["jan"] = 1;
    mapa["feb"] = 2;
    mapa["mar"] = 3;
 
    // simple foreach iteration
    foreach(int v : pole1)
    {
        Print(v); // prints: 7, 3, 6, 8
    }
 
    // foreach with key (key is array index)
    foreach(int i, string v : pole2)
    {
        Print("pole[" + i + "] = " + v);
    }
 
    // map iteration with key and value
    foreach(auto k, auto v : mapa)
    {
        Print("mapa[" + k + "] = " + v);
    }
 
    // map iteration with just value
    foreach(auto v : mapa)
    {
        Print(v);
    }
}
```

#### While

```
int i = 0;
 
while (i < 3)
{
    Print(i);
    i++;
}
```

## Object-oriented programming (OOP)

**Remarks:** All member functions and variables are **public** by default.

**Remarks:** Class member functions are **virtual** and can be overridden by child class. For overriding, **override** keyword must be used.

**Remarks:** Class can inherit from **one** parent class using keyword 'extends'.

**Remarks:** Objects are not initialized and destroyed automatically, use 'new' and 'delete' (or 'autoptr'/'ref').

**Remarks:** Class variables are cleared to default values upon creation.

### Inheritance

```
class Animal
{
    void MakeSound() {}
};
 
class Dog: Animal
{
    override void MakeSound() { Print("Wof! Wof!"); }
 
    void Aport() {}
};
 
class Cat: Animal
{
    override void MakeSound() { Print("Meow!"); }
 
    void Scratch() {}
};
 
void LetAnimalMakeSound(Animal pet)
{
    if (pet) pet.MakeSound();
}
 
void Method()
{
    Cat nyan = new Cat;
    Dog pluto = new Dog;
 
    nyan.MakeSound();              // prints "Meow!"
    pluto.MakeSound();             // prints "Wof! Wof!"
 
    LetAnimalMakeSound(nyan);      // prints "Meow!"
    LetAnimalMakeSound(pluto);     // prints "Wof! Wof!"
}
```

### Constructor & Destructor

Every class can have **one** constructor and one destructor. Constructor has same name as class, destructor has tilde prefix. Base class constructor is called automatically.

**Remarks:** Constructor can have initialization parameters, destructor cannot.

**Remarks:** Both return void.

**Remarks:** If constructor has no arguments, brackets are not mandatory (e.g. new array<int>;).

**Remarks:** Inheriting class can only add new arguments to those accepted by base class constructor.

```
class MyClassA
{
    void MyClassA() { Print("Instance created!"); }
    void ~MyClassA() { Print("Instance destroyed!"); }
};
 
class MyClassB
{
    string m_name;
    void MyClassB(string name) { m_name = name; }
    void ~MyClassB() { Print("Instance destroyed!"); }
};
 
void Method()
{
    MyClassA a = new MyClassA;           // prints "Instance created!"
    MyClassB b = new MyClassB("Michal"); // creates with name
 
    delete b;                            // prints "Instance destroyed!"
} // ARC automatically destroys 'a'
```

### Safe pointers

All classes inherited from **Managed** class work with soft links instead of plain pointers. Soft link is weak reference that is zeroed upon object destruction, **so they are never invalid**.

```
// without Managed
class A
{
    void Hello() { Print("hello"); }
}
 
void Test()
{
    A a1 = new A();
    A a2 = a1;
    delete a1;             // a1 is NULL
    if (a2) a2.Hello();   // a2 still points to deleted memory - CRASH!
}
```

```
// with Managed
class A : Managed
{
    void Hello() { Print("hello"); }
}
 
void Test()
{
    A a1 = new A();
    A a2 = a1;
    delete a1;             // a1 is NULL, a2 is also NULL (thanks to Managed)
    if (a2) a2.Hello();   // nothing happens - safe
}
```

### Modded Class

Modded class is used to inject inherited class into class hierarchy without modifying other scripts. When modded class is declared, the modded class will be instanced instead of original class everywhere. Only classes within the same module can be modded.

```
// game
class A
{
    void Say() { Print("Hello original"); }
};
 
// mod
modded class A
{
    void Say()
    {
        Print("Hello modded");
        super.Say();
    }
};
 
void Test()
{
    A a = new A();   // "modded class A" is instanced
    a.Say();          // prints "Hello modded" and "Hello original"
}
```

## Multithreading

Script has support for cooperative multi-threading. Three commands provide the functionality: thread, Wait and Sleep.

```
bool GlobalSwitch = false;
void TestThread()
{
    Sleep(1000); // wait for 1000ms (1 sec)
    Print("1 second passed");
    Wait(GlobalSwitch != false);
    Print("Someone switched GlobalSwitch");
}
```

Until either Sleep or Wait interrupts execution, no thread switching occurs. An asynchronous script thread can be created using the thread keyword:

```
void Test()
{
    thread TestThread(); // run TestThread in a separate thread
}
```
