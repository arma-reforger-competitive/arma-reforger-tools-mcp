---
title: Arma Reforger:Behavior Editor: Nodes
url: https://community.bistudio.com/wiki/Arma_Reforger:Behavior_Editor:_Nodes
---

Flow nodes define base rules how and with which conditions, are behavior tree branches executed.

### Flow

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Root | The starting point of a behavior tree. |  | Cannot be removed or inserted from the nodes palette. |
| Selector | Executes children until one of them returns Success.<br>Returns:<br>- Running: A child node still runs<br>- Success: A child node returned success<br>- Fail: All children returned fail |  |  |
| Sequence | Executes children from left to right as long as they return success. Returns success only if all children returned success.<br>Returns:<br>- Running: A child node still runs<br>- Success: All children returned success<br>- Fail: Any child returned fail |  |  |
| Parallel | Executes all children regardless what they return.<br>Returns:<br>- Running: If no child is assigned for getting its result<br>- Success: If the index of a child is assigned and the assigned child returns success<br>- Fail: If the index of a child is assigned and the assigned child returns fail | Parameters:<br>- Use Child Result: Select index of a child node, which result will be the result of the parallel node. |  |
| Repeater | Sequence node that is executed multiple times.<br>Returns:<br>- Running: A child node still runs<br>- Success: All children returned success<br>- Fail: Any child returned fail | Parameters:<br>- Repeat Times: Specifies how many times you want to execute. Default is once. |  |
| Run BT | Runs selected Behavior Tree on the agent.<br>Returns:<br>- Running: If the subtree returns running<br>- Success: If the result of the subtree is success<br>- Fail: If the result of subtree is fail, or the subtree could not be executed | Parameters:<br>- Behavior Tree: Specify behavior tree to run from the list.<br>- Run Repeatedly:<br>- True (Checked): Initialise the subtree once, then run it repeatedly without reinitialization<br>- False (Unchecked): Reinitialise and run the subtree every time.<br>- Variables in Array: New, unfinished feature<br>Input:<br>- InBehaviorTree: Behavior Tree to run |  |
| Run Once | Executes all of its children only once.<br>Returns:<br>- Success: Always |  | Intended to be used for one-time-only tasks, such as initialization. |
| Run On Entity | Executes children on a different entity than the owner of the behavior tree.<br>Returns:<br>- Success: If the children were executed on entity successfully and returned success or running<br>- Fail: If one of the children returns fail or the entity is not found | Input:<br>- Entity | Should be used only with nodes and tasks that are not responding with Running result (reason: currently BT does not switch to scope of entity on which you wish to run those nodes, stays in scope of main BT) |
| Switch | Picks children depending on a variable value or randomly.<br>Returns:<br>- Running: A child node still runs<br>- Success: A child node returned success<br>- Fail: It is aborted because of the variable change, the child returned fail, or could not find a proper child to run | Parameters:<br>- Values Array: For each children maximum value of in variable<br>- Random Range: For each children probability of execution. Specifies the range.<br>- Abort Variable Changed:<br>- True (Checked): Abort if the input variable is changed. The default is checked.<br>- False (Unchecked): Do not abort if the input variable is changed<br>Input:<br>- InVariable: The variable that may be tracked for aborting (if abort variable changed is checked) and/or for picking the child node to run from the values array |  |
| For Each Child | Iterates through all group children.<br>Returns:<br>- Running: A child node still runs<br>- Fail: If no entity, no AI group, no agent in the AI group, or a child returns fail<br>- Success: Otherwise | Parameters:<br>- Index from: Which should be starting index in an array. (Default: 0)<br>- Index to: Which should be max index to select. (-1 by default, to go through whole array)<br>- Return controlled entity: Return controlled entity, if false, return AI Agent.<br>Output:<br>- Entity |  |

### Decorators

Decorators are test checking different states and variables, and determine if child or child branch should be executed.

