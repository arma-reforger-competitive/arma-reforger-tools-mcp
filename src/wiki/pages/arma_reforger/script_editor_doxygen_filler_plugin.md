---
title: Arma Reforger:Script Editor: Doxygen Filler Plugin
url: https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Doxygen_Filler_Plugin
---

- **name**: Doxygen Filler
- **editor**: script
- **descr**: Create/Format Doxygen documentation skeleton for configured methods (default public methods only)
- **file**: `enfusion://ScriptEditor/scripts/WorkbenchGame/ScriptEditor/SCR_DoxygenFillerPlugin.c`
- **shortcut**: [Ctrl+Alt+Shift+D]

**Doxygen Filler** is a plugin that automatically adds a [Doxygen](https://community.bistudio.com/wiki/Doxygen) skeleton structure to document methods properly, e.g:

```enforce
//!
//! \param[in] entity
//! \param[out] health
//! \return
bool IsSoldierAlive(notnull IEntity entity, out float health = 0)
{
	// ...
}
```

## Parameters

- **Doxygen public/protected/private methods**: tick to add a Doxygen skeleton to public/protected/private methods respectively
**Note:** It is considered good practice to at least document the public API.

- **Doxygen overridden/static/obsolete methods**:
  - documenting overridden methods is not always wanted, unless the override drastically changes the method's behaviour
  - documenting static methods is also a good practice, see public/protected/private methods above
  - documenting obsolete methods may not be worth it, depending on the context
- **Partial Doxygen Prefixes**: method prefixes for which no description field is added (e.g `GetHealth()`, self-explanatory enough); if emptied, refilled with `Get`, `Set`, `On`, `Is`
- **Convert Doxygen Formatting**: converts comment blocks `/*! */` into inline comments `//!`, preferred <table style="min-width: 75em"><tr><td style="width: 50%">
```enforce
/*!
This is a comment
Made in one block
Try not to use this format too much,
As it sometimes conflicts with other comments
*/
```
</td><td>
```enforce
//!
//! This is a comment
//! Made in one block
//! Try not to use this format too much,
//! As it sometimes conflicts with other comments
//!
```
</td></tr></table>Note that top/bottom lines can be removed here.
- **Add Missing Separators**: add the infamous method separator on methods not having it<br>`//------------------------------------------------------------------------------------------------`
- **Add Constructor Normal Comment**: add a simple `// constructor` comment, helpful in case of class renaming and forgetting to rename the constructor
- **Add Destructor Normal Comment**: same with `// destructor`, so the constructor would not feel singled out
