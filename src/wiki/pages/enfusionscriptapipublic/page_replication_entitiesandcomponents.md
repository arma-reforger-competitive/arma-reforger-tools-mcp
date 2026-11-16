---
title: Replication: Replicating entities, components and hierarchies
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Replication_EntitiesAndComponents.html
---

While it is possible to use RplNode for replication of just about anything (see RplNode for more on this), in most cases, people deal with entities and components.

RplComponent is component that provides glue between replication and system of entities and components. It does this by creating RplNode, inserting entities and components into RplNode, then registering RplNode in replication. **Only entities or components whose replication layout is not empty (contains at least one replicated property, RPC, or replication callback) will be inserted. Everything else will be ignored by replication and cannot be referenced through it (eg. using RplId).**

Base implementation of RplComponent is in gamelib::BaseRplComponent, however each game typically provides its own implementation derived from this base with game-specific changes. It is recommended to name this game-specific implementation RplComponent to avoid confusion, but you should consult game documentation to find out what the name is, as well as what (if any) differences there are compared to default behavior described in this document.

RplComponents impose some limitations on prefab structure and manipulation of entities spawned as part of hierarchy. Because of this, it is recommended that also people who don't normally deal with replication and only create prefabs and manage prefab structure and inheritance, have at least basic understanding of rules and examples described in this document.

## Creation of RplNode

Simply put, RplComponent during its EOnInit will collect replicated entities and components (those that have RPCs or replicated properties) and add them as items into its RplNode. Gathering process first considers entity to which RplComponent is attached, and its components. Then it checks whether RplComponent has Recursive property enabled. If it is not enabled, process ends. If it is enabled, it recursively processes child entities in same manner. Recursion does not enter entities which have their own RplComponent.

Following is pseudo-code version of the algorithm:

```
void GatherItems(entity, rplComponent)
{
    if (entity.IsReplicated())
        rplComponent.InsertItem(entity);
 
    foreach (component in entity.components)
    {
        if (component.IsReplicated())
            rplComponent.InsertItem(component);
    }
 
    if (rplComponent.recursive)
    {
        foreach (child in entity.children)
        {
            if (!child.HasComponent(RplComponent))
                GatherItems(child, rplComponent);
        }
    }
}
 
GatherItems(rplComponent.entity, rplComponent);
```

While above process may appear simple at first, it affects quite a lot and it is important that people working with replication intuitively understand how items are grouped into nodes, as that limits what they can and cannot do at runtime. To make all of this easier to understand, it is best show it on some examples. Let's first look at a simple entity with some components:

Here we have an entity A with 4 components: r, a, b and c. Component r is RplComponent, which will create the RplNode instance and fill it with replicated items. Components b and c are not replicated (they do not have any RPCs or replicated state) and so they are not part of created RplNode. RplNode in replication hierarchy will then contain 3 items: r, A and a. Notice how RplComponent is first item in the node, which means it is the head (see RplNode for details). Because this example only has one entity, results of simple (non-recursive) and recursive gathering look the same. Differences start to show up with entity hierarchies.

## Hierarchies

Let's first take a look at some simple entity hierarchy and how it will translate into node:

Once again, first item in the node is RplComponent r1. We then collect replicated items from the hierarchy and insert them into the node as well, while ignoring items which are not replicated. Important to notice is that entity hierarchy itself is not reflected in the node structure. Node is, after all, just a flat list of items. Pay attention to differences between results of simple and recursive gathering. In simple version, entities B and C are completely ignored, and replication knows nothing about them (thus RPCs, replicated properties and other replication features will not work). In recursive version, both of them were considered during gathering process. Notice also that in recursive version, even though entity C is not replicated, its replicated component e was still added to the node.

Hierarchies with just one RplComponent on root entity will always produce one node consisting of flattened list of replicated items. Things become more interesting when there are are multiple RplComponents present in the hierarchy:

By adding component r2 to entity B, we have created a new node r2, which is child of node r1 (previously node r). This allows us to detach entity B (node r2) from entity A (node r1) at runtime. What we can and cannot do with entity C now differs between simple and recursive version. In simple version, replication does not put any constraints on what we can do with C. However, in recursive version, entity C must always remain child of entity A, because they are part of the same node that cannot be modified after it has been inserted into replication.

One more example that demonstrates a bit more complicated hierarchy. Notice how hierarchy of entities and hierarchy of nodes can look quite different:

RplComponent sets up and maintains replication hierarchy by listening for changes in entity hierarchy and performing similar operations on node hierarchy in replication. This behavior is controlled using RplComponent property "Parent Node From Parent Entity", which is currently turned on by default. When turned off, replication node managed by this component will not have its parent replication node modified to match node of new parent entity. We can demonstrate this on above example. If we were to turn off "Parent Node from Parent Entity" on RplComponent r2, we would create two independent replication hierarchies:

This can be a powerful tool when optimizing streaming. Replication hierarchy is streamed all at once and, for larger hierarchies, this can lead to very large messages and expensive instantiation on clients. It may also lead to streaming unnecessary data to clients. Large buildings may have objects placed inside which are unlikely to become visible for particular client due to distance or obstacles, but if those objects are children of this building (in replication hierarchy), they will always be streamed together with the building itself. By turning off "Parent Node From Parent Entity" on these objects, scheduler can control them independently from building itself. As a consequence, one must keep in mind that presence of parent entity on client does not necessarily imply presence of children that turn this setting off.

## Prefabs

Remember that you must not add or remove items from node after it has been inserted into replication system, but you can modify hierarchy of nodes themselves. This means that entities with RplComponent are movable parts of the hierarchy and can be removed or added as the game progresses. When new player joins an already running game ("join-in-progress", or JIP for short), we need to communicate these changes. To keep traffic low, server sends only limited information:

1. Which entity from which prefab to spawn to recreate each node.
1. How to connect these nodes into hierarchy.

## Problematic situations

### Prefab root does not have RplComponent

One problem that can arise is prefab where root entity does not have RplComponent:

As you can see, replication system does not know anything about entity A and its components, as they are not part of the RplNode. This causes problems when spawning this prefab at runtime (or during JIP), because we can't associate RplNode with resource GUID of the prefab from which it came.

There are two possible solutions in this case:

1. Move RplComponent from entity B to entity A.
1. Add RplComponent to entity A, in addition to entity B. This is required if you need to change who is parent of entity B at runtime.

### Parent entity is not RplNode

You should be careful when intermediate entities between two entities with RplComponent do not form nodes themselves. An example case of this problem:

This might look like it should work at first, and in many cases it will. Problems will arise when this hierarchy is streamed to the client as part of JIP. Replication can only reconstruct the hierarchy of nodes, but that does not have information about entity B being child of entity A, so the hierarchy will be set up differently between client and server, leading to desync. While on its own this is not an issue for replication, it may cause problems in game code. For example, sending position relative to parent from server to client will cause client to interpret that position against different parent, giving wrong results.

More insidious case of this would arise if entity B was not replicated (no replicated state or RPCs) or if simple gathering of items was used (instead of recursive), in which case, it wouldn't even exist in the RplNode r1. It wouldn't have RplId assigned in which case user code can't correct the hierarchy either, unless it implements its own mechanism for addressing these non-replicated entities.

Possible solutions are:

1. Add RplComponent to entity B, making it a node itself and thus getting replication hierarchy closer to entity hierarchy.
1. Make entity C direct child of entity A.
