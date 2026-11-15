---
title: Arma Reforger:Scripting: Performance
url: https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Performance
---

Performance is an important aspect of scripting, as poor performance can tank the whole gaming experience, if not the game itself.

- What is tanking performances
  - Crazy requests
  - Memory saturation
  - Redundant code operations
- How to benchmark/identify what slows down things
  - What is acceptable?
  - What is not?

## Performance Enemies

Many if not all of the usual performance issues can see some of these questions asked:
- is it *needed?
- is it *needed there?*
- is it *needed that much?*
- is it *needed immediately?*
- is it *needed that frequently?*

and the respective solutions to positive replies to these questions would be:
- don't do it: quite straightforward, can mean to remove the code or to replace it with a simpler code
- don't do it *there*: the concerned class might be dealing with responsibilities that are out of its scope
- don't do it *that much*: the structure can be rethought in order to have fewer operations to be executed
- don't do it *in one frame*: the calculation can be spread over multiple frames and result delayed and cached
- don't do it *that frequently*: the code can be set to run every X seconds, result being delayed and cached until the next iteration

### Not Needed

#### Problem

Partially unneeded calculation is done.

#### Solution

- Remove the unneeded code
- Split the code to only do needed processing in each case

### Misplaced

#### Problem

A calculation/storage is done on every instance whereas calculation could be centralised.

#### Solution

