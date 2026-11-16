---
title: Replication: RplNode
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Replication_RplNode.html
---

**Immutable** collection of replicated items.

**Warning:** Work in Progress!

## Role in replication

Nodes describe the relations of singular Items within the system. They represent a single immutable structure that shares the lifetime properties, creation and initialization process.

You can think of them as a collection of entities and their components contained in a prefab instance. Those were designed to work together and probably to be spawned and destroyed at the same time.

## Node contents

The node itself is just an array of structures. You can fill it with your items as you see fit with one exception. The first inserted item had to be the **Head** item. This special item has to implement replication lifetime specific callbacks. These are for example able to recreate the contents of this node, destroy him, move him within the hierarchy of other nodes, save and load initialization data.

## Types of nodes

There are generally three types of nodes reflecting the environment from which they were registered. Each of them will receive a different type of RplId.

1. **Loadtime**
   - The node will be present on both server and clients after the world file gets loaded.
   - *Note: These kinds of nodes don't have to implement a callbacks which are "streaming"(creating) them on clients.*
1. **Runtime**
   - Those will be always created on the fly on server. He will distribute them to clients on demand.
1. **Local**
   - Those will be created on the fly during runtime on the client.
   - They would not be streamed to server and cannot influence the experience of other clients.
   - Effectively just local effects which still share the capabilities of replicated objects (*really cool tool to de-duplicate your code*).

## Node Lifetime

### Creation

1. Allocate your node (his memory is owned by the user).
1. Insert the **Head** item.
1. Insert the rest of your items.
1. Register your node into the replication.

This is a really straight forward process but there is one caveat. You always have to be aware of when you are registering your node. In general there are only two phases influencing the registration process.

#### Loading phase

- Your nodes will be treated as **Static** by default. You have to account for that otherwise you will be greeted with a replication error telling you that you broke the loading table consistency.
- If you know that your node is getting registered in load time and you know that it won't be spawned on clients you can override the registration logic and tell the system to register it as **Dynamic** node instead.

#### Runtime phase

- Everything registered in this phase will be treated as **Dynamic** by default on server and **Local** by default on clients. Trying to override this is most probably pointless and just would not work.

### Runtime

The node will get assigned some **meta information** right after it is created. Those are related to **ownership** and **role**. You can use these right from the start to build your logic around them. All of the contained Items will get an RplId assigned after the node gets registered. This is a unique identifier synchronized across the network.

During runtime the node along with the **Head** item serves as your access point to the replication related data and its features.

When this nodes becomes relevant to a client then a procedure called **Node Streaming** will be initiated. There will be whole chapter on this topic.

### Destruction

Once you know that the items in the node are getting deleted you should unregister the node from replication first. Otherwise there would be a whole lot of nasty crashes happening. You can free the items and eventually your node. Outgoing communication from your items would be properly finalized and destruction will be replicated to your items.

## How to

The nodes are really straight forward to use on their own. The most common use case would be the BaseRplComponent which combines both, the **RplNode** with **Head** item all together. Or the **Connection** which is not part of the entity component structure. Note that your logic does not have to be written in entities and components in order to use the replication and **RplNodes** you are completely free to structure your code however you like. The only limitation is that your items have to inherit from the **BaseItem** and properly use the related macros.