Decorators

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Decorator | Node which tests specific condition. If condition result is true, decorator will execute its child and return true.<br>Returns:<br>- Success: The child node returned success or when negating the result is selected and the child returns fail<br>- Fail: The child node returned fail or when negating the result is selected and the child returns success | **Common decorator parameters**:<br>- Negative result: Decorator will return negative value of the test.<br>- Use child result: Decorator will execute the child and return its result.<br>- Always true: Decorator will execute child node and then return true.<br>- Abort type:<br>- None: Do not abort anything<br>- Abort children branch: Abort all children under this node.<br>- Abort parent node with children: Abort self and all children under this node.<br>- Abort parent node further children: Abort parent node and its children, but proceed with children nodes. | Useful for tests through its parameters – use child result, aborting, etc.<br>Combination of parameters can be used: When you use always true and negative result, decorator will also execute its child and return false. It's useful in case when you want your branch to return false, but execute child node in any case. |
| Test Variable | Decorator checking provided variable for assignment, comparation against value, or check for value change.<br>Returns:<br>- Success: The child node returned success or when negating the result is selected and the child returns fail<br>- Fail: The child node returned fail or when negating the result is selected and the child returns success | Parameters:<br>- Test value: Specific value to test.<br>- Test type: Check if value is:<br>- Assigned<br>- Variable equals value<br>- Variable bigger than value<br>- Variable lower than value<br>- Variable was changed<br>Inputs:<br>- Variable |  |
| Test Entity | Various tests on entity. Decorator checks validity of entity, either connected through variable, or according to its name. Can test other conditions like life state.<br>Returns:<br>- Success: The child node returned success or when negating the result is selected and the child returns fail<br>- Fail: The child node returned fail or when negating the result is selected and the child returns success | Parameters:<br>- Entity name<br>- TEST: Space for selecting new kinds of tests, not working at the moment.<br>Input:<br>- Entity |  |
| Test Distance to Entity | Checks distance of entity with treshold value. Returns true if distance is smaller than treshold by default.<br>Check for ditance larger than treshold can be done with Use Negative Value parameter.<br>Returns:<br>- Success: The child node returned success or when negating the result is selected and the child returns fail<br>- Fail: The child node returned fail or when negating the result is selected and the child returns success | Parameters:<br>- Entity name: Search entity by name.<br>- Distance treshold: Until what distance result will return true<br>Inputs:<br>- Entity<br>- Position<br>- Distance |  |
| Danger Event | Checks agent if he has any danger events. Test can be performed against any or specified danger event.<br>Returns:<br>- Success: The child node returned success or when negating the result is selected and the child returns fail<br>- Fail: The child node returned fail or when negating the result is selected and the child returns success | Parameters:<br>- Has Any: check for any type of danger event<br>- Danger event type: list of available danger events to check<br>- Distance treshold: Limit search for danger events to certain radius. Default 0 = range not limited. | - Is there possibility to test several possible danger events in one decorator? (projectile hit + damage taken)<br>Danger event is type of event about threat to entity, like someone fired, someone was killed, etc. They all have specified position in world. |
| Test Order | Tests if agent has order, or has specific order according to parameters.<br>Returns:<br>- Success: The child node returned success or when negating the result is selected and the child returns fail<br>- Fail: The child node returned fail or when negating the result is selected and the child returns success | Parameters:<br>- Has Any<br>- Order Type |  |
| Aiming | Tests aiming<br>Returns:<br>- Success: The child node returned success or when negating the result is selected and the child returns fail<br>- Fail: The child node returned fail or when negating the result is selected and the child returns success | Parameters:<br>- Test type: What test to run<br>- Is aimed<br>- Is aiming in limits<br>- Aim Threshold: Value for testing |  |

### Scripted Decorators

Decorators created in script, inherited from Scripted Decorator.

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Scripted Decorator | Scripted base class of decorator, used for inheritence of other scripted decorators. | **Common decorator params**:<br>- Negative result: Decorator will return negative value of the test.<br>- Use child result: Decorator will execute the child and return its result.<br>- Always true: Decorator will execute child node and then return true.<br>- Abort type:<br>- None: Do not abort anything<br>- Abort children branch: Abort all children under this node.<br>- Abort parent node with children: Abort self and all children under this node.<br>- Abort parent node further children: Abort parent node and its children, but proceed with children nodes. | Does not contain any new functionality than basic decorator. |

### Tasks

