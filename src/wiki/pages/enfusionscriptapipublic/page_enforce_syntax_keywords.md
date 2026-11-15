---
title: Syntax: Keywords
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Enforce_Syntax_Keywords.html
---

## Class keywords

### class

Declares a class.

```
class ExampleClass
{
}
```

### modded

See Modded Class.

```
class ExampleClass {}
 
modded class ExampleClass // inherits and "takes the place" of the original ExampleClass
{
}
```

### sealed

A class marked with **sealed** cannot be inherited from.

```
sealed class Infertile
{
}
 
class Child : Infertile // compilation error
{
}
```

### extends

States that the declared class inherits from another.

```
class ChildClass extends ParentClass
{
}
 
// alias
class ChildClass : ParentClass
{
}
```

### typedef

Declares an alias for a type.

```
typedef string Text;
 
class ExampleClass
{
    Text m_sMyText = "This is my text, that is a string";
}
```

## Method keywords

### proto

Disclose an engine-side function to script (C++ side).

### external

Compiler hint to link API methods - applies to proto functions (C++ side).

### native

Native call convention of internal function (C++ side).

### volatile

Internal function that may call back to script (hint for compiler that context need to be saved on stack).

### private

The method can be called only from inside of the same class' methods. A modded class will still be able to use it (but not its children).

```
class ExampleClass
{
    private void HelloWorld()
    {
        Print("Hello there");
    }
 
    void PublicHelloWorld()
    {
        HelloWorld(); // works from within the object
    }
}
 
ExampleClass classInstance = new ExampleClass();
classInstance.PublicHelloWorld();  // works - calls PublicHelloWorld() that calls HelloWorld()
classInstance.HelloWorld();       // compilation error - HelloWorld() is private and cannot be accessed
```

### protected

The method can be called only from inside of class' methods or methods of its extended (children) classes, or a modded class.

```
class ParentClass
{
    protected void HelloWorld()
    {
        Print("Hello there");
    }
}
 
class ChildClass : ParentClass
{
    void PublicHelloWorld()
    {
        HelloWorld(); // works from within the object
    }
}
 
ChildClass classInstance = new ChildClass();
classInstance.PublicHelloWorld();  // works - calls PublicHelloWorld() that calls HelloWorld()
classInstance.HelloWorld();       // compilation error - HelloWorld() is protected and cannot be accessed
```

### static

The method can be called without object pointer, just by ClassName.methodName(). Only static values/methods can be accessed from a static method.

```
class ExampleClass
{
    static void HelloWorld()
    {
        Print("Hello there");
    }
}
 
ExampleClass.HelloWorld();  // works
 
ExampleClass classInstance = new ExampleClass();
classInstance.HelloWorld();  // works
```

### event

Marks methods which are called from engine side as callbacks. On Entities, also a hint for tools that the function should be exposed as Entity script event.

### override

Indicates overriding of base class method. Compiler checks if method is present in base class and if signature matches.

```
class ParentClass
{
    void HelloWorld()
    {
        Print("Hello world");
    }
}
 
class ChildClass : ParentClass
{
    override void HelloWorld()
    {
        Print("Hello there");
    }
}
 
ParentClass parentClassInstance = new ParentClass();
parentClassInstance.HelloWorld();  // prints "Hello world"
 
ChildClass childClassInstance = new ChildClass();
childClassInstance.HelloWorld();   // prints "Hello there"
```

### sealed (method)

The method cannot be overridden in derived classes.

```
class ParentClass
{
    sealed void HelloWorld()
    {
        Print("Hello there");
    }
}
 
class ChildClass : ParentClass
{
    override void HelloWorld() // compilation error - cannot override a sealed method
    {
        Print("Hello world");
    }
}
```

## Value keywords

### owned

Modifier for returning internal functions. Tells script-VM that returning variable (string or array) must not be released.

### out

Modifier for function parameters. Variable will be changed by function call.

```
void Method(out array<string> parameter)
{
    parameter = new array<string>();
    parameter.Insert("Hello");
    parameter.Insert("there");
}
 
// ...
 
array<string> myArray = null;
Method(array); // myArray is now { "Hello", "there" }
```

### inout

Modifier for function parameters. Variable will be used and then changed by function call.

```
class ExampleClass
{
    private ref array<string> m_aList;
 
    void Method(inout array<string> parameter)
    {
        parameter = new array<string>();
        parameter.Insert("Hello");
        parameter.Insert("there");
 
        m_aList = parameter;
    }
}
 
ExampleClass classInstance = new ExampleClass();
 
array<string> myArray = null;
classInstance.Method(array); // myArray is now { "Hello", "there" } and classInstance.m_aList is referencing myArray
```

