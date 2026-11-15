---
title: Replication
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Replication.html
---

| Pages |
| --- |
| RplNode |
| Replicating entities, components and hierarchies |
| Loadtime and Runtime |

## Introduction

Gameplay networking in the Enfusion engine is based client-server architecture model. In this model, server is authoritative over game simulation and acts as the source of truth. Each client only communicates with the server, never with other clients, and server decides what each client knows about and what it can do at any given moment.

Replication is a system for facilitating and simplifying implementation of game world and simulation shared by multiple players over network. It introduces high level concepts and mechanisms for handling issues inherent in multiplayer games, such as:

- unreliability of network communication, where packets may be lost or arrive out of order
- limited amount of bandwidth available for network communication
- malicious players using cheats to gain unfair advantage over others

It is important to keep in mind that while replication provides tools, it is up to users (programmers and scripters) to make sure they are used appropriately. Understanding and accounting for the impact of replication on architecture of game objects (entities and components) as well as gameplay systems early in the project development is crucial. Some design decisions may be hard or impossible to change later in the development (often due to time constraints and many other things being built on top of them) and so it is important that they are efficiently networked and tested from the very beginning.

## The Big Picture

Let us first introduce the terminology and how all things work together, before going into specifics. This section will only give rough description of how things work in the most common situations and leave important details and specifics for later.

Replication runs in one of two modes: **server** or **client**. Server hosts replication session and clients connect to this session. There is always exactly one server and there can be any number of players (up to max player limit). Server is the only one who knows about everything and if it disappears, session ends and clients are disconnected. It is not possible for a client to take over when server has disappeared.

Server can be hosted in one of two modes: **listen server** and **dedicated server**. Listen server is server hosted by one of the players. It accepts inputs (keyboard, mouse, controller, etc.) from hosting player directly and provides audio-visual output to hosting player. Dedicated server does not have a way to accept inputs or provide audio-visual output and it only allows players to connect remotely.

The smallest unit that replication works with, is an **item**. Only items created on server will be shared with clients. Item created on a client will only exist on this client and nobody else can see it. An item can by many things, but the most common ones are entities and components. Not all entities and components are items, though. For an entity or component to be registered in replication as an item, it must have at least one of the following:

- replicated properties
- functions callable using remote procedure calls (RPCs)
- replication callbacks

Multiple items are grouped together into a **node**, which is then registered in replication. Once a node has been registered, list of items in it cannot be modified (items cannot be added, removed or destroyed) until the node is removed from replication. The most common node for entities and components is RplComponent.

Each node can have a parent node and multiple child nodes, forming a **node hierarchy**. Node hierarchies can be changed dynamically, after nodes were registered in replication. This can be used for dynamically modified entity hierarchies, such as player character entering into a car and driving around in it.

Node hierarchies on clients may not always be present or synced up with server since a world was loaded, and process managing their presence is called **streaming**. Process of node hierarchy being created or synced up on client is called **streaming in**. The opposite process, when node hierarchy is being removed from client, is called **streaming out**.

Streaming on server is primarily governed by replication **scheduler**. Scheduler determines whether a node hierarchy is relevant for a client and orders it to be streamed in or out based on that. It also determines priority and frequency of replicating properties of an item, to make sure that available bandwidth is used where it matters the most. All of this depends heavily on the type of game, so scheduler is usually fine-tuned for every game to some extent. Most of the relevancy and prioritization is based on distance of an object in the game world from the player, so that closer objects are updated more often, while distant objects are updated less often or streamed out completely. However, there are also more abstract concepts in games (such faction-specific data), which require different rules for determining relevancy. Scheduler provides means for game to implement these custom rules as necessary.

### Architecture implications

Because node hierarchy is the unit of streaming (in general, whole node hierarchy is either streamed in or out on a particular client), it is very important to take it into consideration when thinking about architecture of various systems. Specifically, we want to make sure that client gets some information only when it is really necessary.

Let's say a player is part of a team that is spread out over the game world. We would like to implement a map UI that allows this player to see where in the world all of their teammates are right now. Positions of all team members are updated in real-time as they move around. This map is only visible when player opens it, it is not a minimap that is always present in the corner of the HUD.

An obvious approach would be to just use positions of characters of each team member and draw markers on the map corresponding to those positions. This may appear to work correctly at first, but as soon as one of the team members goes far enough and their character streams out, their marker on the map will also disappear.

To address that, one could set things up in scheduler so that team members are always relevant for each other. This way, no matter how far away they move from each other, they will always know where rest of their team members are. This addresses the problem with missing markers on the map and, for some games, this may be good enough. However, there are several problems with this approach:

- Single character is far more than just its position. By making all of it relevant, we replicate also changes to its equipment, animations, and anything else that is necessary for a character in 3D scene. However, for distant characters we only need their 2D position on the map.
- We only need this information when the map is open.

Ideal solution decouples streaming of data needed for map display from data needed for animated character in 3D world. Because node hierarchy is streamed as a whole, this requirement forces us to replicate these things using independent node hierarchies. Character data will continue to have relevancy based on location in the world, so that their impact is reduced as they get further away. For map indicators, when player opens the map, client makes a request to the server, showing interest in data for map indicators. Server then makes map indicators relevant for this particular client, which begins replicating their state and changes. Server has to update these map indicators to reflect changes in positions of actual characters, but if none of the players have map open, these markers do not produce any network traffic and their CPU cost is minimal. Obviously, there are more improvements possible, such as quantizing marker positions, updating only when they are relevant for some client, etc. But the most important optimization, is separation of data that is only loosely related.

There are two ways to implement map markers in this case:

1. Implement each marker as a separate node.
1. Group multiple markers in a single node.

Which of these two approaches is better depends on a particular problem being solved. In general, first approach is recommended. Second one may be easier to implement at first, but it may also be harder to optimize or adjust to new requirements when design changes.

## Fundamentals

Now that we have described how things work overall, let us take a look at a few examples of using replication in code. We will start by looking at a simple animation that does not use replication at all. Next we will modify this animation to use replication. Finally, we will implement a more complicated system that allows multiple players to interact with it.

**Note:** These examples, as well as the rest of documentation, assume reader is already familiar with entities, components and world editor. Specifically, one should already know how to:
- create an entity with components of specific types
- set a property of an entity or component to particular value
- create an entity prefab
- organize entities in a hierarchy both in prefab and in code
- modify an existing world or create new one for testing

To make things easier to visualize, all examples will use following entity to draw a debug sphere in the world, with sphere color selected using an index:

```
class RplExampleDebugShapeClass: GenericEntityClass {}
RplExampleDebugShapeClass g_RplExampleDebugShapeClassInst;
 
class RplExampleDebugShape : GenericEntity
{
    static const int COLOR_COUNT = 4;
    static const int COLORS[] = {
        Color.BLACK,
        Color.RED,
        Color.GREEN,
        Color.BLUE,
    };
 
    private int m_Color;
 
    void RplExampleDebugShape(IEntitySource src, IEntity parent)
    {
        this.SetEventMask(EntityEvent.FRAME);
    }
 
    override void EOnFrame(IEntity owner, float timeSlice)
    {
        vector worldTransform[4];
        this.GetWorldTransform(worldTransform);
        Shape.CreateSphere(m_Color, ShapeFlags.ONCE, worldTransform[3], 0.5);
    }
 
    bool SetColorByIdx(int colorIdx)
    {
        if (colorIdx < 0 || colorIdx >= COLOR_COUNT)
            return false;
 
        m_Color = COLORS[colorIdx];
        return true;
    }
}
```