Leaf nodes tasked to perform specific operation. They can return true, false and running.

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Get Controlled Entity | Finds character entity controlled by agent running Behavior tree.<br>Return:<br>- True: Entity was found<br>- False: Entity was not found | Parameters:<br>- Character Only: If true, accepts only character as controlled entity, not vehicle for example.<br>Output:<br>- Entity | AI is composed of two entities, character and AI Agent. This is common way how to access character from Agent in Behavior tree. |
| Create Position | Creates position according to specifie vector or position of entity. It's possible to randomise the position from its given center.<br>Return:<br>- True: Position was created.<br>- False: Failed to create position. | Parameters:<br>- Entity name: Calculate vector according to entity<br>- Offset Vector:<br>- Random Offset Multiplier:<br>Input:<br>- Root vector<br>- Offset vector<br>- Entity<br>Output:<br>- Position | Position output has to be saved as variable, it cannot be used directly as input node. |
| Follow Path | Follow given path<br>Return:<br>- True: Move was completed<br>- False: Path couldn't be found or move is invalid<br>- Running: Movement on the path in progress | Parameters:<br>- Path points: Custom array of points to follow, optional<br>Input<br>- Target Location |  |
| Move to Entity | Requests move to position of entity. This position is regularly updated, it should be used to move to target which can move.<br>Return:<br>- True: Move was completed.<br>- False: Failed to add action on entity.<br>- Running: Movement to entity in progress. | Parameters:<br>- Precision XZ: Precision of ground position.<br>- Target object name: Name of the entity to find and follow.<br>- Entity update precision: How far away needs the entity to be to update the move request.<br>Input:<br>- Entity |  |
| Move | Request move to specific position. Use to move to a static target.<br>Return:<br>- True: Move was completed.<br>- False: Failed to add action on entity.<br>- Running: Movement in progress. | Parameters:<br>- Target location: In world location to go to<br>- Precision XZ: Precision of ground position.<br>- Precision Y: Height precision.<br>- Target rotation: Rotation of entity during action.<br>- Target rotation to: Rotation related to specified entity (sun for example).<br>- Rotation precision: Angle in degrees<br>- Time to complete: Desired time to move completion (default: -1: Unlimited time)<br>- Enable avoidance: Avoid obstacles during the action.<br>Input:<br>- Target location<br>- Target rotation |  |
| Aim | Requests aiming on entity, position or specified direction.<br>Returns:<br>- True – task succesfully finished<br>- Running – task is ongoing<br>- False – request failed | Parameters:<br>- Precision XZ<br>- Precision Y<br>Input:<br>- Direction<br>- Aim at position<br>- Entity |  |
| Get in Vehicle | Moves character into vehicle.<br>Return:<br>- True: Entity can get into vehicle<br>- False: Unit is in vehicle already or cannot board | Parameters:<br>- Position:<br>- Pilot<br>- Gunner<br>- Turret<br>- Cargo<br>- Vehicle Name: Select vehicle by name<br>Input:<br>- Vehicle |  |
| Land | Realizes movement over landing position and descend into position.<br>Return:<br>- True: Land was completed.<br>- False: Agent failed landing or cannot move.<br>- Running: Landing in progress. | Parameters:<br>- Target location<br>- Precision XZ: Precision of ground position.<br>- Precision Y: Height precision.<br>- Target rotation: Rotation of entity during action.<br>- Target rotation to: Rotation related to specified entity (sun for example).<br>- Rotation precision<br>- Time to complete: Desired time to move completion (default: -1: Unlimited time)<br>- Enable avoidance: Avoid obstacles during the action.<br>Input:<br>- Target location<br>- Target rotation |  |
| Stop | Cancel movement and/or aiming of entity.<br>Return:<br>- True: Entity can be stopped<br>- False: Stopping failed | Parameters:<br>- Stop flags:<br>- Movements<br>- Aiming |  |
| Get Out Vehicle | Request disembarking from vehicle.<br>Return:<br>- True: Unit is in vehicle<br>- False: Unit is outside vehicle |  |  |
| Set Movement Speed | Request change of movement speed of controlled entity.<br>Return:<br>- True: Movement speed set.<br>- False: Failed to set movement speed. | Parameters:<br>- Speed: Wanted movement speed (m/s) |  |
| Orient | Requests change of orientation of controlled entity.<br>Return:<br>- True: Request completed.<br>- False: Request failed.<br>- Running: During orientation change. | Parameters:<br>- Rotation precision<br>- Target rotation to: Rotation related to specified entity (sun for example).<br>Input:<br>- Target location |  |
| Enable Obstacle Avoidance | Allows or disables obstacle avoidance for controlled character.<br>Return:<br>- True: Change of obstacle avoidance succeeded.<br>- False: Change of obstacle avoidance failed. | Parameters:<br>- Obstacle avoidance<br>Input:<br>- Obstacle avoidance |  |
| Get Aiming Position | Calculate the position of the target. It's possible to predict target's position in future using parameter Lead movement time. This node will not calculate any ballistics, just tracks the position of the target and interpolate the result.<br>Return:<br>- True: Position found.<br>- False: Failed to find position. | Parameters:<br>- Lead movement time - Predict position in future, using current velocity<br>Input:<br>- Entity<br>Output:<br>- Position |  |
| Send Order | Creates order and add it to order list. Can be used on any entity, controlled entity is selected by default. Any entity can be also selected as sender.<br>Return:<br>- True: Order was added to target entity.<br>- False: Order failed to add. | Parameters:<br>- Order type<br>- None<br>- Hold<br>- Move<br>- Follow<br>- Rearm<br>- Defend<br>- Get In<br>- Highest priority - If true, order will be first one in queue. If false will be queued.<br>Input:<br>- Sender<br>- Receiver<br>- Position<br>- Entity<br>- Order type |  |
| Idle | Wait for given seconds. Waiting time can be randomised.<br>Return:<br>- True: Timer is finished<br>- Running: Timer is active | Parameters:<br>- Period - For how long to idle<br>- Period random - Random dispersion<br>Input:<br>- Period key |  |
| Current Order | Get data about current order.<br>Return:<br>- True: Current order data received.<br>- False: Failed to receive current order data. | Output:<br>- Order object<br>- Order position<br>- Order type |  |
| Finish Order | Finishes current order.<br>Return:<br>- True – if there was order and it was finished<br>- False – test failed or order was not finished |  |  |

