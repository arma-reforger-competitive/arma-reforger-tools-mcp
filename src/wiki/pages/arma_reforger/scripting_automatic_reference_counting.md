---
title: Arma Reforger:Scripting: Automatic Reference Counting
url: https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Automatic_Reference_Counting
---

**Automatic Reference Counting**, shortened to **ARC**, is a memory management solution used in Enforce Script. Enforce Script does not use Garbage Collection (a.k.a GC).

In comparison with the widely used https://en.wikipedia.org/wiki/Tracing_garbage_collection (in e.g C#, Java):
- it does not suffer from cleanup stuttering (lags due to high GC load)
- memory management performance is not dependent on the number of managed instances
- it is simple to implement (having a good working GC is a big issue)

On the negative side, it is open to the cyclic reference problem, described in the Cyclic Reference paragraph below.

**Note:** This whole system impacts **objects** passed by **reference** - see [Value Types](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Values#Types).

## Principle

Enfusion has an internal counter of strong references to objects; when this counter reaches 0, the object is released from memory.

### Strong Reference

A strong reference is a reference to an object that increments the reference counter.
The referenced object **cannot** become null during the referencing object's lifetime unless manually deleted (with the `delete` keyword).

Strong references:
- using the `ref` keyword
- scope lifetime's script reference

### Weak Reference

A weak reference is a reference to an object that does not increment the reference counter. The reference may become null during the program's lifetime.

## Usage

```enforce
class ExampleClass
{
	// weak reference to an object
	ReferencedClass m_WeakRefObject;

	// strong reference to an object
	ref ReferencedClass m_StrongRefObject;

	// strong reference to a dynamic array (that is an object itself) of weak references
	ref array<ReferencedClass> m_aWeakRefArray;

	// strong reference to dynamic array of strong references
	ref array<ref ReferencedClass> m_aStrongRefArray;

	// static array of strong references
	ref ReferencedClass m_aStrongRefStaticArray[10];

	// maps and sets work the same
	ref map<string, ReferencedClass> m_mWeakRefMap;
	ref map<string, ref ReferencedClass> m_mStrongRefMap;
	ref set<ReferencedClass> m_sWeakRefSet;
	ref set<ref ReferencedClass> m_sStrongRefSet;

	void Method()
	{
		ReferencedClass localReference = new ReferencedClass();
		localReference = new ReferencedClass();	// the previous object loses its strong reference and is immediately released
												// the newly created object replaces it

		array<ReferencedClass> arrayOfWeakRef = new array<ReferencedClass>(); // array of weak references is created
		arrayOfWeakRef.Insert(new ReferencedClass());
		// ReferencedClass object is created and immediately deleted, as arrayOfWeakRef does not increment the reference counter

		array<ref ReferencedClass> arrayOfStrongRef = new array<ref ReferencedClass>(); // array of strong references is created
		arrayOfStrongRef.Insert(new ReferencedClass()); // ReferencedClass object is created and kept

		// at the end of the method, local variables 'localReference', 'arrayOfWeakRef' and 'arrayOfStrongRef' are released:
		// localReference is destroyed
		// arrayOfWeakRef is destroyed (it only contains NULL),
		// arrayOfStrongRef is destroyed (it contains the last strong ref to object, so its contained object is destroyed as well
	}
}
```

### Cyclic Reference

#### Problem

The main issue with reference counting is cyclic references (aka "Island of isolation").
In the code below:
- the main method creates object A and object B
- object A references object B
- object B references object A
- the main method ends, dropping its references to object A and B
- both objects a and b should be deleted but they still reference each other, keeping the reference counting above 0:
  - any reference to them is lost after main method's ending
  - they remain in memory until the program is stopped

```enforce
class Parent
{
	ref Child m_Child;
}

class Child
{
	ref Parent m_Parent;
}

void main()
{
	Parent a = new Parent();	// 'a' has 1 reference
	Child b = new Child();		// 'b' has 1 reference
	a.m_Child = b;				// 'b' has 2 references
	b.m_Parent = a;				// 'a' has 2 references

	// local variables 'a', 'b' are released (reference count is decreased by 1)
	// both objects remain in memory, having 1 reference each (each other)
}
```

#### Solution

The solution to this issue is to have one object have a strong reference, while the other only holds a weak one:

```enforce
class Parent
{
	ref Child m_Child;
}

class Child
{
	Parent m_Parent;
}

void main()
{
	Parent a = new Parent();	// 'a' has 1 strong (scope) reference
	Child b = new Child();		// 'b' has 1 strong (scope) reference
	a.m_Child = b;				// 'b' has 2 strong references
	b.m_Parent = a;				// 'a' has 1 strong reference, 1 weak reference

	// local variables 'a', 'b' are released (reference count is decreased by 1)
	// a only has 1 weak reference and is released from memory
	// b loses a's reference and has no more reference to it - it gets released
}
```

Using weak references makes null-checking take more importance in scripting.