**Note:** All classes in these examples (including entities and components) are not available by default. To try these examples yourself, you need to add them to some script file (new or existing one) and "Compile and Reload Scripts" in Script Editor, so that entities and components from the code become available.

### Example animation without replication

Our first example will be simple animation that changes color of our shape over time. We will switch to next color every couple of seconds and, after reaching last color, we will wrap around and continue from the first one. All of the example code is implemented in single component:

```
class RplExample1ComponentColorAnimClass : ScriptComponentClass { }
RplExample1ComponentColorAnimClass g_RplExample1ComponentColorAnimClass;
 
class RplExample1ComponentColorAnim : ScriptComponent
{
    // Constant specifying how often (in seconds) to change the color index. For
    // example, setting this to 5 will change the color index every 5 seconds.
    private static const float COLOR_CHANGE_PERIOD_S = 5.0;
 
    // Helper variable for accumulating time (in seconds) every frame and to calculate
    // color index changes.
    private float m_TimeAccumulator_s;
 
    // Color index currently used for drawing the sphere.
    private int m_ColorIdx;
 
    override void OnPostInit(IEntity owner)
    {
        // We check whether this component is attached to entity of correct type and
        // report a problem if not. Once this test passes during initialization, we
        // do not need to worry about owner entity being wrong type anymore.
        auto shapeEnt = RplExampleDebugShape.Cast(owner);
        if (!shapeEnt)
        {
            Print("This example requires that the entity is of type `RplExampleDebugShape`.", LogLevel.WARNING);
            return;
        }
 
        // We initialize shape entity to correct color.
        shapeEnt.SetColorByIdx(m_ColorIdx);
 
        // We subscribe to "frame" events, so that we can run our logic in `EOnFrame`
        // event handler.
        SetEventMask(owner, EntityEvent.FRAME);
    }
 
    override void EOnFrame(IEntity owner, float timeSlice)
    {
        // We calculate change of color index based on time (and configured color
        // change period), then apply the change in color.
        int colorIdxDelta = CalculateColorIdxDelta(timeSlice);
        ApplyColorIdxDelta(owner, colorIdxDelta);
    }
 
    private int CalculateColorIdxDelta(float timeSlice)
    {
        // We first accumulate time and then calculate how many color change periods
        // have occurred, giving us number of colors we've cycled through.
        m_TimeAccumulator_s += timeSlice;
        int colorIdxDelta = m_TimeAccumulator_s / COLOR_CHANGE_PERIOD_S;
 
        // We remove full periods from the accumulator, only carrying over how much
        // time from current period has elapsed.
        m_TimeAccumulator_s -= colorIdxDelta * COLOR_CHANGE_PERIOD_S;
 
        return colorIdxDelta;
    }
 
    private void ApplyColorIdxDelta(IEntity owner, int colorIdxDelta)
    {
        // If there is no change to color index, we do nothing.
        if (colorIdxDelta == 0)
            return;
 
        // We calculate new color index.
        int newColorIdx = (m_ColorIdx + colorIdxDelta) % RplExampleDebugShape.COLOR_COUNT;
 
        // We check also new color index, since shorter periods and lower frame-rate
        // may result in new and old color index values being the same.
        if (newColorIdx == m_ColorIdx)
            return;
 
        // Now we can update the color index ...
        m_ColorIdx = newColorIdx;
 
        // ... and set new color based on new color index value.
        RplExampleDebugShape.Cast(owner).SetColorByIdx(m_ColorIdx);
    }
}
```

To see it in action, we need to place RplExampleDebugShape entity in a world and attach RplExample1ComponentColorAnim component to it. After switching to play mode, you should see a sphere at the position where you placed your entity. This sphere will change color every 5 seconds, cycling through black, red, green and blue, before starting from the first color again. If for some reason you do not see this, then you should determine what is the problem before moving on.

If you were to try this in a multiplayer session (using Peer tool plugin), you will notice that the color of our sphere changes at different time for client and server. Furthermore, depending on multiple factors, color of the sphere is also different between client and server. Why is that? Let's break down what is happening in more detail.

The moment our entity is created marks the beginning of our color animation, which then advances every frame based on elapsed time. For our animation to be the same on both client and server, we need to ensure that they both create the entity at the same time, so that starting point of the animation matches. Unfortunately, this is almost never the case, so we usually see the two offset from each other.

There are multiple ways this can be fixed and we will look at one in Example animation with replication. But before we do, there is one important question worth thinking about: Does it matter? In this case, we can clearly see that client and server see different color of the sphere, which is all this example does, so the answer may obviously be "yes, it matters". But what if this was a component that creates 2 seconds-long flickering of a neon sign on random building somewhere in the background? Would it matter if one player saw it lit up for a moment while another did not? Probably not. **Whenever we can get away with something being simulated only locally, we should take advantage of it.** Networking complex systems is hard and prone to bugs, and network traffic is the most limited resource we have.

### Example animation with replication

When developing multiplayer game, it is good to differentiate between **simulation** and **presentation**. Main purpose of simulation is to simulate the game logic and things going on "under the hood": calculating damage, keeping track of character hit-points, AI making decisions, physics simulation, evaluation of victory conditions, and so on. Presentation then produces audio-visual output that players can observe. When playing game offline in single-player mode, both simulation and presentation happen together. Same is true for player hosting a listen server. A dedicated server runs only simulation, as there is no way to observe audio-visual output of the presentation. A client connected to remote server (whether listen server or dedicated server) only presents results of the simulation that is happening remotely, but does not actually simulate anything. As a rule of thumb, in a multiplayer session exactly one machine runs simulation, but all players run presentation. Primary purpose of replication is to replicate data from machine that runs simulation to all players doing presentation.

**Note:** There are advanced techniques that hide network latency by trying to guess what the simulation will do using limited knowledge on the client. This is usually referred to as **prediction** because the actual simulation will happen in the future, when actions from players reach the server. For simplicity, we will ignore prediction for now.

To replicate our animation, we will need to do a few things:

1. We need to register our entity in replication.
1. We need to decide who is doing the simulation.
1. We need to identify parts that belong to simulation, presentation, and data that needs to be replicated from simulation to presentation.

First change we have to make is to add an RplComponent to our entity. This will register the entity and its components for replication. In short, when there is an RplComponent on an entity, that entity along with its descendants in entity hierarchy will be scanned during initialization, collecting all replicated items (entities and components which are relevant for replication) and registering them.

**Note:** We have simplified RplComponent functionality a lot here. More in-depth description will appear later in RplNode and Replicating entities, components and hierarchies sections.

Next, we need to decide who is doing the simulation. As we have seen in previous example, when one or more clients join a server, an instance of our entity will be created on each of them and they will all start playing the animation independently. To make them see the same animation, we need to make one of them be the source of truth, and everyone else must follow that. When we register an item for replication, it is assigned one of two roles: authority or proxy. Exactly one instance across all machines in multiplayer session is authority and everyone else is proxy. That is exactly what we need: authority is the source of truth, and proxies follow it.

