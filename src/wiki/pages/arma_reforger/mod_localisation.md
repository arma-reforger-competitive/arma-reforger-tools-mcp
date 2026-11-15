---
title: Arma Reforger:Mod Localisation
url: https://community.bistudio.com/wiki/Arma_Reforger:Mod_Localisation
---

This page describes the process of adding localisation to addons. It is recommended to go through [String Editor documentation](https://community.bistudio.com/wiki/Arma_Reforger:String_Editor) first since most of the procedures are shared.

## Creating new localisation table

Creating of new string table (.st) file is described on String Editor - New *.st file page and below is quick rehearsal of those instructions

### Creating new localisation files

**Note:** This is the **recommended method**.

New localisation files - string table and runtime string tables - can be simply created via **File → New** option (alternatively, Ctrl+N shortcut can be used).
Once activated, a new pop up window will appear and ask you to provide file name and location of new localisation files.
After it is confirmed with **OK** button, **String Editor** will create new **.st file** and **runtime configs for all languages** available in-game (see the Localisation targets table for a list of languages).

If you do not plan to translate your addon into all languages, you can **safely remove unnecessary config files** and recreate them at later point manually.

In any case, at this stage it is possible to move forward and register both **string table** and **runtime stringtable configs** in Workbench options. To do so, skip to Register Localisation Table segment

### Manual method

**Obsolete:** this method is error-prone and no longer recommended.

#### Creating new string table

|  | 1. In **Resource Browser** select a folder where you want to create a new ***.st** file - In this example it is **SampleMod_NewCar/Language folder.**<br>Next, either click with *Right Mouse Button* on **Resource Manager** field **(1a)** or use **Create** button **(1b)**<br>In both cases, new context menu should appear - pick "**GUI string table**" **(2)** from that list. |
| --- | --- |
|  | 2. Set the name of the file. It is highly recommended to use some [unique tag](https://community.bistudio.com/wiki/Scripting_Tags) in front of the file name (f.e. *samplenewcar_localization.st*) to avoid GUID clashes. |
|  | 3. Choose **StringTable** class and confirm by clicking once on it **StringTable** entry with *Left Mouse Button* |

#### Setting basic parameters

If those steps above were performed successfully, a new **localization_MyAddon.st** should be now visible in **Resource Manager** asset browser.

Click on newly created localization.st file with *Right Mouse Button* and select **Open File in New Tab (1)** option from context menu.
This should open that string table in new tab, where some of the internal bits of string table class can be changed.

By default, newly created string tables are using platform agnostic settings and both **Target Prefix** & **Item Class Name** parameter have to be changed in order to be used in **String Editor**.
Following values must be used

- *Target Prefix*: **Target_**
- *Item Class Name*: **CustomStringTableItem**

After that, **localization.st** file should be ready to be used by **String Editor**.
A simple test can be performed by either double clicking with *Left Mouse Button* on localization.st resource in **Resource Browser** (this will open that file in String Editor) or by manually opening **String Editor** (*Editors → String Editor*) and then using File → Open option.

#### Adding runtime string tables

Runtime string tables are files which are used by the game itself and they are created in very similar way to regular string tables.

Main difference is fact that instead of selecting **GUI string table** context menu, you should pick **Config file (2)** option instead of *GUI string table*.

In next dialog, fill name of that new run time string table (i.e. *samplenewcar_localization.en_us.conf*).
Once that step is completed, a new window will appear, asking you to **choose a class** of new config - this time **StringTableRuntime** should be used.

Depending on how many languages are supposed to be translated, repeat above steps till all translation variants are created.

### Localisation targets table

Below is small table showing various localisation targets shortcuts

| **en_us** | English (United States)<br>Serves as a source for translated texts.<br>If proofreading or translation is in progress, use 'Target_en_us_edited' instead. |
| --- | --- |
| **en_us_edited** | English (United States)<br>Use this when the string is being processed by proofreaders or translators.<br>Once it is done, the Localisation Manager will move this text to 'Target_en_us' |
| **fr_fr** | French (France) |
| **it_it** | Italian (Italy) |
| **de_de** | German (Germany) |
| **es_es** | Spanish (Spain) |
| **cs_cz** | Czech (Czech Republic) |
| **pl_pl** | Polish (Poland) |
| **ru_ru** | Russian (Russia) |
| **ja_jp** | Japanese (Japan) |
| **ko_kr** | Korean (South Korea) |
| **pt_br** | Portuguese (Brazil) |
| **zh_cn** | Chinese (China) |

## Registering localisation table

Next step is registering of localisation table in game project file. To do so, navigate to **Workbench → Options**

1. Make sure that you are editing correct game project by checking which addon is shown in **drop down menu (1)**. In this case it is **SampleMod_NewCar**
1. Navigate to **Widget Manager Settings (2)**
1. Locate **String Tables (3)** entry and expand it by pressing on little blue arrow on the left side
1. Add new element to **String Tables** array by clicking on small plus sign **(4)**. This will be a new entry holding information about string table file created previously
1. Locate previously created **string table (.st) file** by clicking on three small dots **(5)** and then selecting that file
1. Add new element to **Languages** array by clicking on dot on the right side **(6)**
1. Type **code of new language** (see table above for available codes) to field marked with number **7** on the picture
1. Select **runtime stringtable file (8)** appropriate to the language code which was created in previous step. In this case code is en_us, so String Table Runtime is *samplenewcar_localization.en_us.conf*

If all steps were performed correctly, new string table should be now ready to be used!

**Important:** Restart of the workbench might be necessary to initialise new string tables properly.

## Localising assets

In this chapter, name of Sample New Car visible in in-game Editor will be localised.

### Localising asset via plugin

Alternative method involves using **Resource Manager** plugin called "**Localize Selected Assets**".

This plugin automates most of the process by searching for strings defined by a config file.
Localisation parser configs (selected through **Config Path** parameter in **Localize Selected** plugin) contain various look up rules, default string table where to store changes & prefix settings.
While having string table and prefix defined in config might be quite useful when working on projects involving multiple people, it is also possible to override **String Table Path & Prefix** parameters, so it is also possible to use existing Arma Reforger configs in smaller addons.

Vehicles in-game Editor display name is defined in **SCR_EditableVehicleComponent -** there, in the **Visualization** section, a **Name** parameter can be found.
In this case, **Name** parameter was set to *Sample New Car (Black)* and goal is to have that string present in runtime string table.

To do so, select in Resource Browser assets to localise and then navigate to **Plugins → Localize Selected Files** in top navigation bar or use Ctrl+Alt+L shortcut.

**Localize Selected File** plugin window should now appear. Following changes to default parameter values need to be performed:

- Changing **Config Path** param to **Editor.conf** - this config will ensure that all editor related strings (like this vehicle display name) will be properly localised
- Changing **String Table Override -** by default plugin is storing all changes to the string table defined in config file itself. Since duplicating this config file might be an overkill for localising 2 assets, it is possible to change string table which is used by selecting another string table in plugin itself. In this case **localization.st** in **SampleMod_NewCar** is selected
- Changing **Prefix Override** - similar to string table, string prefix is stored in config file. In this case, prefix is changed to **SampleMod-** to ensure that those new string tables will not clash with vanilla data.

Once parameters are adjusted, it is possible to localise those files - to do so, press the **Localize** button.

After that, plugin will open **String Editor** (if it is not open already), **open string table** defined in **String Table Override** property, **perform scan** for any editor related entries & then finally **store it in string table**.

### Localising asset manually

It is also possible to manually change **Name** property in **Editable Component** and then add that string to string table. To do so, it is necessary to replace **Name** property in **Editable Component** with variant containing # prefix - i.e. *#SampleMod-EditableEntity_SampleCar_01_Base_Name*.

Next, it possible to add that string to **String Table** by following instructions on [String Editor page](https://community.bistudio.com/wiki/Arma_Reforger:String_Editor#Add_a_New_Localised_String). In principle, following steps have to be done:
- Fill in the new string name (**without a hastag in front!**) into StringID field
- Click on **Insert** button **(2)**. After that, new string should be added to **list of available strings (3)**
- Write English version of the string in **Target En Us** field located in **Default (4)** section
- Add localisation for rest of languages - if field is empty, English is used.

Parameters in **Custom (5)** section are usually filled by the plugin and those serve as hint for localisation team, so they are aware of string/sentence context. That section is non mandatory.

### Regenerating runtime tables

If all strings were properly added to string table then it is time to **generate runtime tables**. Without **runtime tables**, game will be not able to see strings located in string table file. Only after string table is parsed to optimized, game ready format, strings will be ready to use. To do so, head to String Editor and select from top bar **Table → Build Runtime Table** (*also accessible under Ctrl+B shortcut*). That's it - process should take just few seconds and runtime string table should be ready to go.

**Important:** Don't forget to regenerate runtime tables every time you are done with tweaking string table file!

## Localising scripts

By default, any engine will try to localise any string (*LocalizedString* ) in **UI** if it is starting with **hash (#) sign**. There is no equivalent of the previous [SQF](https://community.bistudio.com/wiki/SQF_Syntax) [localize](https://community.bistudio.com/wiki/Localize) command.

**Note:** Please use **LocaleEditBox** when exposing variables in script

```enforce
[Attribute(uiwidget: UIWidgets.LocaleEditBox, defvalue: "#AR-Editor_Hint_Intro_Title")]
protected LocalizedString m_sIntroHintTitle;
```

## Testing result in-game

String can be verified in game by launching i.e. in game editor and checking if strings are showing proper data instead of hashtag prefixed strings.

It is also possible to verify if strings are localised by selecting from **[Diag Menu](https://community.bistudio.com/wiki/Arma_Reforger:Diag_Menu) → UI → Disable → Disable Loc** option. This debug will disable automatic localisation of strings and will show names in their original, with hash tag prefix, form.

**Note:** See [Diag Menu](https://community.bistudio.com/wiki/Arma_Reforger:Diag_Menu) for more information.
