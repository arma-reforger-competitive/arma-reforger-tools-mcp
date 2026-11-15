---
title: Arma Reforger:Commanding Menu Modding
url: https://community.bistudio.com/wiki/Arma_Reforger:Commanding_Menu_Modding
---

Arma Reforger 1.0.0

## Class Creation

The first step is to create a new class inheriting from `SCR_BaseGroupCommand`.

From there, it is time to override important methods to give life to the command:

### Execute

The `Execute()` method determines what happens when the command is executed from the menu; it can be AI commanding, but it can actually be anything - an emote, a hint, a Game Master lightning, etc.

**Important:** This method is broadcast to **everyone**, meaning it is executed on the server **and** on each client; be sure to have the adequate checks to retain server-only logic or target UI-equipped machines.

This behaviour may change in the future with more network targeting options.

See also [Replication](https://community.bistudio.com/wiki/Arma_Reforger:Multiplayer_Scripting#Replication).

### CanBeShown

The `CanBeShown()` method determines when is the command *shown* in the command menu; for example, a "move" order cannot be issued by a non-leader, a "get in" order cannot be issued without aiming at a vehicle, etc.

The command can be shown, yet be unavailable - see CanBePerformed below.

### CanBePerformed

The `CanBePerformed()` method determines whether or not the command can be *executed*; for example, an emote cannot be used when in a vehicle, lightning cannot be thrown if not aiming at the terrain, etc.

### Example

```enforce
[BaseContainerProps()]
class SCR_MurderinoCommand : SCR_BaseGroupCommand
{
	//------------------------------------------------------------------------------------------------
	override bool Execute(IEntity cursorTarget, IEntity target, vector targetPosition, int playerID, bool isClient)
	{
		// server-only, as killing a character is handled by other systems that handle its replication
		if (isClient)
			return false;

		ChimeraCharacter character = ChimeraCharacter.Cast(cursorTarget);
		if (!character)
			return false;

		// we get the damage manager so we can do the intended thing
		SCR_DamageManagerComponent dmgManager = character.GetDamageManager();
		if (!dmgManager)
			return false;

		// execute
		dmgManager.Kill(dmgManager.GetInstigator());
		return true;
	}

	//------------------------------------------------------------------------------------------------
	// this method is always called on player machine, no need for additional checks
	override bool CanBePerformed()
	{
		PlayerCamera camera = PlayerCamera.Cast(GetGame().GetCameraManager().CurrentCamera());
		if (!camera)
			return false;

		// get the entity at which the local player is currenty aiming
		IEntity cursorEntity = camera.GetCursorTarget();

		// only characters can be killed
		if (!ChimeraCharacter.Cast(cursorEntity))
			return false;

		SCR_CharacterControllerComponent characterControllerComp = SCR_CharacterControllerComponent.Cast(cursorEntity.FindComponent(SCR_CharacterControllerComponent));
		if (!characterControllerComp)
			return false;

		// do not allow the action on dead characters
		if (characterControllerComp.IsDead())
			return false;

		// all checks passed - the action can be performed
		return true;
	}
}
```

## Config Declaration

### Command Declaration

The `enfusion://ResourceManager/~ArmaReforger:Configs/Commanding/Commands.conf` config file is the file that declares the class as a command and holds its various properties, including its string ID, whether or not the command is leader-only, etc.

Now remains adding that command to the commanding menu.

**Note:** A command class can be used more than once, provided its `[Attribute()]` make it generic and malleable enough

### Command Menu Declaration

- The `enfusion://ResourceManager/~ArmaReforger:Configs/Commanding/CommandingMenu.conf` config file declares what is present in the actual commanding menu.
- The `enfusion://ResourceManager/~ArmaReforger:Configs/Commanding/CommandingMapMenu.conf` config file declares what is present in the *map* commanding menu.

Commands are referenced simply by their previously declared ID there along with their display text.

That's it! The new command is now added to the desired Command Menu.