Finally, we need to decide what is simulation, what is presentation, and what to replicate. A good rule of thumb is to focus on presentation. What is the bare minimum needed to produce the audio-visual result we need? We are looking for something that is both small in size and doesn't change very often. Since our animation is just about changing color every couple of seconds, we could replicate the color value. However, color value is encoded as 32-bit RGBA, and every bit counts when it comes to network traffic. We know there is only limited number of colors we cycle through in our animation, so using color index might be even better, as it can be encoded in fewer bits. In our case, there are 4 possible colors, and we can encode their indices in just 2 bits. To keep this example short, we will not go that far and just stick to default. Still, advantage of using color index is that it is already available, while color value would have to be taken from RplExampleDebugShape. Having settled down on color index as our replicated data, it is now obvious how to divide things between simulation and presentation:

- simulation - advances time, keeps track of color change period, and calculates new color index
- replicated data - color index
- presentation - changing color of our debug shape using color index and rendering of debug shape

After we have added RplComponent to our entity, we can start making changes in code.

We start by saying that we want to replicate color index value from authority to proxies. We do this by decorating color index with RplProp attribute. This attribute also let's us specify name of function that should be invoked on proxy whenever value of the variable is updated by replication.

```
    // We mark color index as replicated property using RplProp attribute, making
    // it part of replicated state. We also say we want OnColorIdxChanged function
    // to be invoked whenever replication updates value of color index.
    [RplProp(onRplName: "OnColorIdxChanged")]
    private int m_ColorIdx;
```

Next we change the initialization. Since our simulation happens in frame event handler EOnFrame, we only need to receive it on authority. Proxies will be reacting to changes of color index variable. If value of that variable does not change, proxies are passive and do not consume any CPU time, which is always nice.

```
    override void OnPostInit(IEntity owner)
    {
        auto shapeEnt = RplExampleDebugShape.Cast(owner);
        if (!shapeEnt)
        {
            Print("This example requires that the entity is of type `RplExampleDebugShape`.", LogLevel.WARNING);
            return;
        }
 
        shapeEnt.SetColorByIdx(m_ColorIdx);
 
        // We must belong to some RplComponent in order for replication to work.
        // We search for it and warn user when we can't find it.
        auto rplComponent = BaseRplComponent.Cast(shapeEnt.FindComponent(BaseRplComponent));
        if (!rplComponent)
        {
            Print("This example requires that the entity has an RplComponent.", LogLevel.WARNING);
            return;
        }
 
        // We only perform simulation on the authority instance, while all proxy
        // instances just show result of the simulation. Therefore, we only have to
        // subscribe to "frame" events on authority, leaving proxy instances as
        // passive components that do something only when necessary.
        if (rplComponent.Role() == RplRole.Authority)
        {
            SetEventMask(owner, EntityEvent.FRAME);
        }
    }
```

We use RplComponent to determine our role in replication. We also warn the user when RplComponent is missing on our entity as we currently require it to work correctly.

**Note:** To keep these examples game-independent, we use BaseRplComponent, which is base class of game-specific RplComponent.

Finally, we need to modify our code for updating color index, so it changes color on both authority and proxies.

```
    private void ApplyColorIdxDelta(IEntity owner, int colorIdxDelta)
    {
        if (colorIdxDelta == 0)
            return;
 
        int newColorIdx = (m_ColorIdx + colorIdxDelta) % RplExampleDebugShape.COLOR_COUNT;
        if (newColorIdx == m_ColorIdx)
            return;
 
        // Update replicated state with results from the simulation.
        m_ColorIdx = newColorIdx;
 
        // After we have written new value of color index, we let replication know
        // that there are changes in our state that need to be replicated to proxies.
        // Without this call, even if we change our color index, new value would not
        // be replicated to proxies.
        Replication.BumpMe();
 
        // Presentation of replicated state on authority.
        RplExampleDebugShape.Cast(owner).SetColorByIdx(m_ColorIdx);
    }
 
    // Presentation of replicated state on proxy.
    private void OnColorIdxChanged()
    {
        RplExampleDebugShape.Cast(GetOwner()).SetColorByIdx(m_ColorIdx);
    }
```

And here is the full example code:

```
class RplExample2ComponentColorAnimClass : ScriptComponentClass { }
RplExample2ComponentColorAnimClass g_RplExample2ComponentColorAnimClass;
 
class RplExample2ComponentColorAnim : ScriptComponent
{
    private static const float COLOR_CHANGE_PERIOD_S = 5.0;
 
    private float m_TimeAccumulator_s;

    // We mark color index as replicated property using RplProp attribute, making
    // it part of replicated state. We also say we want OnColorIdxChanged function
    // to be invoked whenever replication updates value of color index.
    [RplProp(onRplName: "OnColorIdxChanged")]
    private int m_ColorIdx;

    override void OnPostInit(IEntity owner)
    {
        auto shapeEnt = RplExampleDebugShape.Cast(owner);
        if (!shapeEnt)
        {
            Print("This example requires that the entity is of type `RplExampleDebugShape`.", LogLevel.WARNING);
            return;
        }
 
        shapeEnt.SetColorByIdx(m_ColorIdx);
 
        // We must belong to some RplComponent in order for replication to work.
        // We search for it and warn user when we can't find it.
        auto rplComponent = BaseRplComponent.Cast(shapeEnt.FindComponent(BaseRplComponent));
        if (!rplComponent)
        {
            Print("This example requires that the entity has an RplComponent.", LogLevel.WARNING);
            return;
        }
 
        // We only perform simulation on the authority instance, while all proxy
        // instances just show result of the simulation. Therefore, we only have to
        // subscribe to "frame" events on authority, leaving proxy instances as
        // passive components that do something only when necessary.
        if (rplComponent.Role() == RplRole.Authority)
        {
            SetEventMask(owner, EntityEvent.FRAME);
        }
    }
 
    override void EOnFrame(IEntity owner, float timeSlice)
    {
        int colorIdxDelta = CalculateColorIdxDelta(timeSlice);
        ApplyColorIdxDelta(owner, colorIdxDelta);
    }
 
    private int CalculateColorIdxDelta(float timeSlice)
    {
        m_TimeAccumulator_s += timeSlice;
        int colorIdxDelta = m_TimeAccumulator_s / COLOR_CHANGE_PERIOD_S;
        m_TimeAccumulator_s -= colorIdxDelta * COLOR_CHANGE_PERIOD_S;
        return colorIdxDelta;
    }

    private void ApplyColorIdxDelta(IEntity owner, int colorIdxDelta)
    {
        if (colorIdxDelta == 0)
            return;
 
        int newColorIdx = (m_ColorIdx + colorIdxDelta) % RplExampleDebugShape.COLOR_COUNT;
        if (newColorIdx == m_ColorIdx)
            return;
 
        // Update replicated state with results from the simulation.
        m_ColorIdx = newColorIdx;
 
        // After we have written new value of color index, we let replication know
        // that there are changes in our state that need to be replicated to proxies.
        // Without this call, even if we change our color index, new value would not
        // be replicated to proxies.
        Replication.BumpMe();
 
        // Presentation of replicated state on authority.
        RplExampleDebugShape.Cast(owner).SetColorByIdx(m_ColorIdx);
    }
 
    // Presentation of replicated state on proxy.
    private void OnColorIdxChanged()
    {
        RplExampleDebugShape.Cast(GetOwner()).SetColorByIdx(m_ColorIdx);
    }
}
```

Considering this example in isolation, things are reasonably good. It is worth mentioning that what we marked as presentation (setting color used by entity to draw the sphere) is not all of it. Truly expensive parts, rendering and audio mixing, are skipped automatically when presentation is not necessary (such as on dedicated server). If you really wanted to make sure that our presentation is only doing work when necessary, you can use RplSession.Mode() to determine whether we are running in dedicated server mode or not. In general, it is best to avoid this unless absolutely necessary.

