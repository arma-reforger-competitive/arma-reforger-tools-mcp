---
title: Arma Reforger:Animation Editor: State Machine
url: https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor:_State_Machine
---

This article explains the concepts of a state machine and how they can be used in animation graphs.

## State Machine

A **State Machine** is a basic functionality in any graph, and it is what decides what the animated object is currently doing, whether that is moving, idling, eating, eating while moving, and so on.
A character can be in multiple **States** at once, but generally stays in one at a time.

## State

A **State** is a child of the **State Machine**. It can be named anything and it will not conflict with the rest of the graph, as long as the name is unique inside its **State Machine**.

Each **State** has its own set of properties:

| State | Description |
| --- | --- |
| **Link** | Every **State** has a **Link** that it connects to something in a graph, such as a **Source Node, Blend Node**, or another **State Machine**.<br>A node outside of the **State Machine** cannot link directly to a **State** inside of the machine, only to the **State Machine** itself. |
| **Time** | Each **State** handles its own time inside of its **Time** property and it has three options.<br>**Notime:** The **State** does not keep any time, or use any time. Used when the **State** is linking to another **State Machine** that handles its own time.<br>**Realtime:** The **State** keeps time, and the time is incrementing as normal time.<br>This is used primarily when the **State** is linking directly to an animation that needs to loop or transition out of the **State**.<br>**Normtime:** The **State** keeps the time **Normalised**, which means that time inside of the state is going to be uniform and if passed to another node (described how to later) it is going to be the same for each node that it passes to. |
| **Start Condition** | When the parent **State Machine** of the **State** is first reached, the **Start Condition** of each **State** is evaluated, and depending on which one is true, that one will be the starting **State**. If **none is true**, the **first** node that was created in that **State Machine will be chosen**. This should be avoided when possible, as it could lead to unwanted final results. |
| **Exit** | When this is checked, any time that this node has will be passed up to the **State Machine** above, assuming there is one.<br>This is explained in depth inside of the **Transitions** article. |

## Transition

Transitions are needed to go between between **States** inside of **State Machines**.

To create a transition, right click on a **State** and click **Insert Transition** and then click on the next **State** to create the transition between them.

All transitions have these properties:

| Property | Description |
| --- | --- |
| **From** | The node that the transition will go from |
| **To** | The node that the transition will go to |
| **Condition** | On the condition that the transition will execute upon. Read the **Functions and Conditions** article for further information on what syntax it uses. |
| **Post Eval** | Tick to enable. If the transition is using time to evaluate within its condition statement or while send time through its start time, this **needs to be ticked** (**VERY IMPORTANT**).<br>Read more on this in **Functions and Conditions**.<br>This is because the transition needs to keep the data even after the tick has finished evaluating. |
| **Duration** | The blend duration of the transition.<br>**Note:** Has to be decimals, eg. 0.3, 1.0, or 0.0.<br>If 0 or 1 are typed then they become an integer, and an error will appear. |
| **Start Time** | Using this will make the node the transition is blending to start at this time. Can be used to send time from one state to another. See **Functions and Conditions**. |
| **Blend Fn** | Lin - Linear<br>SStart - Curve in then linear<br>SEnd - Linear then curve out<br>S - Blend in blend out<br>S2 - Similar to blend in and blend out, but with a dip down at the start and a dip up near the end |

## Statements

This article lists conditional statements that can be used in State Machine's start conditions or transitions.

### Operators

Used when comparing inputs.

| Operator | Name | Example |
| --- | --- | --- |
| == | Equals | (integer_1 == integer_2), if integer_1 is equal to integer_2, returns **true.** |
| != | Not Equal | (float_1 != float_2), if float_1 is not equal to float_2, returns **true.** |
| > | Greater than | (integer_1 > integer_2), if integer_1 is greater than integer_2, returns **true.** |
| < | Lesser than | (float_1 < float_2), if float_1 is lesser than float_2, returns **true.** |
| >= | Greater than or equals | (integer_1 >= integer_2), if integer_1 is greater or equal than integer_2, returns **true.** |
| <= | Lesser than or equals | (float_1 <= float_2), if float_1 is lesser or equal than float_2, returns **true.** |
| ! | Not | (!boolean), if boolean is **true**, return **false**. If boolean is **false**, return **true**. "Inverts" booleans. |
| && | And | (float_1 > float_2 && boolean), if float_1 is greater than float_2, **AND** boolean is **true**, return **true**. If **any** of part of the operation is **false**, return **false.** |
| \|\| | Or | (integer_1 <= integer_2 \|\| boolean), if integer_1 is lesser or equal than integer_2, **OR** boolean is **true**, if **any** of part of the operation is **true**, return **true**. If all parts of the operation is **false**, return **false.** |

Example:

| Name | Type | Description |
| --- | --- | --- |
| MovementSpeed | Float | 0 - Idle<br>1 - Walk<br>2 - Run<br>3 - Sprint |
| Stance | Integer | 0 - Stand<br>1 - Crouch<br>2 - Prone |
| Aiming | Boolean | True - Aiming<br>False - Not aiming |

If the character is either running or sprinting and he is standing up, return true.

**MovementSpeed >= 2 && Stance == 0**

If the character is currently not moving and not prone, return true.

**MovementSpeed == 0 && Stance != 2**

If the character is not aiming and not moving while crouched OR the character is aiming and moving:

**(!Aiming && MovementSpeed == 0 && Stance == 1) || (Aiming && MovementSpeed > 0)**

### Functions

These functions are primarily used when needing to evaluate time, or using commands to transition and/or evaluating start conditions.