Move the responsibility and the data to a dedicated class (**S** from https://en.wikipedia.org/wiki/SOLID, **S**ingle **R**esponsibility **P**rinciple).

#### Example

| Original | Optimised |
| --- | --- |
| `class PerformanceExample : IEntity`<br>`{`<br>`protected ref array<ref PotentialTarget> m_aBigObjectList = { /* ... */ };`<br>`protected array<ref PotentialTarget> GetTargets()`<br>`{`<br>`array<ref PotentialTarget> result = {};`<br>`foreach (PotentialTarget object : m_aBigObjectList)`<br>`{`<br>`if (object.IsEnemyTo(this.GetFaction()))`<br>`result.Insert(object);`<br>`}`<br>`return result;`<br>`}`<br>`}` | `class EnemyManager`<br>`{`<br>`protected ref map<Faction, ref array<ref PotentialTarget>> m_mTargetMap;`<br>`void EnemyManager(array<ref PotentialTarget> potentialTargets)`<br>`{`<br>`m_mTargetMap = new map<Faction, ref array<ref PotentialTarget>>();`<br>`array<ref PotentialTarget> factionTargets;`<br>`foreach (Faction faction : Faction.GetAllFactions())`<br>`{`<br>`factionTargets = {};`<br>`foreach (PotentialTarget potentialTarget : potentialTargets)`<br>`{`<br>`if (potentialTarget.IsEnemyTo(faction))`<br>`factionTargets.Insert(potentialTarget);`<br>`}`<br>`m_mTargetMap.Set(faction, factionTargets);`<br>`}`<br>`}`<br>`array<ref PotentialTarget> GetTargets(Faction faction)`<br>`{`<br>`return m_mTargetMap.Get(faction);`<br>`}`<br>`}`<br>`class PerformanceExample : IEntity`<br>`{`<br>`protected ref EnemyManager m_EnemyManager;`<br>`void PerformanceExample(notnull EnemyManager enemyManager)`<br>`{`<br>`m_EnemyManager = enemyManager;`<br>`}`<br>`protected array<ref PotentialTarget> GetTargets()`<br>`{`<br>`return m_EnemyManager.GetTargets(this.GetFaction());`<br>`}`<br>`}` |

### Ill-Conceived

#### Problem

A big array of data is processed at once. Going through all the data is not needed.

#### Solution

Stop as soon as an acceptable result has been obtained.

#### Example

| Original | Optimised |
| --- | --- |
| `class PerformanceExample : IEntity`<br>`{`<br>`protected ref array<ref AliveOrNotObject> m_aBigObjectList = { /* ... */ };`<br>`protected int GetNumberOfAliveObjects()`<br>`{`<br>`int result;`<br>`foreach (AliveOrNotObject object : m_aBigObjectList)`<br>`{`<br>`if (object.IsAlive())`<br>`result++;`<br>`}`<br>`return result;`<br>`}`<br>`bool IsAnObjectAlive()`<br>`{`<br>`// heavy: goes through all objects to know if -one- is alive`<br>`return GetNumberOfAliveObjects() > 0;`<br>`}`<br>`}` | `class PerformanceExample : IEntity`<br>`{`<br>`protected ref array<ref AliveOrNotObject> m_aBigObjectList = { /* ... */ };`<br>`bool IsAnObjectAlive()`<br>`{`<br>`foreach (AliveOrNotObject object : m_aBigObjectList)`<br>`{`<br>`if (object.IsAlive())`<br>`return true; // stops on the first occurence`<br>`}`<br>`return false; // goes through all instances to reach false - less likely`<br>`}`<br>`}` |

### Spread

#### Problem

A big array of data is processed at once. The calculation itself is not the issue, the amount of items is.<br>
**or**<br>
A heavy calculation is done, and only one such operation should happen per frame.

#### Solution

Spread the calculation over multiple frames.

#### Example 1

Array size issue:

| Original | Optimised |
| --- | --- |
| `class PerformanceExample : IEntity`<br>`{`<br>`protected ref array<ref AliveOrNotObject> m_aBigObjectList = { /* ... */ };`<br>`void EOnInit(IEntity owner)`<br>`{`<br>`GetGame().GetCallqueue().CallLater(PrintResult, 250, true);`<br>`}`<br>`int GetNumberOfAliveObjects()`<br>`{`<br>`int result = 0;`<br>`foreach (AliveOrNotObject object : m_aBigObjectList)`<br>`{`<br>`if (object.IsAlive())`<br>`result++;`<br>`}`<br>`return result;`<br>`}`<br>`void PrintResult()`<br>`{`<br>`Print("Alive objects = " + GetNumberOfAliveObjects());`<br>`}`<br>`}` | `class PerformanceExample : IEntity`<br>`{`<br>`protected ref array<ref AliveOrNotObject> m_aBigObjectList = { /* ... */ };`<br>`protected ref array<ref AliveOrNotObject> m_aObjectsToBeProcessed = {};`<br>`protected bool m_bIsCalculating;`<br>`protected int m_iTempResult;`<br>`protected int m_iAliveCountBuffer;`<br>`void EOnInit(IEntity owner)`<br>`{`<br>`Calculate();`<br>`GetGame().GetCallqueue().CallLater(PrintResult, 250, true);`<br>`}`<br>`void EOnFrame(IEntity owner, float timeSlice)`<br>`{`<br>`if (m_bIsCalculating)`<br>`Calculate();`<br>`// other frame things`<br>`// ...`<br>`}`<br>`void Calculate()`<br>`{`<br>`int objCount = m_aObjectsToBeProcessed.Count();`<br>`if (objCount < 1)`<br>`{`<br>`m_bIsCalculating = true;`<br>`m_iTempResult = 0;`<br>`m_aObjectsToBeProcessed.Copy(m_aBigObjectList);`<br>`}`<br>`// max 10k items slice`<br>`for (int i, maxCount = Math.Min(objCount, 10000); i < maxCount; i++)`<br>`{`<br>`AliveOrNotObject object = m_aObjectsToBeProcessed[0];	// always @ index 0`<br>`// for we remove the first item later`<br>`if (object.IsAlive())`<br>`m_iTempResult++;`<br>`m_aObjectsToBeProcessed.Remove(0);`<br>`objCount--;`<br>`}`<br>`if (objCount < 1)`<br>`{`<br>`m_iAliveCountBuffer = m_iTempResult;`<br>`GetGame().GetCallqueue().CallLater(Calculate, 1000); // restart calculation in 1 second`<br>`m_bIsCalculating = false;`<br>`}`<br>`}`<br>`int GetNumberOfAliveObjects()`<br>`{`<br>`return m_iAliveCountBuffer;`<br>`}`<br>`void PrintResult()`<br>`{`<br>`Print("Alive objects = " + GetNumberOfAliveObjects());`<br>`}`<br>`}` |

#### Example 2

Heavy calculation issue:

| Original | Optimised |
| --- | --- |
| `class PerformanceExample : IEntity`<br>`{`<br>`protected ref array<ref Player> m_aPlayers = { /* ... */ };`<br>`protected ref map<ref Player, bool> m_mCastResult;`<br>`protected BaseWorld m_World;`<br>`void PerformanceExample()`<br>`{`<br>`m_mCastResult = new map<ref Player, bool>();`<br>`m_World = GetGame().GetWorld();`<br>`}`<br>`void CheckLineOfSight()`<br>`{`<br>`float result;`<br>`TraceParam traceParam;`<br>`foreach (Player player : m_aPlayers)`<br>`{`<br>`traceParam = new TraceParam();`<br>`traceParam.Start = this.Position();`<br>`traceParam.End = player.Position();`<br>`// a Trace is a very expensive operation`<br>`result = m_World.TraceMove(traceParam, null);`<br>`m_mCastResult.Set(player, result == 1);`<br>`}`<br>`}`<br>`}` | `class PerformanceExample : IEntity`<br>`{`<br>`protected ref array<ref Player> m_aPlayers = { /* ... */ };`<br>`protected ref map<ref Player, bool> m_mCastResult;`<br>`protected BaseWorld m_World;`<br>`protected int m_iIndex = -1;`<br>`protected bool m_bIsCalculating;`<br>`void PerformanceExample()`<br>`{`<br>`m_mCastResult = new map<ref Player, bool>();`<br>`m_World = GetGame().GetWorld();`<br>`}`<br>`void EOnFrame(IEntity owner, float timeSlice)`<br>`{`<br>`if (m_bIsCalculating)`<br>`CheckLineOfSight();`<br>`}`<br>`void CheckLineOfSight()`<br>`{`<br>`m_iIndex++;`<br>`m_bIsCalculating = m_iIndex < m_aPlayers.Count();`<br>`if (!m_bIsCalculating)`<br>`{`<br>`m_iIndex = -1;`<br>`return;`<br>`}`<br>`Player player = m_aPlayers[m_iIndex];`<br>`TraceParam traceParam = new TraceParam();`<br>`traceParam.Start = this.Position();`<br>`traceParam.End = player.Position();`<br>`// only one Trace per frame`<br>`float result = m_World.TraceMove(traceParam, null);`<br>`m_mCastResult.Set(player, result == 1);`<br>`}`<br>`}` |

### Immediate Calculation

#### Problem

A heavy operation is done frequently.

#### Solution

Do the heavy operation once, store its result in a member variable and accessible through a getter.

#### Example

| Original | Optimised |
| --- | --- |
| `class PerformanceExample : IEntity`<br>`{`<br>`protected ref array<ref AliveOrNotObject> m_aBigObjectList = { /* ... */ };`<br>`void EOnInit(IEntity owner)`<br>`{`<br>`GetGame().GetCallqueue().CallLater(PrintResult, 250, true);`<br>`}`<br>`int GetNumberOfAliveObjects()`<br>`{`<br>`int result = 0;`<br>`foreach (AliveOrNotObject object : m_aBigObjectList)`<br>`{`<br>`if (object.IsAlive())`<br>`result++;`<br>`}`<br>`return result;`<br>`}`<br>`void PrintResult()`<br>`{`<br>`Print("Alive objects = " + GetNumberOfAliveObjects());`<br>`}`<br>`}` | `class PerformanceExample : IEntity`<br>`{`<br>`protected ref array<ref AliveOrNotObject> m_aBigObjectList = { /* ... */ };`<br>`protected int m_iAliveCountBuffer;`<br>`void EOnInit(IEntity owner)`<br>`{`<br>`GetGame().GetCallqueue().CallLater(Calculate, 1000, true);	// calculate only every second`<br>`GetGame().GetCallqueue().CallLater(PrintResult, 250, true);	// display result every 1/4 second`<br>`}`<br>`void Calculate()`<br>`{`<br>`int result = 0;`<br>`foreach (AliveOrNotObject object : m_aBigObjectList)`<br>`{`<br>`if (object.IsAlive())`<br>`result++;`<br>`}`<br>`m_iAliveCountBuffer = result;`<br>`}`<br>`int GetNumberOfAliveObjects()`<br>`{`<br>`return m_iAliveCountBuffer;`<br>`}`<br>`void PrintResult()`<br>`{`<br>`Print("Alive objects = " + GetNumberOfAliveObjects());`<br>`}`<br>`}` |

## Specific Issues

### Big Foreach

#### Problem

Going through one big list of items in one operation.

#### Specific Solution

- Use a smaller list by being more precise
- Continue out of the scope if the item is not viable for the operation

### High-Frequency Scripts

#### Problem

OnEachFrame calculations - are they needed?

#### Specific Solution

- Less resource-hungry calls
- Buffering/caching

### High-Performance Request

#### Problem

Is a 10km raycast really needed?

#### Solution

- Reconsider the need
- Find a smarter solution

## Benchmark

### What Should Be Of Concern

- Non time-critical operations that are made in the same frame
- High RAM usage

### What Should Not Be Of Concern

- Normal operations that are required and that one cannot do without (yes, check if the player is alive before performing action, otherwise the feature breaks)
- One-time required big operation (spawning a base = data loading, no can do anything about it)
