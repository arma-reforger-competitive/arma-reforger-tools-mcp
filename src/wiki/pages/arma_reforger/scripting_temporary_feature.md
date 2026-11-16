---
title: Arma Reforger:Scripting Temporary Feature
url: https://community.bistudio.com/wiki/Arma_Reforger:Scripting_Temporary_Feature
---

Developing or updating a feature may happen over the course of multiple days and sometimes having the ability to switch it on/off for feature or performance comparison is a good thing.
Multiple options are available.

## Bool Switch

```enforce
class TAG_TestEntity : GenericEntity
{
	[Attribute(defvalue: "0")]
	protected bool m_bActivateFeature; // can be changed during runtime with SetActivateFeature

	void SetActivateFeature(bool activate)
	{
		m_bActivateFeature = activate;
	}

	void PublicMethod()
	{
		if (m_bActivateFeature)
		{
			// feature actions
		}
		else
		{
			// old/no feature actions
		}
	}
}
```

## Preprocessor If

**Important:** [Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor) *usually* covers such cases well but proper error finding in `#ifdef` blocks is not guaranteed.

Sometimes a bool switch is not a possible solution (due to required data initialisation or other systems, etc). A preprocessor definition can be used:

```enforce
class TAG_TestEntity : GenericEntity
{
	protected ref array<int> m_aObjectData;
#ifdef TAG_ACTIVATE_FEATURE
	protected ref array<int> m_aNewFeatureData; // this will not exist at all in the object if the flag is not defined, therefore no reference/memory will be attributed
#endif

	void PublicMethod()
	{
#ifdef TAG_ACTIVATE_FEATURE
		// feature actions
#else
		// old/no feature actions - m_aNewFeatureData can NOT be referenced as it does not exist here
#endif
	}

	void TAG_TestEntity()
	{
		m_aObjectData = {};
#ifdef TAG_ACTIVATE_FEATURE
		// all these operations will not happen at all - actually, all these instructions will not even exist if the flag is not defined
		m_aNewFeatureData = {};
		for (int i = 0; i < 1001; i++)
		{
			m_aNewFeatureData.Insert(i * 2);
		}
#endif
	}
}
```

This is very often used with `#ifdef WORKBENCH` across Arma Reforger code, allowing code to only be compiled for Workbench usage (saving memory space in game).

**Note:** Useful frequently used flags:
- `WORKBENCH` - if the Workbench executable is running
- `PLATFORM_CONSOLE` - if the platform is a game console (Xbox, PlayStation, etc)

### Definition

A preprocessor value can be defined in two ways:
- preprocessor definition, using `#define TAG_ACTIVATE_FEATURE` at the top of the file - required in every file that uses it. Can be commented with `//`
- the [scrDefine](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#scrDefine) startup parameter `ArmaReforgerSteam.exe -scrDefine TAG_ACTIVATE_FEATURE`
