---
title: Arma Reforger:BaseDoorComponent
url: https://community.bistudio.com/wiki/Arma_Reforger:BaseDoorComponent
---

Arma Reforger 1.6.0

Starting with Arma Reforger 1.6.0, a new base class was created for doors named `BaseDoorComponent`.
Both `DoorComponent` and `SlidingDoorComponent` derive from this class.
This means that `entity.FindComponent(DoorComponent)` does not detect sliding doors anymore.

## Removed Angles from Sliding Doors

Sliding doors now derive from `BaseDoorComponent` and use distance parameters as they should.

Before 1.6.0, sliding doors derived from `DoorComponent` and had the "angle" parameters for their opening and closing distances (e.g. "Angle Range" and "Closed Angle").

**Note:** The angle parameters are kept (hidden) for the sake of backward compatibility;
however data must still be updated using the new distance parameters or log warnings will happen.

## Door Width Axis

The axis plays a very important role in collision checking for both sliding and rotating doors.
For rotating doors, rotation is always around the Y axis, so that should never be the door width axis.
If a door does not stop when it hits a character, it probably means the door width axis is incorrect, or when using the Test Contacts option, the provided collider is incorrect.

Before 1.6.0, the door width axis was not used consistently.

## Smoothing Animations

Doors now support smoothing animations. There are 4 presets, **Linear** (default, old behavior), **EaseIn** (start of the animation is smoothed), **EaseOut** (end of the animation is smoothed), and **EaseInOut** (both ends are smoothed).

**Note:** The animation is still played over the same time period, so when smoothing curves are used the door will move faster at some points during the animation.

The smoothing curve is a [5th order Smoothstep](https://en.wikipedia.org/wiki/Smoothstep#5th-order_equation) curve.
It is also possible to to provide the smoothing slope at each end of the curve using the **Custom** curve mode.

The first value (`X`) is the slope of the curve at the start of the animation, and the second value (`Y`) is the slope of the curve at the end of the animation. Slope must be between 0 and 1.
A slope of 0 means flat (very smooth) and 1 means a 45 degrees slope, so lower values mean smoother animations.
For example, X=0 and Y=1 is similar to an EaseIn curve, X=1 and Y=0 is similar to an `EaseOut` curve, and X=0 and Y=0 is similar to an `EaseInOut` curve.