In larger context of the game, if there were many of these entities placed in the world, we might start seeing constant EOnFrame calls on authority take significant amount of time. We could improve things with use of ScriptCallQueue.CallLater(), specifying delay based on our color change period. This will only work well for long color change periods (on the order of seconds) where inaccuracy introduced is not significant. However, when using very short color change periods (on the order of milliseconds) we wouldn't be able to accurately determine how many periods have passed since last call.

If the game also provides a replicated time value, we have another possible approach to making sure animation is in sync across all machines. We can just take this value and calculate color index from it directly. This would require either checking this replicated time periodically (such as using EOnFrame) on all machines, or using ScriptCallQueue.CallLater() with delay being an estimate of when should next color change occur. Network bandwidth cost in this case would be essentially zero for our animation. Cost of replicating time may be potentially higher, but it is constant, predictable, and it doesn't increase with number of things in the world relying on it.

### Example system with per-player controller

So far, we have seen how to make simple non-interactive animation synchronized across network, with proper distinction between simulation and presentation parts. However, games are interactive medium and players play multiplayer games to interact with others in shared virtual world. So this time we will take a look at how to let server know what a player wants to do.

This time, instead of having our animation change colors in predefined period, we will be changing colors in reaction to player pressing keys on the keyboard. We will also create more shapes, where each will be controlled by different key. Whenever a key corresponding to specific shape is pressed, color of the shape changes to next color in the sequence (and again, last color is followed by the first, repeating sequence from the beginning).

When we have multiple players interacting with objects in single shared world, one situation we need to always consider is how to resolve conflicts when two players interact with the same object in contradicting ways. In our authoritative server architecture, there are two main ways to resolve this:

1. At any time, at most one player is allowed to interact with the object.
1. Multiple players are allowed to interact with the object and server resolves conflicts when they happen.

To allow implementing both of these approaches, replication has a concept of node ownership. A client who owns a node (which means he owns all items that belong to this node) is allowed to send messages to server. Server can give ownership of a node to client (or take it back) whenever it wants.

Ownership is natural fit for implementing the exclusive right to interact with an object. Let's say a player is only allowed to drive a car when they are sitting in driver's seat. Server gives them ownership over car when they get in the driver's seat and as soon as they leave, ownership is taken from them. Notice that there is clear moment when ownership is given to the client (sitting in the driver's seat) and taken from it (moving to another seat or leaving the car).

There are many cases where there is no natural moment when ownership change should occur. For example, when two players run up to some closed door and decide to open it. Giving ownership to client just to perform single action (opening the door), then taking it back, is unnecessarily complicated and will probably make the action feel clunky by adding extra latency. These situations are usually handled through some kind of server-side *system* which creates per-player *controller*. Ownership of the controller is given to the player and all interactions with this system happen through the controller.

```
class RplExample3ComponentColorAnimClass : ScriptComponentClass { }
RplExample3ComponentColorAnimClass g_RplExample3ComponentColorAnimClass;
 
class RplExample3ComponentColorAnim : ScriptComponent
{
    [RplProp(onRplName: "OnColorIdxChanged")]
    private int m_ColorIdx;
 
    override void OnPostInit(IEntity owner)
    {
        auto shapeEnt = RplExampleDebugShape.Cast(owner);
        if (!shapeEnt)
        {
            Print("This example requires that the entity is of type `RplExampleDebugShape`.", LogLevel.WARNING);
            return;
        }
 
        shapeEnt.SetColorByIdx(m_ColorIdx);
 
        auto rplComponent = BaseRplComponent.Cast(shapeEnt.FindComponent(BaseRplComponent));
        if (!rplComponent)
        {
            Print("This example requires that the entity has an RplComponent.", LogLevel.WARNING);
            return;
        }
    }
 
    void NextColor()
    {
        m_ColorIdx = (m_ColorIdx + 1) % RplExampleDebugShape.COLOR_COUNT;
        Replication.BumpMe();
        RplExampleDebugShape.Cast(GetOwner()).SetColorByIdx(m_ColorIdx);
    }
 
    private void OnColorIdxChanged()
    {
        RplExampleDebugShape.Cast(GetOwner()).SetColorByIdx(m_ColorIdx);
    }
}
 
class RplExample3SystemClass : ScriptComponentClass { }
RplExample3SystemClass g_RplExample3SystemClassInst;
 
class RplExample3System : ScriptComponent
{
    static const ResourceName s_ControllerPrefab = "{65B426E2CD4049C3}kroslakmar/RplExampleController.et";
    static const ResourceName s_SpherePrefab = "{1AD0012447ACCE3F}kroslakmar/RplExampleShape.et";
 
    ref RplExample3SessionListener m_SessionListener = new RplExample3SessionListener(this);
    ref map<RplIdentity, RplExample3Controller> m_Controllers = new map<RplIdentity, RplExample3Controller>();
 
    ref array<RplExample3ComponentColorAnim> m_Spheres = new array<RplExample3ComponentColorAnim>();
 
    override void OnPostInit(IEntity owner)
    {
        if (g_Game.InPlayMode())
            SetEventMask(owner, EntityEvent.INIT);
    }
 
    override void EOnInit(IEntity owner)
    {
        RplMode mode = RplSession.Mode();
        if (mode != RplMode.Client)
        {
            RplSession.RegisterCallbacks(m_SessionListener);
        }
 
        if (mode == RplMode.None || mode == RplMode.Listen)
        {
            RplExample3Controller controller = NewController(RplIdentity.Local());
            controller.RplGiven(null);
        }
 
        Resource prefab = Resource.Load(s_SpherePrefab);
        EntitySpawnParams spawnParams = new EntitySpawnParams();
        spawnParams.TransformMode = ETransformMode.WORLD;
        owner.GetWorldTransform(spawnParams.Transform);
        float xBase = spawnParams.Transform[3][0];
        float yBase = spawnParams.Transform[3][1] + 2.0;
        for (int y = -1; y <= 1; y++)
        for (int x = -1; x <= 1; x++)
        {
            spawnParams.Transform[3][0] = xBase + x;
            spawnParams.Transform[3][1] = yBase + y;
            IEntity ent = g_Game.SpawnEntityPrefab(prefab, owner.GetWorld(), spawnParams);
            m_Spheres.Insert(RplExample3ComponentColorAnim.Cast(
                ent.FindComponent(RplExample3ComponentColorAnim)
            ));
        }
    }
 
    RplExample3Controller NewController(RplIdentity identity)
    {
        ref Resource controllerPrefab = Resource.Load(s_ControllerPrefab);
        auto controller = RplExample3Controller.Cast(
            g_Game.SpawnEntityPrefab(controllerPrefab, GetOwner().GetWorld(), null)
        );
        controller.m_System = this;
        m_Controllers.Set(identity, controller);
 
        return controller;
    }
 
    void DeleteController(RplIdentity identity)
    {
        auto controller = m_Controllers.Get(identity);
        delete controller;
        m_Controllers.Remove(identity);
    }
 
    void ChangeColor(int idx)
    {
        m_Spheres[idx].NextColor();
    }
}
 
class RplExample3SessionListener: RplSessionCallbacks
{
    RplExample3System m_System;
 
    void RplExample3SessionListener(RplExample3System system)
    {
        m_System = system;
    }
 
    override void EOnConnected(RplIdentity identity)
    {
        RplExample3Controller controller = m_System.NewController(identity);
        auto rplComponent = BaseRplComponent.Cast(controller.FindComponent(BaseRplComponent));
        rplComponent.Give(identity);
    }
 
    override void EOnDisconnected(RplIdentity identity)
    {
        m_System.DeleteController(identity);
    }
};
 
class RplExample3ControllerClass : GenericEntityClass {}
RplExample3ControllerClass g_RplExample3ControllerClassInst;
 
class RplExample3Controller : GenericEntity
{
    static const KeyCode s_KeyMap[] = {
        KeyCode.KC_NUMPAD1,
        KeyCode.KC_NUMPAD2,
        KeyCode.KC_NUMPAD3,
        KeyCode.KC_NUMPAD4,
        KeyCode.KC_NUMPAD5,
        KeyCode.KC_NUMPAD6,
        KeyCode.KC_NUMPAD7,
        KeyCode.KC_NUMPAD8,
        KeyCode.KC_NUMPAD9,
    };
 
    RplExample3System m_System;
    int m_IsDownMask = 0;
 
    bool RplGiven(ScriptBitReader reader)
    {
        if (false)
        {
            SetEventMask(EntityEvent.FRAME);
        }
        else
        {
            SetEventMask(EntityEvent.FIXEDFRAME);
        }
        return true;
    }
 
    override void EOnFrame(IEntity owner, float timeSlice)
    {
        foreach (int idx, KeyCode kc : s_KeyMap)
        {
            int keyBit = 1 << idx;
            bool isDown = Debug.KeyState(kc);
            bool wasDown = (m_IsDownMask & keyBit);
            if (isDown && !wasDown)
                Rpc(Rpc_ChangeColor_S, idx);
 
            if (isDown)
                m_IsDownMask |= keyBit;
            else
                m_IsDownMask &= ~keyBit;
        }
    }
 
    [RplRpc(RplChannel.Reliable, RplRcver.Server)]
    void Rpc_ChangeColor_S(int idx)
    {
        if (idx < 0 || idx >= 9)
            return;
 
        m_System.ChangeColor(idx);
    }
 
    override void EOnFixedFrame(IEntity owner, float timeSlice)
    {
        int isDownMask = 0;
        int keyBit = 1;
        foreach (KeyCode kc : s_KeyMap)
        {
            if (Debug.KeyState(kc))
                isDownMask |= keyBit;
 
            keyBit <<= 1;
        }
        Rpc(Rpc_OwnerInputs_S, isDownMask);
    }
 
    [RplRpc(RplChannel.Unreliable, RplRcver.Server)]
    void Rpc_OwnerInputs_S(int isDownMask)
    {
        int inputsChanged = m_IsDownMask ^ isDownMask;
        if (!inputsChanged)
            return;
 
        for (int idx = 0; idx < 9; idx++)
        {
            int keyBit = 1 << idx;
            bool isDown = isDownMask & keyBit;
            bool wasDown = m_IsDownMask & keyBit;
            if (isDown && !wasDown)
                m_System.ChangeColor(idx);
        }
 
        m_IsDownMask = isDownMask;
    }
};
```

