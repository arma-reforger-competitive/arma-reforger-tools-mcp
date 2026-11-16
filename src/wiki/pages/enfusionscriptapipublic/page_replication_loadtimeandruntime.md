---
title: Replication: Loadtime and Runtime
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Replication_LoadtimeAndRuntime.html
---

## What are these about

There are two main replication game states which replication distinguishes and which affect some characteristics of replicated items: loadtime state and runtime state. Items inserted into replication while game is loading (ie. at loadtime) are treated as loadtime items, while those inserted once the game is running (ie. at runtime) are treated as runtime items when inserted on server, or local items when inserted on client (as they only exist locally).

As always, for simplicity this page will use "items" to refer to both entities and components (and anything else that might be replicated).

### Loadtime items

Loadtime items are generally entity instances placed in world. These are objects that shouldn't be moved around too much (though some degree of movement is allowed) and usually need to be visible from greater distances. Typical examples are buildings or street signs. Main things to keep in mind:

- **They do not require prefab for spawning.**
- **Their insertion must be deterministic on server and clients.** Server relies on clients to have the same initial world state after the map has been loaded. It can then replicate changes from this initial state as they become relevant for a given client, reducing overall traffic (by sending only changes) and spreading the load over time. Client will still be able to see things in the distance, even though their state has not been perfectly synchronized.
  - Replication validates that initial world state matches. That is, RplId and type information of each loadtime item is the same on client as was on server initially. When a mismatch is detected, "inconsistent item table" error will appear in log and client will be disconnected with JIP_ERROR.
- **They may be out-of-sync with server for a long time** (possibly throughout whole play session). Replication scheduler running on server decides when to stream their current state to each client. When this decision is based on proximity, clients will only get current state streamed in when they get "close enough" for these changes to be relevant. Because of the world size, this may take a while or never happen at all.
  - The only exception is complete removal of these items on server. In that case, removal will be replicated to clients unconditionally.
- **As long as authority exists on server, proxy exists on client.**
  - Streaming in synchronizes state of proxy with authority. Once streamed in, proxy starts receiving state updates and it can send and receive RPCs.

**Warning:** Streaming out loadtime items while authority exists on server is currently considered undefined behavior. It should be avoided!

### Runtime items

Runtime items generally come from some game system that creates them during session. They can move around the map freely and they are usually not visible from far away. Typical examples are vehicles, player characters, collectible items. Main things to keep in mind:

- **They require prefab for spawning.**
- **They may only be inserted on server.** This is the authority.
- **Proxy may or may not exist on a client.**
  - Streaming in creates a proxy.
  - Streaming out destroys this proxy.
  - While proxy exists, it receives state updates and it can send and receive RPCs.

### Local items

Local items are items inserted on client during session. They can be used for locally predicted effects of player actions, such as firing from rocket launcher immediately creating a flying rocket on client who fired it, instead of waiting for server-side rocket to be streamed to this client. Main things to keep in mind:

- **They do not require prefab for spawning.**
- **They may only be inserted on client.** This is the authority.
- **There are no proxies on server or other clients.**

**Note:** It is common to run same code on server and clients. This can sometimes lead to unintended insertion of local items on clients. In order to prevent accidents, Arma Reforger modifies prefab spawning in a way that (by default) only allows spawning prefabs on server.

## Replication state override

Replication state override (represented by RplStateOverride enum and configured using "Rpl State Override" property) is a property of RplComponent that allows modifying the behavior of spawning and insertion process to behave as-if insertion of node hierarchy happened in specified state. Currently supported values are:

- **None**
  - State for this node and its descendants at the time of insertion is inherited from parent node (on root of node hierarchy, it is current replication game state).
- **Runtime**
  - State for this node and its descendants at the time of insertion is considered to be runtime.

In other words, as of right now it is only possible to change from loadtime to runtime state. This allows creation of complex prefabs that combine together both loadtime items and runtime items. As an example we can use prefab of a building with doors and windows (typical loadtime items) as well as food and weapons (typical runtime items) that player can pick up and carry around. To make this work, we set items meant to be carried around to use Runtime override. This will ensure that even if they were inserted while loading the map, they will be subject to rules for runtime items, rather than loadtime items. However, we have to make sure that none of the items meant to be loadtime items are descendants of those with Runtime override, otherwise the override will apply to them as well.

Notice also how state override applies to node and its descendants **at the time of insertion**. With entities and components, **child entities spawned during initialization must be attached to parent as part of spawning process**.

**Warning:** A common mistake is to spawn an entity first and then attach it as child of entity that spawned it. However, this results in spawned entity being inserted separately from parent. If this happens at loadtime and parent entity had state overridden to be runtime, child entity will not inherit state override (because it was not child at the time of insertion). It will instead be inserted as loadtime. Runtime entity spawning loadtime entity is currently considered undefined behavior because it violates requirement for deterministic insertion on server and client (runtime entities are not spawned on client, so they can't spawn loadtime children on client either) and should be avoided!

One could argue that overriding state should not be necessary, because correctly created map and systems for spawning dynamic objects like player and non-player characters, vehicles and items, will make sure that everything happens in correct game state. While that may be true, there may still be situations where these state overrides are useful:

- **Simpler creation of map for testing.**
  - One doesn't have to configure complicated loot system just to have a car for testing in the map. Well made car prefab can just be dropped in the map and messed around with as if it were spawned by some loot system.
- **Support for less dynamic map design.**
  - Games with relatively static level structure and carefully placed collectibles have no need for dynamic loot systems and all loot would be placed manually. Join-in-progress for coop mode in such games should just work when these items are correctly configured.
- **Greater resilience against mistakes.**
  - We don't live in ideal world and mistakes happen. Sometimes, prefabs may be used in the map directly instead of being left up to dynamic spawning system. While that may seem like a small issue for designers who would be performing clean sweep later in the development cycle, it can potentially lead to various undefined behaviors once players interact with these items in multiplayer, causing crashes and stability problems which are much more serious (and harder to track down). Correctly configured prefab can stay misplaced without causing any issues.