| Name | Description | Note | Usage |
| --- | --- | --- | --- |
| abs(value) | Returns the absolute value |  | removes the sign of number |
| clamp(value, min, max) | Clamp the value to the range. |  | Returns min if value <= min, returns max if value >= max. Returns unmodified value otherwise. |
| cos(value) | Trigonometric function cosinus. | Argument is in radians, not degrees. |  |
| deg(value) | Convert radians to degrees. |  |  |
| float(value) | Converts integer value to float (decimal number). |  | Commonly this is implicit, but sometimes the conversion should be forced to match correct function. |
| GetCommandF(command name) op float | Returns the float value (decimal number) given by the command matching the given command name | if there's no command found the return is invalid float (-3.402823466e+38F) | Could be used when a command such as CMD_VehicleGetOut is called, and the float is the speed of the vehicle. If zero, step out, if greater than zero, jump out. |
| GetCommandI(command name) op int | Returns the integer value given by the command matching the given command name | if there's no command found the return is invalid integer (-2147483648) | If you have a command, such as CMD_Action, and you trigger the action with the integer 0, you could transition on 0, but not on 1. |
| GetCommandIa(command name) op int<br>```<br>GetCommandIb(command name) op int<br>```<br>```<br>GetCommandIc(command name) op int<br>```<br>```<br>GetCommandId(command name) op int<br>``` | Integer value share storage with four small integers. This function returns value of small integer A,B,C or D retrospectively. | if there's no command found the return is invalid integer (-2147483648) | Memory space is shared with the normal integer value. Use either entire integer storage or the smaller parts.<br>A,b,c,d parts are accessible for programmers - named the same way to prevent confusion.<br>Could be used when a command such as CMD_VehicleGetOut is called, part A contains door number (front left, front right...) and part B action type (get out slowly, quickly, jump out...). |
| GetEventTime("animName", "eventName") | returns float time of the event time in animation |  | usage -> starttime expression in transition<br>GetEventTime("ladder.enterOn", "LegUp")<br>starts new state where the event "LegUp" is in animation of entering ladder<br>This function works only in post-eval! |
| GetLowerRTime() | Returns the raw real-time value of the previous state. | If used in transition, enable **Post Eval**. Read more on **Transitions.** |  |
| GetLowerTime() | Returns the time of the previous state | If used in transition, enable **Post Eval**. Read more on **Transitions.** | Used when transitioning from one animation to another, and the time of those two animations needs to line up. An example would be when **Syncing Animations**, and you need to pass the time from one **State** to another. |
| GetRemainingTime() | Gets the time remaining of the current animation | If used in transition, enable **Post Eval**. Read more on **Transitions.** | If used like "GetRemainingTime() < 0.1", it works the same way RemainingTimeLess() does |
| GetStateTime() | Returns time spent in current state node (in seconds). Returns 0 when used outside state machines. |  |  |
| GetUpperNTime() | Gets the normalised time from the upper node. |  |  |
| GetUpperRTime() | Gets the real time from the upper node in the animation graph. |  |  |
| HasVariableChanged(variable) | Returns true if variable value has changed in comparison to last frame. |  |  |
| HasVariableChanged(variable, oldValue) | Returns true if variable value has changed and the old value was the same as the one given by argument. |  |  |
| HasVariableChangedTo(variable, newValue) | Returns true if variable value has changed and the new value is the same as the one given by argument. |  |  |
| HasVariableChanged(variable, oldValue, newValue) | Returns true if variable value has changed and both old and new values match. |  |  |
| IsAttachment("attachmentName") | Returns true if the attachment having the given name is set |  |  |
| IsCommand(command) | Returns true if command is called |  |  |
| IsEvent("eventName") | Returns true if there was sampled event |  | This function works only in post-eval! |
| IsTag("tagName") | Return true if the tag is set |  | This function works only in post-eval! |
| inRange(value, min, max) | Returns true if value >= min && value <= max |  |  |
| inRangeExclusive(value, min, max) | Returns true if value > min && value < max |  |  |
| isbitset(value, bitNumber) | Returns true if the specified bit is set in the given integer value. |  | Value must be integer. Bits are numbered from the least significant to most significant. (example: for number 10011 the bits are: [4]=1, [3]=0, [2]=0, [1]=1, [0]=1 ) |
| int(value) |  |  | Commonly this is implicit, but sometimes the conversion should be forced to match correct function. |
| LowerNTimePassed(float) | Returns true if the normalised time given by parameter has passed. | If used in transition, enable **Post Eval**. Read more on **Transitions.** | This function works only in post-eval! |
| maskint(value, bitmask) | Returns "bitwise and" (value & bitmask) |  | Bitwise and above two given values, both values must be integers. |
| max(value1, value2) | Returns the greater of two arguments. |  |  |
| min(value1, value2) | Returns the lesser of two arguments. |  |  |
| normalize(value, min, max) | Maps `value` from original range (`min-max`) to (`0`-`1`). Values outside are clamped. |  | Effectively it is the same as writing `clamp((x-from)/(to-from), 0, 1)`. |
| Random01() | Returns random value between 0 and 1. |  |  |
| rad(value) | Convert degrees to radians. |  |  |
| RandomPerc(percentage) | Returns true with a probability of percentage%. Valid range: 0-100. |  |  |
| RemainingTimeLess(float) | Returns true when the remaining animation time in child node is lesser than the float given | If used in transition, enable **Post Eval**. Read more on **Transitions.** | If used in a **Transition**, it will transition if the remaining time is less than the float given<br>This function works only in post-eval! |
| sin(value) | Trigonometric function sinus. | Argument is in radians, not degrees. |  |
| UpperNTimePassed(time) | Returns true if the normalised time coming from node above is greater than given argument. |  |  |