### Utility Tasks

Tasks handling very basic functions as operations with variables.

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Set/Clear Variable | Allows for setting value or clearing variable.<br>Return:<br>- True: Value set or cleared successfully.<br>- False: Failed to set/clear variable. | Parameters:<br>- Set value - What value should variable have<br>- Clear variable - If value should be cleared (deassigned) |  |
| Find Entity | Find entity by name or class name.<br>Return:<br>- True: Entity found.<br>- False: Entity not found. | Parameters:<br>- Entity name - Search by entity name<br>- Entity class name - Search by class name<br>- Distance test - if bigger than 0, returns entity only if closer than this distance<br>Output:<br>- Entity |  |

### Scripted Tasks

Scripted variants of Task, inherited from scriptedTask.

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Find Target | Example scripted node for finding target in certain radius. For now enemy is any character entity other than BT agent or player character.<br>Return:<br>- True: Target found.<br>- False: Target not found. | Input:<br>- Radius: In what distance should target be searched.<br>Output:<br>- Target: Target character. Outputs null if entity not found. |  |

### Character Tasks

Tasks that can be used only on characters. They will not work on vehicles, turrets, etc.

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Fire | Request fire action on controlled character.<br>Return:<br>- True: Action was requested<br>- False: Requester is not a character |  |  |
| Change Stance | Request stance change on character.<br>Return:<br>- True: Action request succeeded<br>- False: Action request failed | Parameters:<br>- Stance - Wanted Stance<br>- Prone<br>- Crouch<br>- Stand<br>Input:<br>- Stance |  |
| Throw Grenade | Request throw grenade action on controlled character.<br>Return:<br>- True: Action was requested<br>- False: Requester is not a character |  | This node might be obsoleted in future, depends on implementation of grenade throwing. |
| Character Raise Weapon | Request action to raise or lower weapon.<br>Return:<br>- True: Action was requested<br>- False: Requester is not a character | Parameters:<br>- Raise<br>- True<br>- False |  |
| Character Set Movement Speed | Request change of movement type on controlled entity.<br>Return:<br>- True: Action was requested<br>- False: Requester is not a character | Parameters:<br>- Movement type - Wanted movement speed<br>- Idle<br>- Walk<br>- Run<br>- Sprint |  |
| Play Gesture | Attempts to find a gesture by name and request it.<br>Returns:<br>- True: If succeeded<br>- False: Gesture not found | Parameters:<br>- Gesture: Pick gesture type from the list<br>- Greeting<br>- Point finger<br>- Thumb up<br>- Thumb down<br>- Silence<br>- Middle finger<br>- Timeout<br>- Heart<br>- Facepalm<br>- Watching<br>- Hold<br>- Listening<br>- Point at myself<br>- Look at me<br>- Play: Play gesture if TRUE; Stop gesture if FALSE |  |
| Perform Object Action | Finds and performs action on object, for example open door.<br>Returns:<br>- True: Action found and attempted<br>- False: no object, no character or no action | Input<br>- Action Entity |  |