### const

Declares a constant that cannot be modified later.

```
const string MY_TEXT = "Hello there";
 
void Method()
{
    Print(MY_TEXT); // will print "Hello there"
    MY_TEXT = "General Kenobi"; // compilation error - a constant cannot be modified
}
```

### reference

Hint for tools (Material editor) that the value may be used as parameter in material.

### notnull

Modifier for function parameter of class pointer type. Function expects parameter will never be null.

```
void Method(notnull IEntity entity) // no need to check for "if (!entity)" inside the method
{
    entity.DoSomething(); // the argument can be accessed safely
}
 
if (myEntity) // it is still needed to check for null before calling the method
    Method(myEntity);
```

### private (value)

Value can only be accessed from the class/instance methods. Mutually exclusive with "protected".

```
class ExampleClass
{
    private bool m_bIsActive;
 
    void SetActive(bool value)
    {
        m_bIsActive = value;
    }
}
 
ExampleClass classInstance = new ExampleClass();
Print(classInstance.m_bIsActive); // compilation error - m_bIsActive is private
```

### protected (value)

Value can only be accessed from the class/instance methods or its children. Mutually exclusive with "private".

```
class ParentClass
{
    protected bool m_bProtected;
    private bool m_bPrivate;
 
    void SetPrivate(bool value)
    {
        m_bPrivate = value;
    }
}
 
class ChildClass : ParentClass
{
    void SetProtected(bool value)
    {
        m_bProtected = value; // works - m_bProtected declaration reaches ChildClass
    }
}
 
ChildClass childClassInstance = new ChildClass();
Print(childClassInstance.m_bProtected); // compilation error - m_bProtected is protected
```

### static (value)

Value belongs to the class, not an instance. If public, accessible via ClassName.variable.

```
class ExampleClass
{
    static bool bStaticValue;
    private static bool m_bPrivate; // good practice to make it protected/private with getters/setters
}
 
Print(ExampleClass.bStaticValue);   // works
ExampleClass.bStaticValue = true;   // works
 
ExampleClass classInstance = new ExampleClass();
Print(classInstance.bStaticValue);   // works
classInstance.bStaticValue = true;   // works
```

### autoptr

Modifier for class/pointer values. Pointed non-managed object will be automatically destroyed upon end of variable lifetime. For *Managed* objects use *ref* keyword.

### ref

Strong reference - see Enforce Automatic Reference Counting for more information.

## Code keywords

### new

Creates an instance of the provided class.

```
class ExampleClass
{
}
 
ExampleClass classInstance = new ExampleClass();
```

```
class ExampleClass
{
    void ExampleClass(int myValue) // a method named from the class is the constructor
    {
        Print(myValue);
    }
}
 
ExampleClass classInstance = new ExampleClass(5); // needs Constructor-defined arguments
```

### delete

Deletes an object and sets all its references to null.

### thread

Creates a thread (parallel processing) from a method. Returns the thread, **not** the method value.

### null

A null value.

### this

Refers to the method's object itself.

```
class ExampleClass
{
    void MethodA()
    {
        this.MethodB();
    }
 
    void MethodB()
    {
        Print("It works");
    }
}
```

### super

Refers to the **parent** class for the requested variable/function.

```
class Gen1
{
    void Method()
    {
        Print("Gen1 method");
    }
}
 
class Gen2 : Gen1
{
    void Method()
    {
        Print("Gen2 method");
    }
}
 
class Gen3 : Gen2
{
    void Method()
    {
        super.Method(); // prints "Gen2 method"
    }
}
```

### continue

Used to skip to the next iteration in for, foreach, while loops.

```
class ExampleClass
{
    // this method will print 0, 1, 2, 4, 5
    void Skip3()
    {
        for (int i; i < 5; i++)
        {
            if (i == 3)
                continue;
 
            Print(i.ToString());
        }
    }
 
    void MethodB()
    {
        foreach (IEntity entity : m_aEntities)
        {
            if (!entity)
                continue;
 
            Print(entity);
        }
    }
}
```

### return

Exits the method immediately with the provided value if any.

```
class ExampleClass
{
    void PrintHelloThere(bool print)
    {
        if (!print)
            return; // ends here if print is false
 
        Print("Hello there");
    }
 
    float GetDistanceNotZero(vector v1, vector v2)
    {
        if (v1 == v2)
            return -1;
 
        return vector.Distance(v1, v2);
    }
}
```
