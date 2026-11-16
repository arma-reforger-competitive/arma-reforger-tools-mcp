---
title: Arma Reforger:String Editor
url: https://community.bistudio.com/wiki/Arma_Reforger:String_Editor
---

## Open String Editor

- In Resource Manager, select Editors → String Editor. In the newly open window select File → Open and in the newly open dialog navigate to the wanted `*.st` file.
- alternatively, search for the `*.st` file in Resource Manager's Resource Browser tab and open it by double-clicking it.

## Translation Workflow

| Status | Operation | Editable | If the string is correct | If the string is incorrect |
| --- | --- | --- | --- | --- |
| **DEVELOPMENT_PENDING** | Fill **Target_en_us** as translation reference | Yes | Change the status to **DEVELOPMENT_DONE** | Fix it directly in the **Target_en_us** field |
| **DEVELOPMENT_DONE** | Feature development end, translation requirement confirmation | Yes | Change the status to **PROOFREADING_PENDING** | Set the status back to **DEVELOPMENT_PENDING** and edit the **Target_en_us** field |
| **PROOFREADING_PENDING** | Proofreaders acknowledgement | No | Change the status to **PROOFREADING_DONE** | **Unlock** the string, set the status back to **DEVELOPMENT_PENDING** and edit the **Target_en_us_edited** field |
| **PROOFREADING_DONE** | Proofreaders validation | No | Change the status to **TRANSLATION_PENDING** | **Unlock** the string, set the status back to **DEVELOPMENT_PENDING** and edit the **Target_en_us_edited** field |
| **TRANSLATION_PENDING** | Translators acknowledgement | No | Change the status to **TRANSLATION_DONE** | **Unlock** the string, set the status back to **DEVELOPMENT_PENDING** and edit the **Target_en_us_edited** field |
| **TRANSLATION_DONE** | Translators validation | No | Nothing | **Unlock** the string, set the status back to **DEVELOPMENT_PENDING** and edit the **Target_en_us** field |

**Note:** This workflow used in a professional environment to synchronise all the actors of the translation process.<br>
In the scope of a custom, one-man mod this is bound to change and be adapted.<br><br>
Its behaviour is defined in the `LocStatusPlugin` and can be disabled, making the workflow optional and customisable to one's needs.

## ID Convention

A translation id must follow a certain convention. Its name is constituted of specific parts in a specific order, and these parts are separated with a **dash** ( - ):

| Name | Description |
| --- | --- |
| Game/Product | Game or Project prefix: **AR** for **A**rma **R**eforger, **ENF** for **Enf**usion |
| Game Project | Optional, subsequential game project code (e.g a DLC, etc) |
| **Name** | PascalCased string name - e.g `MyNewWeapon`, `SplendidActionOnTruck`<br>Name prefixes/suffixes can exist through the usage of **underscores** ( **_** ):<br>- MainMenu_Options<br>- MainMenu_Options_Video<br>Iterators are part of the name section too, e.g AOSettings_**1**, AOSettings_**2**, AOSettings_**3** |
| Case | Optional, defines if the string MUST remain uppercase or lowercase.<br>Can be one of:<br>- **UC** for **U**pper**C**ase<br>- **LC** for **L**ower**C**ase |
| Platform | Used to determine a string that is only present on a certain platform, as opposed to a string that is present on PC.<br>Can be one of:<br>- **XB** for Microsoft **XB**ox<br>- **PS** for Sony **P**lay**S**tation<br>- **SWC** for Nintendo **Sw**it**c**h |

### Naming Convention

- use **meta** naming; the id must **not** be a 1:1 of the original text
  - e.g: **#AR-ExitConfirmation** and not **#AR-DoYouWantToQuit**
- do **not** use any of:
  - trademarks (use a generic name)
  - profanities (for obvious reasons)
- write in English

### Examples

- ENF-Settings
- AR-**MainMenu**
- AR-**MainMenu_Options**
- AR-OFP-**MainMenu_Options_RefreshRate**-UC-PS

## Writing Convention

A translation text must follow some rules and advices to be valid:

The in-game text should **always** refer to a stringtable and **never** be written directly in UI/data.

Do **not** hardcode:

- text in UI
- text in data (config)
- text in code
- text in image (unless it is part of the lore)
- numeric format (e.g **1000** could be written **1,000** or **1 000** in another language)
- date format (2001-12-01, 01/12/2001, 12/01/2021 etc)
- percentage values (25%, 25 %, %25 etc)

### Tips

To use arguments (formatted) text, use **%1**, **%2**, etc to include argument 1, 2, etc in the text. to display the percentage sign itself (%), use **%%**. Composed strings should be restricted to the bare minimum and used only when necessary for translation reasons.

**Localised string**

```enforce
// formatted text example unlocalised
string testName = "Peter";
PrintFormat("Hello %1, how are you?", testName);	// displays "Hello Peter, how are you?"

// #AR-Test is the translation container's id
string testName = "Peter";
PrintFormat("#AR-Test", testName);					// displays "Hello Peter, how are you?" as well
```

**Important:** If a string contains any **%1** arguments, they **must** be documented in the **Comment** field!

Do **not** assemble multiple parts to make a sentence, as such construction might be invalid in another language:

| Bad Example | Good Example | Explanation |
| --- | --- | --- |
| Hold Space to Take weapon<br>- Hold %1 to %2<br>- Space<br>- Take weapon<br>- Sleep | Hold Space to take weapon<br>- Hold %1 to take weapon<br>- Hold %1 to sleep<br>- Space | The translator does not always know that the value is to be used in a composition<br>The translation itself might not have been thought for this usage in the first place |
| Remaining time: 30 seconds<br>- Remaining time: %1 %2<br>- 30 (game value)<br>- seconds | Remaining time: 30 seconds<br>- Remaining time: %1 second(s)<br>- Remaining time: %1 minute(s)<br>- 30 (game value) |  |
| Heavy Machinegun<br>- %1 %2<br>- Heavy<br>- Machinegun | Heavy Machinegun<br>- Heavy Machinegun | Some languages place adjectives in a different order; such translation must then be done in one piece |

## Create a Stringtable File

Follow these steps to create a new `*.st` file:

1. In Resource browser select a folder where you want to create a new `*.st` file
1. Use right mouse button to open an option and here select **GUI String table**
1. Set the name of the file
1. Choose **StringTable** class and confirm
1. A new `*.st` was created - all there is to do now is to assign this `*.st` file to the project
1. In Workbench select Options
1. In newly open window select Game project
1. In the Configuration dropbox, select the desired platform (Headless, PC, Xbox, PS5, etc)
1. Navigate to Widget Manager Settings → String Tables → String Table Definition and here:
   - String Table Source - select the newly created file
   - Languages - press the "+" button to add all desired languages.

## Add a New Localised String

1. Open String Editor
1. Write an Id (stringtable entry key) into the **insert section**
1. Press **insert**, the new container is created
1. Select the new container in the **list of containers**
1. In the **Details** section, add the original text in the **Target_en_us** field.
1. Fall all other necessary details like Comment, Max Length of the text, all matching labels and so on. Details about those items are in this section of tutorial.
1. Save changes by using File → Save (or [Ctrl+S])
1. Go to Table → Build Runtime Table (or [Ctrl+B]). **Only** after the following confirmation is the text ready to be used in game.
**Recommended read:** Detailed tutorial on creating localisation in mod can be found on [**Mod Localisation**](https://community.bistudio.com/wiki/Arma_Reforger:Mod_Localisation) page.
