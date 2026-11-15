---
title: Arma Reforger:Scenario Framework Update Plugin
url: https://community.bistudio.com/wiki/Arma_Reforger:Scenario_Framework_Update_Plugin
---

Arma Reforger 1.1.0

With the 1.1.0 update, [huge changes](https://community.bistudio.com/wiki/Arma_Reforger:Scenario_Framework#1.1.0_Changes) were introduced about how AIs and their Waypoints are handled by the Scenario Framework.<br>
This results in a backward compatibility impossibility for the scenarios created with a previous version.

This [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) plugin does the conversion in an automated way, provided its usage steps are respected.

## Usage

1. Back up your work before loading your world (It is good practice to regularly back up your work whenever you can so you can always go back/fix things when something breaks)
1. Load the Scenario Framework world to be updated
1. Navigate to the top bar → Plugins → Update/1.0.0 to 1.1.0 and run phase 1
1. Save the world
1. **Load the World again** (important!)
1. Navigate to the top bar → Plugins → Update/1.0.0 to 1.1.0 and run phase 2
1. Save the world again to apply all changes.

The scenario is now 1.1.0-compatible and can be worked on again.

**Important:** Running the plugin on a 1.1.0-compatible scenario is not recommended and its behaviour is undocumented (although it *should not* be an issue, no guarantees are offered).
