---
title: Enforce script language: Enforce Automatic Reference Counting
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Enforce_EnforceScriptARC.html
---

Enforce has support of automatic reference counting. In a spirit of flexibility a developer can choose if his class should or shouldn't be managed, simply by inheriting or not from Managed class. Use of managed classes can be also enforced for whole script modules (e.g. we can force to use managed classes for gameplay module and for mods)

Simple "C++" like classes remains as an option for high performance, but less secure scripts.

- raw power of Enforce!
- objects referenced by plain C pointers
- no automatic memory management, owner must delete them
- suitable to use by programmers and experienced scripters/designers

For common gameplay scripts/mods/etc there are managed objects, which are

- slightly slower (due indirect weak pointer object accessing), but still fast enough for most situations
- internally ref-counted and automatically deleted when no needed.
- objects referenced by weak pointers (pointer is always valid or NULL, never points to deleted memory etc.)
- suitable to use by everyone

## Automatic Reference Counting

Reference counting is an one of the solutions for "garbage collecting". In compare with widely used [Tracing garbage collection](https://en.wikipedia.org/wiki/Tracing_garbage_collection) (Java, C#):

- it doesn't suffer by "stop the world" problem (lags due high load GC processing)
- memory management performance is not dependent by number of managed instances
- it's simple to implement (make good working GC it's big issue)
- used by e.g. Swift language
- suffer by cyclic references problem

### Island of isolation

Main issue with reference counting are cyclic references (aka "Island of isolation").

```
// Let's have a script where all references are increasing/decreasing reference count
 
class Parent
{
    ref Child m_child;
};
 
class Child
{
    ref Parent m_parent;
};
 
void main()
{
    Parent a = new Parent(); // 'a' has 1 reference
    Child b = new Child(); // 'b' has 1 reference
    a.m_child = b; // 'b' has 2 references
    b.m_parent = a; // 'a' has 2 references
 
    // local variables 'a', 'b' are released (reference count is decreased by 1)
}
```

In the code above, in the end of function *main* should be both objects **a** and **b** deleted, but both objects has still reference count '1' so remains in memory even these objects are unreachable.

### Strong and weak references

Solution for cyclic references problem is concept of strong and weak references:

- strong reference increase reference count: holds object alive
- weak reference just pointing to an object, but doesn't increase reference count

By using strong and weak references, user give a hint for compiler, how to break a reference cycle. Let's have a **ref** keyword for marking strong references:

```
class Parent
{
    ref Child m_child; // putting 'ref' keyword, we give a hint to compiler, that this is strong reference.
};
 
class Child
{
    Parent m_parent; // this reference is weak reference (there is no 'ref')
};
 
void main()
{
    Parent a = new Parent(); // 'a' has 1 strong reference (local vars are strong by default)
    Child b = new Child(); // 'b' has 1 reference (local vars are strong by default)
    a.m_child = b; // 'b' has 2 strong references, because Parent.m_child is strong ref
    b.m_parent = a; // 'a' has still just 1 strong reference, because Child.m_parent is weak reference
 
    // local variables 'a', 'b' are released (reference count is decreased by 1)
}
```

In the code above, in the end of function *main* object 'a' has zero strong references, thus is deleted, in destructor release *m_child*, so object 'b' has also zero strong references and is deleted too. No leak.

## Usage

In the Enforce by default are all variables weak references, **ref** keyword is marking that variable is strong reference. In some special cases, variables are strong references by default:

1. local variables inside functions
1. function arguments
1. function return value

and are released after their scope ends.

While an object is stored in at least one strong reference, it's being kept alive. When the last strong reference is destroyed or overwritten, the object is destroyed and all other (only weak refs left) references are set to NULL.

When an object is deleted manually by **delete** command (e.g., 'delete a;'). The object is deleted immediately ignoring reference count, and all references (weak and strong) are set to NULL.

**Remarks:** **Best practice:** Optimal usage of references in Enforce is to have exactly one strong reference per object, placed in "owner" object who creates it. This ensures no cyclic references and proper order of object destroying: the object is destroyed when its "creator" is destroyed.

### Examples

**In functions**

```
class MyClassA
{
    void MyClassA() { Print("MyClassA()"); } // constructor
    void ~MyClassA() { Print("~MyClassA()"); } // destructor
};
 
void function1()
{
    MyClassA a = new MyClassA(); // 'MyClassA()'
    a = new MyClassA();
    // new object is created 'MyClassA()',
    // 'a' is overwritten,
    // first instance of MyClass is released: '~MyClassA()'
    // any code here
 
    // the end of function, local variable 'a' is released: '~MyClassA()'
}
 
ref MyClassA g_MyGlobal;
 
void function2()
{
    MyClassA a = new MyClassA(); // 'MyClassA()'
    g_MyGlobal = a;
 
    // the end of function, local variable 'a' is released, but there is still one strong ref g_MyGlobal,
    // so object will live until end of this script module.
}
 
void PrintMyObj(MyClassA o)
{
    Print("object is " + o);
}
 
void function3()
{
    MyClassA a = new MyClassA(); // 'MyClassA()'
    PrintMyObj(a); // 'object is MyClassA<address>'
 
    // the end of function, local variable 'a' is released: '~MyClassA()'
}
 
void function4()
{
    PrintMyObj(new MyClassA());
    // object is created 'MyClassA()',
    // PrintMyObj function is called 'object is MyClassA<address>'
    // and right after PrintMyObj function finish, temporary object is released '~MyClassA()'
}
 
MyClassA CreateMyObj()
{
    return new MyClassA();
}
 
void function5()
{
    MyClassA a = CreateMyObj(); // object is created inside CreateMyObj function 'MyClassA()'
    // any code here
 
    // the end of function, local variable 'a' is released: '~MyClassA()'
}
 
void function6()
{
    array<MyClassA> pole1 = new array<MyClassA>(); // array of weak references is created
    pole1.Insert(new MyClassA());
    // object is created 'MyClassA()',
    // but pole1 is array of weak references, so no one is keeping this object alive,
    // its deleted immediately '~MyClassA()'
 
    array<ref MyClassA> pole2 = new array<ref MyClassA>(); // array of strong references is created
    pole2.Insert(new MyClassA()); // object is created 'MyClassA()'
 
    // any code here
 
    // the end of function, local variables 'pole1' and 'pole2' are released:
    // pole1 is destroyed (it contains just NULL),
    // pole2 is destroyed (it contains last strong ref to object, so object is destroyed '~MyClassA()')
}
```

**Classes and arrays**

```
class ExampleClass
{
    ClassA m_a;

    ref ClassA m_b;

    ref array<ClassA> m_c;

    ref array<ref ClassA> m_d;

    ref ClassA m_e[10];
};
```
