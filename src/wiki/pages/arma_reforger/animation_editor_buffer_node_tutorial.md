---
title: Arma Reforger:Animation Editor: Buffer Node Tutorial
url: https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor:_Buffer_Node_Tutorial
---

## Definitions

In order to understand the buffer nodes, you must first have a firm grasp of the animation system regarding its blending properties.

- An **Absolute Animation** is an animation which has absolute values; i.e. a **run cycle animation** or an **idle animation**.
- A **Differential Animation** is animation that compares the **difference** between itself and a target animation; i.e. a **look at animation** or a **short gesture animation**.
- An animation can be exported with different bones included; the run cycle will most likely use all of the bones, while the look at will only use the head.

**However,** animations can be exported with **both Absolute & Differential Bones.** This means that you can have an animation that includes **different parts of the skeleton** with **different settings for each bone.**

See below:

| Bone | Animation |  |
| --- | --- | --- |
| Bone | **Wave At** | **Run** |
| Head | Differential | Absolute |
| Neck | Differential | Absolute |
| Spine 2 | Absolute | Absolute |
| Spine 1 | Absolute | Absolute |
| Pelvis | Absolute | Absolute |
| L Clavicle | Absolute | Absolute |
| L Arm | Absolute | Absolute |
| L Elbow | Absolute | Absolute |
| L Hand | Absolute | Absolute |
| R Clavicle | n/a | Absolute |
| R Arm | n/a | Absolute |
| R Elbow | n/a | Absolute |
| R Hand | n/a | Absolute |

Blending bones with these parameters has varying results, as shown below:

**Animation Blending Result**

|  | Absolute | Differential |
| --- | --- | --- |
| **Absolute** | Blending two **absolute bones** will result in a **linear interpolation** between their positions and rotations. | Blending an **absolute bone** with a **differential bone** will give you a result wherein the **differential bone** is added on top of the **absolute bone**. |
| **Differential** | Blending an **absolute bone** with a **differential bone** will give you a result wherein the **differential bone** is added on top of the **absolute bone**. | Blending two **differential bones** will result in a **linear interpolation** between them, which then still needs to be added on top of an **absolute bone**. |

A combination of **Any + **Any** (with missing bones) will result in **ERROR**:<br>
Blending **two animations that are either absolute or differential** with one or both of them **having bones that the other does not** will result in an error where it tries to blend **from something to nothing**, which is not possible, and will therefore 'tick' when it starts/finished the blend.

This is where **Buffer Nodes** comes to the rescue.

## Buffer Node

First, you must understand the **animation buffer.** It is the **processed data of the animation tree** that gets used by the system to blend animations together to achieve different results. For example, by having a **Queue Node,** the system would calculate the result of the **Queue Node's children** and create a buffer for them and then blend it onto the **Queue Node Main Link's Animation Buffer** to achieve the final result.

Sometimes when you blend animations together, the animations might be using different parts of the skeleton, with details shown above.

An example that is used in DayZ is the **Look At functionality,** where the players can use their heads to look around themselves while still staying stationary. When you're running around you're only using your head, but when you're stationary the character uses its entire upper body to achieve a natural range of motion. The problem is when you're using an animation that uses the upper body, and blending it with an animation that only includes the head makes the animation 'tick'.

This is how the animations appear in the buffer when you are blending them together:

| Look_Upperbody_Anim | Blends | Look_Head_Anim |
| --- | --- | --- |
| Head | Yes | Head |
| Neck1 | Yes | Neck1 |
| Neck2 | Yes | Neck2 |
| Spine3 | No | --- |
| Spine2 | No | --- |
| Arm | No | --- |
| Elbow | No | --- |
| Hand | No | --- |

It fails because to complete this blend because it attempts to blend from/to something into nothing.
The **Buffer Node,** however, will save the state of a part of the **Animation Tree** and use it at a later time.
This way, we can fill in these 'gaps' and achieve a natural blend.
