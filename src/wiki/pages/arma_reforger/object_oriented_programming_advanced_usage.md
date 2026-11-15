---
title: Arma Reforger:Object Oriented Programming Advanced Usage
url: https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Advanced_Usage
---

**Note:** This guideline requires the understanding of [Arma Reforger:Object Oriented Programming Basics](https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Basics).

## Casting

Casting is the act of "presenting" a value as another type. For example, if a class hierarchy is `Animal > Dog > Cocker`, a dog is an animal, a cocker is a dog (that is an animal), but a dog is not especially a cocker.

```enforce
class Animal {}
class Dog : Animal {}
class Cocker : Dog {}
class Labrador : Dog {}
```

### Upcasting

Upcasting means seeing the class as one of its parents:

```enforce
void Method()
{
	Cocker cocker = new Cocker();
	Animal animal = cocker;			// OK, as a cocker is an animal
	string sentence = cocker;		// error: cocker does -not- inherit from string
}
```

### Downcasting

Downcasting means seeing a parent class as a specific child - this must be done by manually casting:

```enforce
void Method(Dog dog)
{
	Cocker cocker1 = dog;				// error: "Dog" is too generic to be casted as Cocker - it could be e.g a Labrador
	Cocker cocker2 = Cocker.Cast(dog);	// OK: manual casting tells the code "the developer knows what he is doing"
										// if 'dog' is not castable as a Cocker, null is returned - the code does not crash
}
```

### Manual Casting

```enforce
void Method()
{
	float value1 = 4.9;
	int value2 = value1;						// value2 = 4 as integer casting -truncates- the value, not rounds it
	string value3 = "result = " + (bool)value2;	// "result = true", as a non-zero integer is true when casted to bool
}
```

## Template

A template is a class that allows a generic management for multiple types. Its methods cannot assume anything about the type.

**Note:** The generic type is by convention declared by the `T` letter.

```enforce
// template class Item with generic type T
class Item<Class T>
{
	protected T m_Data;

	void Item(T data)
	{
		m_Data = data;
	}

	void SetData(T data)
	{
		m_Data = data;
	}

	T GetData()
	{
		return m_Data;
	}

	void PrintData()
	{
		Print(m_Data);
	}
}
```

```enforce
void Method()
{
	Item<string> stringItem = new Item<string>("Hello!");	// template class Item declared with type "string". In Item<string> class, all Ts are substituted with 'string'
	Item<int> intItem = new Item<int>(72);					// template class Item declared with type "int". In Item<int> class, all Ts are substituted with 'int'

	stringItem.PrintData();		// prints "m_Data = 'Hello!'"
	intItem.PrintData();		// prints "m_Data = 72"
}
```

## Modding

A mod can inherit/replace an existing class with the use of the `modded` keyword.<br>
It is used to inject inherited class into class hierarchy without modifying other scripts (especially suitable in modding).
A modded class behaves like a class inherited from the original class (one can use [super](https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Basics#super) to access the original class) but also allows `private` methods and functions access and modification.
When a modded class is declared, the modded class will be instantiated instead of the original class.

**Important:** Only classes within the same module can be modded (to mod a class in e.g `GameLib` module, the modded class has to be placed in the `GameLib` module).

```enforce
// game
class A
{
	private string m_sPrivateString = "something said";

	void Say()
	{
		Print("original Say method");
		Print(m_sPrivateString);
	}
}

void Test()
{
	A a = new A();	// "class A" is instantiated
	a.Say();		// prints "original Say method" then "something said"
}

// mod
modded class A // this class automatically inherits from the original class A
{
	override void Say()
	{
		m_sPrivateString = "modded said";
		Print("modded Say method");
		super.Say();
	}
}

void TestModded()
{
	A a = new A();	// "modded class A" is instantiated
	a.Say();		// prints "modded Say method" then "original Say method" and "modded said"
}
```

### Precedence

Multiple mods can edit the same class if loaded together. In such case, vanilla class gets modded in order by mod1, mod2, mod3.
Mod loading order is defined by dependency (if mod2 requires mod1, mod1 is loaded first) - if no dependencies are involved, order is not guaranteed.

| Vanilla | Mod 1 | Mod 2 |
| --- | --- | --- |
| `class SCR_ExampleClass`<br>`{`<br>`string GetMessage()`<br>`{`<br>`return "Vanilla message";`<br>`}`<br>`}` | `modded class SCR_ExampleClass`<br>`{`<br>`override string GetMessage()`<br>`{`<br>`return super.GetMessage() + ", Mod A";`<br>`}`<br>`}` | `modded class SCR_ExampleClass`<br>`{`<br>`override string GetMessage()`<br>`{`<br>`return super.GetMessage() + ", Mod B";`<br>`}`<br>`}` |

| If Mod B requires Mod A, the load order will be Vanilla > Mod A > Mod B. | `instance.GetMessage(); // Vanilla message, Mod A, Mod B` |
| --- | --- |
| If Mod A requires Mod B, the load order will be Vanilla > Mod B > Mod A (obviously). | `instance.GetMessage(); // Vanilla message, Mod B, Mod A` |
| If no dependencies are defined between mods, the load order is arbitrary. | `// can be one of`<br>`instance.GetMessage(); // Vanilla message, Mod A, Mod B`<br>`// or`<br>`instance.GetMessage(); // Vanilla message, Mod B, Mod A` |