### Waypoints

Tasks related for handling waypoints.

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Get Waypoint | Finds and returns waypoint with selected index. | Parameters:<br>- Index - From 0 up, index of waypoint in current queue<br>Input:<br>- Index<br>Output:<br>- Waypoint<br>- Behavior<br>- Contact behavior<br>- Danger behavior |  |

### Group

Tasks related to group management.

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Get Group children | Iterates through array of group children, and output child of current index as variable. It can be specified which indexes should be iterated through.<br>Return:<br>- True: Entity found<br>- False: Invalid or empty group | Parameters:<br>- Index from: Which should be starting index in an array. (Default: 0)<br>- Index to: Which should be max index to select. (-1 by default, to go through whole array)<br>- Return controlled entity: Return controlled entity, if false, return AI Agent.<br>Output:<br>- Entity | Example:<br>Behavior tree that will cause all children of the group stop. This BT has to be called multiple times, as Get Group children only stores correct index of array, but cannot call anything multiple times.<br>Note: This handling of "forEach" might change in the future. |
| Create Group |  |  | This seems to be WIP node. |

### Perception

Tasks related to AI Agent perception, which is basic source for finding threats and targets.

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Pick Target | Picks one target from known target list, and output it as variable.<br>Return:<br>- True: Target found and assigned<br>- False: No target found | Output:<br>- Entity |  |

### Action Tasks

Task nodes helping with performing actions.

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Request Action | Attempt to request specified action on controlled entity.<br>Return:<br>- True: Action requested succesfully<br>- False: Invalid action requested, or unable to complete request. | Parameters:<br>- Action name - as defined by input configuration<br>- Action value<br>Input:<br>- Action name<br>- Action value |  |

### Smart Actions

Tasks related to smart actions. Smart actions are objects in world, with which AI can interact. Bench can have smart action for sitting.

| Name | Description | Parameters/Input/Output | Notes |
| --- | --- | --- | --- |
| Perform Smart Action | Makes controlled entity to perform smart action<br>Return:<br>- True: Smart action assigned to entity.<br>- False: Smart action assignment failed. | Input:<br>- Smart action |  |
| Find Smart Action | Allows finding smart object in the world which support smart action and save it into variable. If multiple smart actions found, random is selected.<br>Return:<br>- True: Smart action found.<br>- False: Smart action not found. | Parameters<br>- Radius: The radius to search for actions within.<br>- Tags: Tags of smart action<br>- Tags test: Way of testing tags<br>Output:<br>- Smart action |  |
| Get Current Smart Action | Get smart action object currently realised by Agent.<br>Return:<br>- True: Always. | Output:<br>- Smart action |  |
| Get Smart Action Pos | Get position of given smart action.<br>Return:<br>- True: Always. | Input:<br>- Smart action<br>Output:<br>- Action pos |  |
| Get Smart Action BT | Returns behavior tree for given smart action.<br>Return:<br>- True: Always. | Input:<br>- Smart action<br>Output:<br>- Behavior |  |

### Experimental / Work in Progress

These nodes might not be finished or are used in some experimental or unfinished systems. These nodes are changed often, so it's not recommended to use them.

| Name | Description | Parameters | Notes |
| --- | --- | --- | --- |
| Flocking | Implements features | Parameters:<br>- Separation distance<br>- Neighborhood radius<br>- Accumulation type<br>- Ground<br>- Aerial |  |
| Get Random Point |  |  |  |
| Scripted Target | Allows for overriding in scripts |  |  |