## Glossary

- **Server** - instance of the game that has the authority over the game state
- **Client** - instance of the game that connects to server
- **Proxy** - Mirror image of an item controlled by someone else (a replica)
- **Item** - instance of a type with replicated state, RPCs or replication callbacks
- **State** - collection of properties
- **Property** - member variable of a type
- **RPC** (*Remote procedure call*) - item member function that may be invoked over network
- **Attribute** - metadata attached to a property, member function or type
- **Snapshot** - copy of state at a specific point in time
- **Injection** - process of copying state from snapshot into an item
- **Extraction** - process of copying state from an item into snapshot

## Concepts

The core idea of our Replication System is code simplification, state synchronization and rpc delivery with the least amount of boilerplate possible.

Single-player, server and client code should utilize the same code path with minimal differences.

The authority in the system is shifted towards the server. This should bring more stability and security, but it may also create more load on the server side.

The Replication code is completely independent of the engine generic classes such as Entities and Components. The intention is to keep everything as lightweight as possible at a slight cost of added complexity.

## Data flow

Avoiding networked races and writing a secure logic is always a big challenge. Every time a race or a security breach occurs there's is a big chance that it will take a serious amount of debugging effort to track it down and fix it. Therefore we should try to avoid them by design. Replication brings set of rules of thumb and design choices that should help:

- **Item** has to be **explicitly inserted into the replication**.
  - The aim is to reduce the amount of items inserted into replication to an absolute minimum and reduce the pollution of engine types by replication code.
  - User code is the one who knows which items should and shouldn't be replicated, therefore he is the one responsible for item registration.
- There should always be **at most one *client* talking to a *server* for a given item**. Such *client* is the **owner** of the item.
  - This solves questions such as "Who sent the RPC first?", "Am I safe to modify the items data?", "Did someone override the value before I did?".
- **State** is always distributed **from server to clients**, never the other way around.
  - Synchronization from the client side would bring in the "Feedback loop" problem. If client changes state on the server then who should correct this client? If the server does, it will break the client data consistency which could bring adjustment from the client invoking a synchronization back to server in an infinite loop.
  - The state synchronization is basically a free access to the memory of the receiver. Allowing the synchronization from client side would be a big security risk.
  - State adjustment from client to server can be accomplished by an RPC which makes the code more readable.
- **Peer-to-Peer** (P2P) communication is strictly **forbidden**. Clients can only talk to server.
  - From the security perspective the client should always protect his sensitive data and share them only with the server. Not just positions/health/ammo_count are considered sensitive information. Also the client identity and IP are something that others shouldn't be able to access. The system would have to share those just by sending such message which is a big security violation.
  - Such communication should always be performed through trusted 3rd party which is the server.

## State replication

In order to get your properties replicated you need to **annotate** them with **the property attribute**. Once the item gets replicated all of the annotated properties will get checked for changes, extracted into snapshot and encoded into packets via the type **Codecs**. Most of the system types should have the codecs already implemented.

Process of state replication is roughly as follows:

- on server
  1. Replication.BumpMe() is used to signal that properties of an item have changed and they need to be replicated to clients. This is up to users to do as necessary. Replication does not automatically check for changes on all registered items.
  1. Replication compares replicated properties against the most recent snapshot using codec function PropCompare(). If codec says snapshot is the same as current state, process ends.
  1. Replication creates new snapshot and uses codec function Extract() to copy values from instance to snapshot.
  1. Snapshot is transmitted to clients as needed. This process has to deal with tracking multiple snapshots per item per connected client, join-in-progress, streaming, relevancy, etc. It often uses codec function SnapCompare() to determine whether two snapshots are the same. When a snapshot is finally being prepared for transmission over network, codec function Encode() will be used to convert snapshot into compressed form (using as few bits as possible for each value) suitable for network packet.
- on client
  1. When new packet with compressed snapshot arrives, it is decompressed using codec function Decode() first.
  1. Snapshots are compared using codec function SnapCompare() to determine whether changes have occurred.
  1. Replication updates properties of replicated item using codec function Inject().

Above description should give you some idea of where various codec functions fit into the state replication process, but it skips over many details. Specifically, when or if at all some codec function is called is complicated and subject to changes as replication is developed over time, so you should make no assumptions about that.

**Warning:** Injection always writes a new value into replicated property. For complex types (strings, arrays or classes), this means that a pointer to new instance overwrites previous pointer to the old one. Any data stored in the old instance that is not written during injection will be lost.

