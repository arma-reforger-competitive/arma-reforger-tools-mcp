---
title: Arma Reforger:Resource Manager: Data To Spreadsheet
url: https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Data_To_Spreadsheet
---

- **name**: Data To Spreadsheet plugin
- **editor**: resource
- **descr**: Import/export of data to spreadsheets
- **file**: `enfusion://ScriptEditor/Scripts/WorkbenchGame/ResourceManager/SCR_DataToSpreadsheet.c;4`

# Description

Data to spreadsheet is a **Resource Manager** plugin which allows to export selected parameters to clipboard, which then can be pasted into various spreadsheets software like Excel, Libre Calc or Google Docs.

# Usage

## Interface

### Prefab Config

Link to config file containing spreadsheet parser data. **Required field**

### Print To Console

When this option is enabled, some debug output will be printed to the Log Console.

### Comma As Delimiter

When enabled, importer & exporter will use comma instead of dot as delimiter. Depending on locale of PC, Excel might use one or another.

### Use Selection From Config

When enabled, Data To Spreadsheet parser will use Prefabs Array list defined in the config, instead of currently selected assets in Resource Browser

### Import

Initializes data import from clipboard to Workbench

### Export

Initializes data export from Workbench to clipboard

## Spreadsheet Parser

### Prefabs Array

List of **prefabs** or **configs** which should be parsed by the plugin. Only used if **Use Selection From Config** option is enabled

### Attributes Array

List of attributes which should be extracted. This array supports nested entries, where elements can also define sub-arrays, enabling you to reach a specified depth

#### List of attributes

##### SCR_DataToSpreadsheetTemplatesAttribute

Extracts attribute of given name. This entry cannot have sub-entries

##### SCR_DataToSpreadsheetTemplatesComponent

Finds Component of given name.

Example: *WeaponComponent*

##### SCR_DataToSpreadsheetTemplatesObject

Finds Object of given name

##### SCR_DataToSpreadsheetTemplatesObjectIndex

Finds Object by given index. In some cases you might encounter array where objects don't have name

##### SCR_DataToSpreadsheetTemplatesObjectArray

Finds & retrieves ObjectArray

Example: *components*

### Working with Spreadsheet Parser

New spreadsheet parser config can be created by creating new config file and selecting **SCR_DataToSpreadsheetTemplates** config. Once its created, open it in **Resource Manager** config viewer. After that, it is possible to start filling config with data

Parameters visible in Workbench are beautified in the interface so what you see in the Workbench is not how its stored or referenced by engine

Use **Copy property path** to get name of property useable by **Data To Spreadsheet** plugin

#### Example: Getting Ammo data

In case you would like to grab some basic data from ammo prefabs, first you need to decide what you want to grab. In our case, parameters like **Init Speed**, **Mass**, **Air Drag** or **Diameter** are located in **ShellMoveComponent.**

Then follow below steps

- Add new **SCR_DataToSpreadsheetTemplatesComponent** element to root **Attributes Array**
  - Type in **Component Name** field name of the component **-** in this case it is **ShellMoveComponent**
- Add new **SCR_DataToSpreadsheetTemplatesAttribute** element to **Attributes Array** inside **ShellMoveComponent**
  - In **Attribute Name** field fill in engine name of variable (*see tool tip above about **property path***)
- Repeat those steps until list is filled with all attributes that should be parsed

*In most cases default Float attribute type is good enough - even if value is int - this is due how excel might treat it afterwards.*

#### Example: Getting Ammo data from Object Array

In scenario where you would wish to extract i.e. **Damage** Value from **ProjectileDamage** object inside **Projectile Effects** of bullet prefab, some extra steps are necessary.

- Add new **SCR_DataToSpreadsheetTemplatesComponent** element to root **Attributes Array**
  - Fill in **Component Name** as before (*ShellMoveComponent in this case)*
- Add **SCR_DataToSpreadsheetTemplatesObjectArray** element to **ShellMoveComponent Attributes Array**
  - Type in **Object Name** name of array - in this case it is **ProjectileEffects**
- Add **SCR_DataToSpreadsheetTemplatesObjectIndex** to **Attributes Array** of **ProjectileEffects**
  - Find the index of element in array you want to retrive - you can do it by for instance hovering mouse over the property. In particular case index of that class is 4
- Add **SCR_DataToSpreadsheetTemplatesAttribute** for all relevant parameters that are supposed to be retrieved to  **Object Index: 4** element added in previous step

As a result, you should get something like this

#### Example: Getting Ammo data from Object Array

Getting data from child components - like on picture below -  is again bit different than other methods.

**SCR_DataToSpreadsheetTemplatesComponent doesn't support finding child components** so it is necessary to use different method in this case. Instead of it, it is possible to use **SCR_DataToSpreadsheetTemplatesObjectArray & SCR_DataToSpreadsheetTemplatesObject** classes to retrieve desired data.

- Add **SCR_DataToSpreadsheetTemplatesObjectArray** to root **Attributes Array**
  - Type in **Object Name "components"** - that's the object array containing components under the hood. You can check the structure by opening prefab in text editor of your choice
- Add **SCR_DataToSpreadsheetTemplatesObject ** to **Attributes Array** of **components**
  - Write inside **WeaponComponent** - this way you will get reference to **WeaponComponent**
- Add **SCR_DataToSpreadsheetTemplatesObjectArray** to **WeaponComponent**
  - Type in "**components"** again
- Add once again **SCR_DataToSpreadsheetTemplatesObject -** this time to **components** class under **WeaponComponent**
  - Write name of component you were looking for **- MuzzleComponent**
- Add as many **SCR_DataToSpreadsheetTemplatesAttribute** to **Attributes Array** of **MuzzleComponent** as you wish in order to extract desired parameters

*End result below*

## Exporting to Spreadsheet

## Importing from Spreadsheet
