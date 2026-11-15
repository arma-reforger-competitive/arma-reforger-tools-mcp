---
title: Arma Reforger:Scripting: Do's and Don'ts
url: https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Do's_and_Don'ts
---

****

| Don't | Do |
| --- | --- |
| `class ExampleClass {`<br>`int LProcLgh;`<br>`int stringlength( string Value ){`<br>`LProcLgh= Value.Length();`<br>`Print ( "String length obtained: "+LProcLgh );`<br>`return LProcLgh;`<br>`}`<br>`}` | `class ExampleClass`<br>`{`<br>`protected int m_iLastProcessedLength;`<br>`int GetStringLength(string value)`<br>`{`<br>`m_iLastProcessedLength = value.Length();`<br>`Print("String length obtained: " + m_iLastProcessedLength);`<br>`return m_iLastProcessedLength;`<br>`}`<br>`int GetLastProcessedLength()`<br>`{`<br>`return m_iLastProcessedLength;`<br>`}`<br>`}` |

****

| Don't | Do |
| --- | --- |
| `class ExampleClass`<br>`{`<br>`protected int m_iLength;`<br>`int GetStringLength(string name)`<br>`{`<br>`m_iLength = name.Length();`<br>`Print("String length obtained: " + m_iLength);`<br>`return m_iLength;`<br>`}`<br>`}` | `class ExampleClass`<br>`{`<br>`// keeping the variable into method scope and away from the instance`<br>`int GetStringLength(string name)`<br>`{`<br>`int length = name.Length();`<br>`Print("String length obtained: " + length);`<br>`return length;`<br>`}`<br>`}` |

****

| Don't | Do |
| --- | --- |
| `switch (value)`<br>`{`<br>`case 0:`<br>`{`<br>`Print("0");`<br>`break;`<br>`}`<br>`default:`<br>`{`<br>`Print("default");`<br>`break;`<br>`}`<br>`}` | `switch (value)`<br>`{`<br>`case 0:`<br>`Print("0");`<br>`break;`<br>`default:	// brackets are needed only if variables are declared in the 'case' code`<br>`{			// otherwise scoping is superfluous (and very slightly impacts performance)`<br>`string message = "default";`<br>`Print(message);`<br>`break;`<br>`}`<br>`}` |

****

| Don't | Do |
| --- | --- |
| `// this array only lists pointers but does not increase the reference count`<br>`array<ExampleClass> classArray = {};`<br>`ExampleClass newInstance;`<br>`for (int i; i < 10; i++)`<br>`{`<br>`newInstance = new ExampleClass();`<br>`classArray.Insert(newInstance);`<br>`// newInstance will be deleted at the end of the scope`<br>`// as there are no references to it`<br>`}` | `// this array keeps a strong reference to its items`<br>`array<ref ExampleClass> classArray = {};`<br>`ExampleClass newInstance;`<br>`for (int i; i < 10; i++)`<br>`{`<br>`newInstance = new ExampleClass();`<br>`classArray.Insert(newInstance);`<br>`// classArray keeps a strong reference to newInstance - it will not be cleared`<br>`}` |

****

| Don't | Do |
| --- | --- |
| **Note:** Here, both ParentClass and ChildClass have a strong reference to each other, keeping the reference count above zero - creating an "island of isolation" (see [Arma Reforger:Scripting: Automatic Reference Counting](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Automatic_Reference_Counting) for more information).<br>`class MainClass`<br>`{`<br>`ref SubClass m_SubClass;`<br>`void MainClass()`<br>`{`<br>`m_SubClass = new SubClass(this);`<br>`}`<br>`}`<br>`class SubClass`<br>`{`<br>`ref MainClass m_Parent;`<br>`void SubClass(MainClass parent)`<br>`{`<br>`m_Parent = parent;`<br>`}`<br>`void DoSomething()`<br>`{`<br>`Print(m_Parent);`<br>`}`<br>`}` | **Note:** Here, the MainClass needs the SubClass (it creates it in its constructor meaning it needs it to work) but the subClass does not require MainClass - if the MainClass reference does not exist, it will simply not use it.<br>`class MainClass`<br>`{`<br>`ref SubClass m_SubClass;`<br>`void MainClass()`<br>`{`<br>`m_SubClass = new SubClass(this);`<br>`}`<br>`}`<br>`class SubClass`<br>`{`<br>`MainClass m_Parent; // ref removed`<br>`void SubClass(MainClass parent)`<br>`{`<br>`m_Parent = parent;`<br>`}`<br>`void DoSomething()`<br>`{`<br>`if (!m_Parent) // null safety check`<br>`return;`<br>`Print(m_Parent);`<br>`}`<br>`}` |
