---
title: Arma Reforger:Event Handlers
url: https://community.bistudio.com/wiki/Arma_Reforger:Event_Handlers
---

## Script Invokers

- An Event Handler can be created using a **ScriptInvoker (`enfusion://ScriptEditor/scripts/Game/Helpers/SCR_ScriptInvokerHelper.c`)** class.
- Any Script Invoker signature can be created using `typedef` - see [Arma Reforger:ScriptInvoker Usage#Signature Declaration](https://community.bistudio.com/wiki/Arma_Reforger:ScriptInvoker_Usage#Signature_Declaration).
- Using the `ScriptInvoker` class itself is considered bad practice and **must** be avoided.
- As Script Invokers are big objects and can be numerous, it is good practice to **not** instanciate them by default but to rather create them on request, using their getter - saving memory.

**Note:** See [Arma Reforger:ScriptInvoker Usage](https://community.bistudio.com/wiki/Arma_Reforger:ScriptInvoker_Usage) and its [Signature Declaration](https://community.bistudio.com/wiki/Arma_Reforger:ScriptInvoker_Usage#Signature_Declaration) section for tutorials on how to use Script Invokers.

## Event Handler Manager Component Events

The `EventHandlerManagerComponent` groups the following events under its category:

| Event | Class | Description |
| --- | --- | --- |
| **OnADSChanged** | `CharacterControllerComponent` |  |
| **OnConsciousnessChanged** | `CharacterControllerComponent` |  |
| **OnInspectionModeChanged** | `CharacterControllerComponent` | on player character which switches in/out of inspection mode |
| **OnMagazineCountChanged** | `InventoryStorageManagerComponent` |  |
| **OnDestroyed** | `DamageManagerComponent` | when the default hit zone is set to destroyed |
| **OnCompartmentEntering** | `BaseCompartmentManagerComponent` |  |
| **OnCompartmentEntered** | `BaseCompartmentManagerComponent` |  |
| **OnCompartmentLeft** | `BaseCompartmentManagerComponent` |  |
| **OnCompartmentLeaving** | `BaseCompartmentManagerComponent` |  |
| **OnLightStateChanged** | `BaseLightManagerComponent` |  |
| **OnTurretReload** | `TurretControllerComponent` | `isFinished` is true when reload is done and false when it started |
| **OnADSChanged** | `TurretControllerComponent` | on player character which switches to ADS<br>`TurretControllerComponent` can also trigger this event |
| **OnWeaponChanged** | `BaseWeaponManagerComponent` |  |
| **OnMuzzleChanged** | `BaseWeaponComponent` |  |
| **OnAmmoCountChanged** | `BaseMuzzleComponent` | Both weapon and character can raise this event (see `BaseWeaponManagerComponent`) |
| **OnMagazineChanged** | `BaseMuzzleComponent` |  |
| **OnProjectileShot** | `BaseMuzzleComponent` |  |
| **OnFiremodeChanged** | `BaseMuzzleComponent` |  |
| **OnWeaponAttachmentChanged** | `WeaponComponent` | `isAttached` is true if `attachmentEntity` was attached to the weapon, and false if it was detached |
| **OnZeroingChanged** | `WeaponComponent` | currently does not carry currentMuzzle |
| **OnGrenadeThrown** | `WeaponSlotComponent` |  |
| **HHMMSS** | `TimeAndWeatherManagerEntity` | Triggers based on `TimeAndWeatherManagerEntity`'s `TIME_EVENT_PERIODICITY` value:<br>- 0: never<br>- 1: every hour<br>- 2: every minute<br>- 3: every second |
| **OnDayStart** | `TimeAndWeatherManagerEntity` |  |
| **OnNightStart** | `TimeAndWeatherManagerEntity` |  |

## Entity Event Handlers

**Note:** See also [Arma Reforger:Entity Lifecycle](https://community.bistudio.com/wiki/Arma_Reforger:Entity_Lifecycle).

The following events are present and available in the **`IEntity`** class - read the code documentation to learn more about them and how to use them.

**Note:** **E** in e.g **E**OnInit stands for Event (not Entity).

| Event | Description |
| --- | --- |
| **EOnInit** | Event after the entity is allocated and initialised. |
| **EOnVisible** | This event triggers when the entity is made visible (versus being invisible). |
| **EOnFrame** | This event, as its name suggests, triggers on each simulation frame. |
| **EOnPostFrame** | Triggers after physics update. |
| **EOnFixedFrame** | **Warning:** This event can be called on another thread than the main thread. This means that you must avoid any modifications on other entities during this event! |
| **EOnFixedPostFrame** |  |
| **EOnAnimEvent** | Event from the animation system |
| **EOnPhysicsActive** | Triggers on (de)activation of the `RigidBody`'s physics. |
| **EOnPhysicsMove** | Triggers when the physics engine moves this entity. |
| **EOnSimulate** | Happens before physics engine iteration - called from sub-iterations. |
| **EOnPostSimulate** | Happens after physics engine iteration. Happens once per frame. |
| **EOnJointBreak** | Triggers when a joint attached to this entity's `RigidBody` is broken. |
| **EOnTouch** | Event when touched by another entity. It requires the entity to have `TouchComponent`. |
| **EOnContact** | Triggers when contact with another `RigidBody` has been registered. |
| **EOnDiag** | Happens every frame after EOnFrame when "Entity Diag" is enabled in the [Diag Menu](https://community.bistudio.com/wiki/Arma_Reforger:Diag_Menu) (e.g Workbench). |
| **EOnUser0** |  |
| **EOnUser1** |  |
| **EOnUser2** |  |
| **EOnUser3** |  |
| **EOnUser4** |  |