**Snapshots** are an important part of state replication and serve to decouple extraction/injection process from encoding/decoding into network packet. Following are some of the reasons for this separation:

- They are often used in comparisons between snapshots (SnapCompare()) or between snapshot and item state (PropCompare()). Especially second type of comparison would become more expensive if data optimized for network was used, as each property would have to be decoded again during every comparison.
- They can be encoded into network packet in full or as delta from previous state. Having original information provides more options when creating and storing delta.

The property annotation can be expanded by a bit of metadata to influence the replication. You can detect that your properties were updated using **OnRpl** callback and adjust the internal state of your item. Or use a **Condition** for certain special cases where you would need more control over who will be receiving updates. You can find below examples of both.

### Examples

```
class EntityWithRplProp : GenericEntity
{
    [RplProp()]
    bool bFlag;
 
    [RplProp(onRplName: "OnIValueChanged")]
    int iValue;
 
    [RplProp(condition: RplCondition.NoOwner)]
    float fValue;
 
    [RplProp(customConditionName: "MyCondition")]
    float fCustomCondition;
 
    void OnIValueChanged()
    {
        // ...
    }
 
    // The custom condition can be helpful in places where you know that
    // certain values are no longer needed on the other side. This can't be
    // utilized to filter out connections!
    bool MyCondition()
    {
        return bFlag;
    }
}
```

## Remote procedure calls (RPCs)

This is where Replication really shines. *RPCs* are routed to receivers by ownership rules so the user does not have to look up any identifier or address. The design leads the programmer towards uniform code in most *Client/Server* scenarios.

On sending side, codec function Extract() from corresponding RPC argument is used to create snapshot of relevant properties, and codec function Encode() then compresses this snapshot for network packet. On receiving side, codec function Decode() first decompresses data from packet into snapshot, then an instance is created and filled from snapshot using codec function Inject().

```
class EntityWithRplRpc : GenericEntity
{
    // This is a RPC. It has to be annotated with the RplRpc attribute.
    [RplRpc(RplChannel.Reliable, RplRcver.Owner)]
    void OwnerRpc(int a)
    {
        // ...
    }
 
    override void EOnFrame(IEntity owner, float timeSlice)
    {
        // The RPC has to be call via special method on the entity/component.
        Rpc(OwnerRpc, 7);
        // This call can have multiple outcomes depending on the role and
        // ownership of this instance.
        // 1. If this instance is the owner the RPC will be called directly
        //    (the same way a method would be).
        // 2. This instance is the server and some client owns the instance.
        //    Then the RPC would be called on the owning client.
        // 3. This is the client that does not own the instance. Then this
        //    RPC would be dropped.
        //
        // You can avoid a lot of branches if you design your code around these rules.
 
 
        // You can still use the RPC method the same way as you would use any other method.
        // In many cases this would be the best way how to unify your logic.
        OwnerRpc(7);
    }
}
```

### RPC routing table (RRT)

These tables specify where will be the RPC body invoked when you call it on either Server or Client engine instance.

RPC invoked from the server:

| Is owner | RplRcver Server | RplRcver Owner | RplRcver Broadcast |
| --- | --- | --- | --- |
| Owner | On Server | On Server | On all Clients |
| Not Owner | On Server | On Client Owner | On all Clients |

RPC invoke from the client:

| Is owner | RplRcver Server | RplRcver Owner | RplRcver Broadcast |
| --- | --- | --- | --- |
| Owner | On Server | Locally | Locally |
| Not Owner | Dropped | Dropped | Locally |

## Codecs

Replication uses codecs for various types that show up as either RPC arguments or replicated properties on items. Most system types already have codecs implemented, but when you attempt to use some user-defined type in one of these cases, you will have to implement a codec yourself. Codec consists of several static functions on user-defined type T:

- bool Extract(T instance, ScriptCtx ctx, SSnapSerializerBase snapshot)
  - Extracts relevant properties from an instance of type T into snapshot. Opposite of Inject().
- bool Inject(SSnapSerializerBase snapshot, ScriptCtx ctx, T instance)
  - Injects relevant properties from snapshot into an instance of type T. Opposite of Extract().
- void Encode(SSnapSerializerBase snapshot, ScriptCtx ctx, ScriptBitSerializer packet)
  - Takes snapshot and compresses it into packet. Opposite of Decode().
- bool Decode(ScriptBitSerializer packet, ScriptCtx ctx, SSnapSerializerBase snapshot)
  - Takes packet and decompresses it into snapshot. Opposite of Encode().
- bool SnapCompare(SSnapSerializerBase lhs, SSnapSerializerBase rhs, ScriptCtx ctx)
  - Compares two snapshots to see whether they are the same or not.
- bool PropCompare(T instance, SSnapSerializerBase snapshot, ScriptCtx ctx)
  - Compares instance and a snapshot to see if any property has changed enough to require new snapshot.
- (optional) void EncodeDelta(SSnapSerializerBase oldSnapshot, SSnapSerializerBase newSnapshot, ScriptCtx ctx, ScriptBitSerializer packet)
  - Produces delta-encoded packet from two snapshots. Opposite of DecodeDelta().
- (optional) void DecodeDelta(ScriptBitSerializer packet, ScriptCtx ctx, SSnapSerializerBase oldSnapshot, SSnapSerializerBase newSnapshot)
  - Produces new snapshot from delta-encoded packet and an older snapshot. Opposite of EncodeDelta().

**Note:** Implementing codec is a lot of extra work that is prone to bugs over time. Following are some cases where it may be better to not write codec and split user-defined type into parts instead:
- if a type is only ever used as an argument in one RPC
- if the codec only uses helpers and does not do anything fancy (like reducing number of bits during encoding)
- if you don't always encode all properties of the type

Here is an example of a user-defined type ComplexType and its codec functions:

