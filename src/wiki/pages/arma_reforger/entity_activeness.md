---
title: Arma Reforger:Entity Activeness
url: https://community.bistudio.com/wiki/Arma_Reforger:Entity_Activeness
---

## Active Flag and Frame Events

Setting a `FRAME` event on a component or an entity makes said entity (or component's parent entity) simulated.
The `ACTIVE` flag should be set on entities that are moved every frame but do not have any components using the `FRAME` event.

Doing so automatically calls an update on the entity along other engine operations such as updating the bounding box, etc.

When activating/deactivating a component, `EOnActivate`/`EOnDeactivate` component events are called, meaning that every component needs to take care of its own activation and deactivation by setting or clearing its `FRAME` flag.

**Important:** Since **0.9.8** `EOnActivate` is **not** called on components when an entity is spawned anymore, as components are now considered active by default.

### Pre-0.9.8 Behaviour

Arma Reforger 0.9.5

**Before 0.9.8**, an `ACTIVE` flag on the entity would be necessary for any `FRAME` events. This means that any component which needed the `FRAME` events would need to set `ACTIVE` on its owner.

This led to the issue where multiple components could clear the `ACTIVE` flag event of the entity where another component would still require the `FRAME` event, bringing to this change.

## Good Practices

The first step is to not set the `ACTIVE` flag on the entity by the component in order to have `FRAME` events (pre-0.9.8 way).

And then, as said above, when a component is disabled, it is its job to set/clear its event mask itself when it gets activated/deactivated.

### Examples

| Bad Example | Good Example |
| --- | --- |
| `class MyComponent : ScriptComponent`<br>`{`<br>`override void OnPostInit(IEntity owner)`<br>`{`<br>`SetEventMask(owner, EntityEvent.FRAME);`<br>`owner.SetFlags(EntityFlags.ACTIVE);`<br>`}`<br>`}` | `class MyComponent : ScriptComponent`<br>`{`<br>`override void OnPostInit(IEntity owner)`<br>`{`<br>`SetEventMask(owner, EntityEvent.FRAME);`<br>`}`<br>`override void EOnActivate(IEntity owner)`<br>`{`<br>`super.EOnActivate(owner);`<br>`SetEventMask(owner, EntityEvent.FRAME);`<br>`}`<br>`override void EOnDeactivate(IEntity owner)`<br>`{`<br>`super.EOnDeactivate(owner);`<br>`ClearEventMask(owner, EntityEvent.FRAME);`<br>`}`<br>`}`<br>Advanced example:<br>`class MyComponent : ScriptComponent`<br>`{`<br>`protected bool m_bCustomCondition = false;`<br>`void SetCustomCondition()`<br>`{`<br>`m_bCustomCondition = true;`<br>`SetEventMask(owner, EntityEvent.FRAME);`<br>`}`<br>`override void EOnActivate(IEntity owner)`<br>`{`<br>`super.EOnActivate(owner);`<br>`if (!m_bCustomCondition)`<br>`return;`<br>`SetEventMask(owner, EntityEvent.FRAME);`<br>`}`<br>`override void EOnDeactivate(IEntity owner)`<br>`{`<br>`super.EOnDeactivate(owner);`<br>`if (!m_bCustomCondition)`<br>`return;`<br>`ClearEventMask(owner, EntityEvent.FRAME);`<br>`}`<br>`}` |
