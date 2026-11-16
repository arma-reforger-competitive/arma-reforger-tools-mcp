---
title: Arma Reforger:Resource Usage
url: https://community.bistudio.com/wiki/Arma_Reforger:Resource_Usage
---

A `Resource` object is a source of data through many classes (`BaseResourceObject` and its `BaseContainer`, `IEntitySource` etc).
Due to how resource references are managed, some safeties are required.

**Note:** See also [Arma Reforger:BaseContainer Usage](https://community.bistudio.com/wiki/Arma_Reforger:BaseContainer_Usage).

## Good Practice

Resources are managed by the engine, outside of script's reach.
If the script loses reference to a Resource object, the engine may dispose of it instantly or at some point in the future, deleting also its related `BaseContainer`/`IEntitySource`/etc objects (it is impossible to have a strong `ref` to a BaseContainer object in script).<br>
To avoid this, always keep a reference to the parent Resource.

### Examples

| Bad Example | Good Example |
| --- | --- |
| `static BaseContainer GetBaseContainer(ResourceName resourceName)`<br>`{`<br>`// the only reference to said Resource is here and will be lost at the end of the scope`<br>`Resource resource = Resource.Load(resourceName);`<br>`if (!resource.IsValid())`<br>`return null;`<br>`return resource.GetResource().ToBaseContainer();`<br>`// resource's reference is dropped here`<br>`// the returned BaseContainer may become null at any time,`<br>`// even with a script's reference`<br>`}` | `static BaseContainer GetBaseContainer(notnull Resource resource)`<br>`{`<br>`if (!resource.IsValid())`<br>`return null;`<br>`return resource.GetResource().ToBaseContainer();`<br>`// resource's reference is managed by the method caller`<br>`}` |
| `// GetBaseContainer being the above method`<br>`BaseContainer baseContainer = GetBaseContainer(m_sPrefab);`<br>`string name = baseContainer.GetClassName(); // may result in a null pointer exception` | `Resource resource = Resource.Load(m_sPrefab);`<br>`BaseContainer baseContainer = GetBaseContainer(resource);`<br>`string name = baseContainer.GetClassName(); // fine as long as a reference to resource is kept` |
| `// beware of loops!`<br>`array<BaseContainer> baseContainers = {}; // strong ref is not possible`<br>`Resource resource;`<br>`foreach (ResourceName resourceName : resourceNames)`<br>`{`<br>`resource = Resource.Load(resourceName);`<br>`if (!resource.IsValid())`<br>`continue;`<br>`baseContainers.Insert(resource.GetResource().ToBaseContainer());`<br>`// reference to resource is lost at each loop end`<br>`}`<br>`Process(baseContainers); // the array may contain nulls` | `array<ref Resource> resources = {};`<br>`array<BaseContainer> baseContainers = {};`<br>`Resource resource;`<br>`foreach (ResourceName resourceName : resourceNames)`<br>`{`<br>`resource = Resource.Load(resourceName);`<br>`if (!resource.IsValid())`<br>`continue;`<br>`resources.Insert(resource);`<br>`baseContainers.Insert(resource.GetResource().ToBaseContainer());`<br>`// reference to resource is kept in the resources array`<br>`}`<br>`Process(baseContainers);	// all references are kept`<br>`resources = null;			// this can now be done without issue` |
| n/a | `// this is fine - a Managed instance is created, resource reference can be dropped`<br>`// simplified, from {{Link/Enfusion\|armaR\|SCR_BaseContainerTools}}.CreateInstanceFromPrefab()`<br>`static Managed CreateInstanceFromPrefab(ResourceName prefab)`<br>`{`<br>`Resource resource = Resource.Load(prefab);`<br>`if (!resource.IsValid())`<br>`return null;`<br>`BaseContainer baseContainer = resource.GetResource().ToBaseContainer();`<br>`if (!baseContainer)`<br>`return null;`<br>`return BaseContainerTools.CreateInstanceFromContainer(baseContainer);`<br>`// resource reference is dropped but the created instance will remain`<br>`}` |