```
class ComplexType
{
    bool m_Bool;
    int m_Int;
    string m_String;
    float m_Float;
    WorldTimestamp m_Timestamp;
    vector m_Vector;
 
    // ## Extract/Inject
    // Extracting data from instance into snapshot, and injecting data from snapshot to instance.
    // Snapshot is meant to be fast to work with, so values are left uncompressed
    // to avoid extra work when accessing these values.
 
    // ## Encode/Decode
    // Encoding snapshot into a packet and decoding snapshot from a packet.
    // Packets need to be as small as possible, so this process tries to reduce the
    // size as much as it can. Knowing what range of values can certain variable have and
    // encoding that range in minimum number of bits required is key. If you have
    // to assume full range of values is needed, you can use helpers for different
    // types that already implement those.
 
    // ## EncodeDelta/DecodeDelta
    // Optional functions for implementing encoding of snapshot differences
    // (delta encoding). Encoding reads from old and new snapshots and writes differences
    // between them into delta-encoded packet. Decoding then reads from old snapshot
    // and delta-encoded packet and writes new packet based on them.
 
    static bool Extract(ComplexType instance, ScriptCtx ctx, SSnapSerializerBase snapshot)
    {
        // Fill a snapshot with values from an instance.
        snapshot.SerializeBool(instance.m_Bool);
        snapshot.SerializeInt(instance.m_Int);
        snapshot.SerializeString(instance.m_String);
        snapshot.SerializeFloat(instance.m_Float);
        snapshot.SerializeBytes(instance.m_Timestamp, 8);
        snapshot.SerializeVector(instance.m_Vector);
        return true;
    }
 
    static bool Inject(SSnapSerializerBase snapshot, ScriptCtx ctx, ComplexType instance)
    {
        // Fill an instance with values from snapshot.
        snapshot.SerializeBool(instance.m_Bool);
        snapshot.SerializeInt(instance.m_Int);
        snapshot.SerializeString(instance.m_String);
        snapshot.SerializeFloat(instance.m_Float);
        snapshot.SerializeBytes(instance.m_Timestamp, 8);
        snapshot.SerializeVector(instance.m_Vector);
        return true;
    }
 
    static void Encode(SSnapSerializerBase snapshot, ScriptCtx ctx, ScriptBitSerializer packet)
    {
        // Read values from snapshot, encode them into smaller representation, then
        // write them into packet.
        snapshot.EncodeBool(packet);   // m_Bool
        snapshot.EncodeInt(packet);    // m_Int
        snapshot.EncodeString(packet); // m_String
        snapshot.EncodeFloat(packet);  // m_Float
        snapshot.Serialize(packet, 8); // m_Timestamp
        snapshot.EncodeVector(packet); // m_Vector
    }
 
    static bool Decode(ScriptBitSerializer packet, ScriptCtx ctx, SSnapSerializerBase snapshot)
    {
        // Read values from packet, decode them into their original representation,
        // then write them into snapshot.
        snapshot.DecodeBool(packet);   // m_Bool
        snapshot.DecodeInt(packet);    // m_Int
        snapshot.DecodeString(packet); // m_String
        snapshot.DecodeFloat(packet);  // m_Float
        snapshot.Serialize(packet, 8); // m_Timestamp
        snapshot.DecodeVector(packet); // m_Vector
        return true;
    }
 
    static bool SnapCompare(SSnapSerializerBase lhs, SSnapSerializerBase rhs , ScriptCtx ctx)
    {
        // Compare two snapshots and determine whether they are the same.
        // We have to compare properties one-by-one, but for properties with known
        // length (such as primitive types bool, int, float and vector), we do multiple
        // comparisons in single call. However, because we do not know length of string,
        // we use provided function which will determine number of bytes that need
        // to be compared from serialized data.
        return lhs.CompareSnapshots(rhs, 4+4)   // m_Bool, m_Int
            && lhs.CompareStringSnapshots(rhs)  // m_String
            && lhs.CompareSnapshots(rhs, 4+8+12); // m_Float, m_Timestamp, m_Vector
    }
 
    static bool PropCompare(ComplexType instance, SSnapSerializerBase snapshot, ScriptCtx ctx)
    {
        // Determine whether current values in instance are sufficiently different from
        // an existing snapshot that it's worth creating new one.
        // For float or vector values, you could use some threshold to avoid creating too
        // many snapshots due to tiny changes in these values.
        return snapshot.CompareBool(instance.m_Bool)
            && snapshot.CompareInt(instance.m_Int)
            && snapshot.CompareString(instance.m_String)
            && snapshot.CompareFloat(instance.m_Float)
            && snapshot.Compare(instance.m_Timestamp, 8)
            && snapshot.CompareVector(instance.m_Vector);
    }
 
    static void EncodeDelta(SSnapSerializerBase oldSnapshot, SSnapSerializerBase newSnapshot, ScriptCtx ctx, ScriptBitSerializer packet)
    {
        // Generate packet that allows other side to produce new snapshot when it already
        // has old one available.
 
        // We write new value of bool directly, as there is no way to reduce it below one bit.
        // We still need to read old value from old snapshot to correctly access following
        // properties.
        bool oldBool;
        oldSnapshot.SerializeBool(oldBool);
        bool newBool;
        newSnapshot.SerializeBool(newBool);
        packet.Serialize(newBool, 1);
 
        // We encode difference between old and new value of the int, and rely on the fact
        // that difference of these values requires fewer bits to encode than value itself.
        int oldInt;
        oldSnapshot.SerializeInt(oldInt);
        int newInt;
        newSnapshot.SerializeInt(newInt);
        int deltaInt = newInt - oldInt;
        packet.SerializeInt(deltaInt);
 
        // For remaining values (string, float, vector), we use single bit to signal whether
        // value has changed and we only encode their new value if it is different from
        // the old one.
        string oldString;
        oldSnapshot.SerializeString(oldString);
        string newString;
        newSnapshot.SerializeString(newString);
        bool stringChanged = newString != oldString;
        packet.Serialize(stringChanged, 1);
        if (stringChanged)
            packet.SerializeString(newString);
 
        float oldFloat;
        oldSnapshot.SerializeFloat(oldFloat);
        float newFloat;
        newSnapshot.SerializeFloat(newFloat);
        bool floatChanged = newFloat != oldFloat;
        packet.Serialize(floatChanged, 1);
        if (floatChanged)
            packet.Serialize(newFloat, 32);
 
        // For timestamps, rounding of float will occur when absolute value of
        // difference is more than 16777216ms (~4.5 hours). Also, past certain
        // point, differences cannot be represented as 32-bit int either. Therefore,
        // once difference cannot be sent accurately, we instead send full value.
        // This might not be the best solution for extreme uses, but it demonstrates
        // how one can implement delta-encoding using domain-specific knowledge.
        WorldTimestamp oldTimestamp;
        oldSnapshot.SerializeBytes(oldTimestamp, 8);
        WorldTimestamp newTimestamp;
        newSnapshot.SerializeBytes(newTimestamp, 8);
        float deltaMs = newTimestamp.DiffMilliseconds(oldTimestamp);
        bool isUsingDelta = -16777216.0 <= deltaMs && deltaMs <= 16777216.0;
        packet.Serialize(isUsingDelta, 1);
        if (isUsingDelta)
        {
            int deltaMsInt = deltaMs;
            packet.SerializeInt(deltaMsInt);
        }
        else
        {
            packet.Serialize(newTimestamp, 64);
        }
 
        vector oldVector;
        oldSnapshot.SerializeVector(oldVector);
        vector newVector;
        newSnapshot.SerializeVector(newVector);
        bool vectorChanged = newVector != oldVector;
        packet.Serialize(vectorChanged, 1);
        if (vectorChanged)
            packet.Serialize(newVector, 96);
 
        // More techniques are possible when modification patterns are known. For example,
        // when multiple properties are always modified together, they could share single
        // bit that would say whether there were any changes (and new values were encoded)
        // or not.
    }
 
    static void DecodeDelta(ScriptBitSerializer packet, ScriptCtx ctx, SSnapSerializerBase oldSnapshot, SSnapSerializerBase newSnapshot)
    {
        // Generate new snapshot using data from old snapshot and packet.
        // Note that even when value from old snapshot is not used because packet carries
        // new value, we always read it to make sure we correctly access following
        // properties that may not have changed.
 
        // Bool value is read directly from packet, just as it was written.
        bool oldBool;
        oldSnapshot.SerializeBool(oldBool);
        bool newBool;
        packet.Serialize(newBool, 1);
        newSnapshot.SerializeBool(newBool);
 
        // New value of int is reconstructed by applying delta to old value.
        int oldInt;
        oldSnapshot.SerializeInt(oldInt);
        int deltaInt;
        packet.SerializeInt(deltaInt);
        int newInt = oldInt + deltaInt;
        newSnapshot.SerializeInt(newInt);
 
        // Remaining properties are reconstructed by checking whether they changed
        // and either reading new value from packet, or copying old value.
        string oldString;
        oldSnapshot.SerializeString(oldString);
        bool stringChanged;
        packet.Serialize(stringChanged, 1);
        string newString;
        if (stringChanged)
            packet.SerializeString(newString);
        else
            newString = oldString;
        newSnapshot.SerializeString(newString);
 
        float oldFloat;
        oldSnapshot.SerializeFloat(oldFloat);
        bool floatChanged;
        packet.Serialize(floatChanged, 1);
        float newFloat;
        if (floatChanged)
            packet.Serialize(newFloat, 32);
        else
            newFloat = oldFloat;
        newSnapshot.SerializeFloat(newFloat);
 
        WorldTimestamp oldTimestamp;
        oldSnapshot.SerializeBytes(oldTimestamp, 8);
        bool isUsingDelta;
        packet.Serialize(isUsingDelta, 1);
        WorldTimestamp newTimestamp;
        if (isUsingDelta)
        {
            int deltaMsInt;
            packet.SerializeInt(deltaMsInt);
            float deltaMs = deltaMsInt;
            newTimestamp = oldTimestamp.PlusMilliseconds(deltaMs);
        }
        else
        {
            packet.Serialize(newTimestamp, 64);
        }
        newSnapshot.SerializeBytes(newTimestamp, 8);
 
        vector oldVector;
        oldSnapshot.SerializeVector(oldVector);
        bool vectorChanged;
        packet.Serialize(vectorChanged, 1);
        vector newVector;
        if (vectorChanged)
            packet.Serialize(newVector, 96);
        else
            newVector = oldVector;
        newSnapshot.SerializeVector(newVector);
    }
}
```

