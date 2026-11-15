---
title: Enforce script language: Coding Guidelines
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Enforce_CodingGuidelines.html
---

## Conventions

### Rule: Writing curly parentheses

- All curly parentheses should be written in a new line

**Bad**

```
class MyClass {
    void PrintMsg( string msg ){
        Print( msg );
    }
}
```

**Good**

```
class MyClass
{
    void PrintMsg( string msg )
    {
        Print( msg );
    }
}
```

### Rule: Constants

- All names of constants must be written with uppercase letters and words separated by an underscore
- Constant should have a minimum of two words, the first word should define category and the second word should defined name of constant

**Bad**

```
const int WindowWidth = 10;
const int Window_Height = 20;
```

**Good**

```
const int WINDOW_WIDTH  = 10;
const int WINDOW_HEIGHT = 20;
```

### Rule: Global variables

- For declaration of global variable use g_ prefix (as global) and all next words start with uppercase letter
- Note: Global variables should be avoided

**Bad**

```
DayZGame    Game;
Mission     global_Missison;
```

**Good**

```
DayZGame    g_Game;
Mission     g_MissionGame;
```

### Rule: Member variables in class

- For declaration of member variable of class use m_ prefix (as member) and all next words start with uppercase letter

**Bad**

```
DayZGame    dayZGame;
Mission     mission_game;
```

**Good**

```
DayZGame    m_DayZGame;
Mission     m_MissionGame;
```

### Rule: Local variables in functions/methods

- Local variables should be written with a lowercase. All next words are separated with underscore

**Bad**

```
object  item;
string  itemName;
```

**Good**

```
object item;
string item_name;
```

### Rule: Functions/Methods

- All functions/methods begin with big letter and every next word starts again with an uppercase letter

**Bad**

```
void add_item( string item_name )
{
    Object new_object = g_Game.CreateObject( item_name, "0 0 0" );
}
```

**Good**

```
void AddItem( string item_name )
{
    Object new_object = g_Game.CreateObject( item_name, "0 0 0" );
}
```

### Rule: Classes

- Class name starts with an uppercase letter and every next word starts again with an uppercase letter
- Every class should have its own file. File name must have same name like class name

**Bad:**

*playerScripts.h*

```
class Action
{
    void ThrowMeat()
    {
        DoSomethingElse();
    }
}
 
class Scripted_Player
{
    Action m_PlayerAction;
 
    void CallDog( string senderName )
    {
        m_PlayerAction = new Action;
        m_PlayerAction.ThrowMeat();
    }
}
```

**Good:**

*Action.h*

```
class Action
{
    void ThrowMeat()
    {
        DoSomethingElse();
    }
}
```

*ScriptedPlayer.h*

```
class ScriptedPlayer
{
    Action m_PlayerAction;
 
    void CallDog( string sender_name )
    {
        m_PlayerAction = new Action;
        m_PlayerAction.ThrowMeat();
    }
}
```

## Recommendations

### Variables aligning and sorting

- Align variables with TAB. Do not use spaces
- TAB should be set to 4 TAB size. 2 is too short and 8 is too long
- When the name of a type is too long, separate it with empty line
- Declaration of variables should be sorted by type

**Bad**

```
string m_Msg;
int m_Player;
SomeVeryLongClass m_SometingLong;
float m_SomeValue;
Action m_PlayerActions;
string m_UserName;
int m_Counter;
float m_TimeElapsed;
ScriptedPlayer m_ScrPlayer;
```

**Good**

```
int     m_Player;
int     m_Counter;
float   m_SomeValue;
float   m_TimeElapsed;
string  m_UserName;
string  m_Msg;
 
SomeVeryLongClass   m_SometingLong;
ScriptedPlayer      m_ScrPlayer;
```

- Single declaration of variable can be written with only single space

**Good**

```
class MyClass
{
    int m_Counter;
}
```

### IF/While Condition

- Always write conditions with parentheses
- Always use space between operator if, parentheses and condition expression
- else statement should be written on new line like curly parentheses

**Bad**

```
if(true) {
    DoSomething();
} else {
    DoSomethingElse();
}
 
while(true) {
    Print("trolling");
}
```

**Good**

```
if( con1 )
{
    DoSomethingFirst();
}
else if( con2 )
{
    DoSometingSecond();
}
else
{
    DoSometingElse();
}
 
while( true )
{
    Print( "trolling" );
}
```

### For loop

- Write spaces like in if condition statement

**Bad**

```
for(int i=0;i<=10;i++) {
    Print( "Loop: "+ itoa(i) );
}
```

**Good**

```
for( int i = 0; i <= 10; i++ )
{
    Print( "Loop: "+ itoa(i+2) +"." );
}
```

### Writing names of Classes, Functions and Variables

- Names should be clear, short and meaningful

**Bad**

```
class NewScriptedLandMine
{
    string m_LastPlayerOnLandMine;
 
    void ExecuteExplosionAfterStep( string sender_name )
    {
        m_LastPlayerOnLandMine = sender_name;
        DoBigExplosive();
    }
}
```

**Good**

```
class ScriptedMine
{
    string m_LastPlayer;
 
    void Explode( string sender_name )
    {
        m_LastPlayer = sender_name;
        DoExplosive();
    }
}
```