## Networked logic

You will have to start thinking at a bigger scale If you want to write **readable and unified replication code**. Keep in mind that you want to reuse the most of your replication code and still keep it readable **for all of the application use-cases (listen server, dedicated server, single-player)**. This is not an easy task but trust me it will save you time and a lot of typing in the long run.

To give you more context about the current usage of your item the replication uses **RplNode** structures. These are immutably bound to your items and function as a proxy between your code and replication layer. I won't talk about how to create and maintain them as it will be explained in depth later on. Now they will just give us two pieces of context: **Role and Ownership**. These are the strongest tools you will get from the replication layer. Lets look at an example:

```
// The task is to control a car in multiplayer.
class CarControllerComponent : ScriptComponent
{
    // Lets assume that we already got the node from somewhere.
    private RplNode m_Node;
 
    // We would like our logic to fulfill following constraints.
    // 1. Server is in charge of the transformation of the car.
    // 2. The player provides input to move the car around.
    // 3. Everybody should see the car moving.
    override void EOnFrame(IEntity owner, float timeSlice)
    {
        // Theres three parts that wee need to be handled.
 
        // 1. Authority (the server)
        // There are only two roles at the moment (Authority and Proxy).
        if(m_Node.Role() == RplRole.Authority)
        {
            CalculateNewTransform(...);
        }
 
        // 2. Owner (the driving client)
        if(m_Node.IsOwner())
        {
            SendInput(...);
        }
 
        // 3. Everybody
        MoveCar(...);
    }
 
    void CalculateNewTransform(...)
    {
        // ...
        // Calculate new target transformation based on players input.
        // ...
    }
 
    void SendInput(...)
    {
        if (KeyDown(...))
        {
            // ...
            // Send to authority Throttle/Break.
            // ...
            // Send to authority Left/Right.
            // ...
        }
        // For authority only localy record the input.
        // RPCs do this automatically for you but that will be covered later.
    }
 
    void MoveCar(...)
    {
        // ...
        // Move the car when authority provides new target transformation.
        // ...
    }
}
```

Now put the above example into different settings and debug whats happening.

### Setting 1: Listen server and a client

Two car instances. One owned by the server player and second owned by the client.

- **Server and his car**: CalculateNewTransform() → SendInput() → MoveCar()
- **Server and client's car**: CalculateNewTransform() → MoveCar()
- **Client and server's car**: MoveCar()
- **Client and his car**: SendInput() → MoveCar()

Our constraints hold for both of our instances on both sides. The server controls the transform, car owner provides input and everybody is moving both of the instances around.

### Setting 2: Dedicated server and two clients

Two car instances (car1, car2). One owned by each client (client1, client2).

- **Server car1**: CalculateNewTransform() → MoveCar()
- **Server car2**: CalculateNewTransform() → MoveCar()
- **Client1 car1**: SendInput() → MoveCar()
- **Client1 car2**: MoveCar()
- **Client2 car1**: MoveCar()
- **Client2 car2**: SendInput() → MoveCar()

Once again our constraints hold for all instances on all sides. The server controls the transformations of both cars and every client can control only his own car. Everybody sees cars moving.

### Setting 3: Single-player

One car on single instance ("server" that doesn't allow client connections). There is no need to call different code path or make specific changes for single-player. Roles and ownership should take care of the logic.

- **Server**: CalculateNewTransform() → SendInput() → MoveCar()

As you can see there is no difference in behavior. The player would be still able to drive the car even when not running a multiplayer game.

### Takeaway - TLDR

The replication provides you with tools to structure your networked code in minimal and readable manner.

There's a bit of mental gymnastics involved along with a few rules of thumb that will carry you through the process:

- Base your logic around the **roles and ownership**.
- Your code will be running in more than one setting. **Verify that your assumptions** also work for the others.
- Use **real examples** to reason about the design and put them into context of different settings**.
- Before reaching out to some utility like "IsDedicatedServer()" or "IsClient()" think twice**.
  - **These will be available somewhere in the codebase but instead of making your job easier they will make it much much harder.**
  - **Think of those as usage of hardcoded constants instead of adaptible state machine.**
- When you find a case where these tools don't fit then this is not your hammer. Use something else.
- **Always proceed with caution.** - *Nothing takes longer than fixing a bug in distributed asynchronous codebase.*

## Item registration

### Lightweight using enf::RplNode

Any **enf::BaseItem** derived object (**Item**) can utilize the State synchronization and rpcs. We will discuss in this section how to integrate replication into the codebase and what we will get from doing so.

1. **Derive** your classes/structs **from enf::BaseItem** and properly implement the ENF_DECLARE_ITEM(...) macro. **This is a crucial step! Beware!**
1. Now you need an **enf::RplNode or its derived instance** that will serve as the container for your Items. Here you have a couple of options:
   - Just raw **instantiate** the enf::RplNode.
   - Make the enf::RplNode **member** of one of the items.
   - **Derive** one of Items from the enf::RplNode.
1. When all of your Items are constructed and ready **insert them into the node** via his API.
1. (optional) The enf::RplNode is a hierarchical structure. You may set the **parent child relations** between your nodes at this point.
1. **Call the self registration** method of the enf::RplNode InsertToReplication(...) (or its replacement if defined in the derived type).

Now you should have your items successfully registered and ready to synchronize their state and send/receive RPC messages.

Your items will receive the RplId (a unique identifier in the networked environment), replication role and ownership information. You should design your logic around these as they will help you to unify your code for every use-case using replication (single player, listen server, dedicated server, client).

**Todo:** Node owner (first item in the node) → Creation + Destruction

### Entity system using gamelib::BaseRplComponent

**Todo:** ease of use and relation to RplNode ...

## Streaming

**Todo:** Item creation, destruction, hierarchy and overall lifetime in replication. (The complicated stuff ...)

## Life cycle

**Todo:** draw

## Caveats

- When running multiple instances of the GameApp with -forceupdate flag on the same machine consider using also -maxFPS XX flag to balance out the load of both instances. Otherwise you could end up with foreground instance running at 200 fps and the background instance at 5fps because of the operating system prioritization.
