---
title: Arma Reforger:Server Management
url: https://community.bistudio.com/wiki/Arma_Reforger:Server_Management
---

**Note:** See also: [Server Hosting](https://community.bistudio.com/wiki/Arma_Reforger:Server_Hosting).

## Administrator Roles

An in-game server administrator can be:
- the player hosting the server (PC only)
- a logged-in admin, using `#login passwordAdmin` (see login and [passwordAdmin](https://community.bistudio.com/wiki/Arma_Reforger:Server_Config#passwordAdmin) config)
- a voted-in admin

## Permissions

**Standard**

| Right | Player Role |  |  | RCON Permission |  |
| --- | --- | --- | --- | --- | --- |
| Right | **Logged Admin** | **Voted Admin** | **Player** | **RCON Admin** | **RCON Monitor** |
| `#login` | No | No | Yes | No | No |
| `#logout` | Yes | Yes | No | No | No |
| `#roles` | Yes | Yes | Yes | No | No |
| `#restart` | Yes | No | No | Yes | No |
| `#shutdown` | Yes | No | No | Yes | No |
| `#kick` | Yes | No | No | Yes | No |
| `#ban` | Yes | No | No | Yes | No |
| `#id` | Yes | Yes | Yes | No | No |
| `#players` | Yes | Yes | Yes | Yes | Yes |

**Custom**

| Right | Player Role |  |  | RCON Permission |  |
| --- | --- | --- | --- | --- | --- |
| Right | **Logged Admin** | **Voted Admin** | **Player** | **RCON Admin** | **RCON Monitor** |
| Arma Reforger 1.2.1 `@logout` | No | No | No | Yes | Yes |

## Commands

### login

Login as server administrator with the password set in server config's field `passwordAdmin`.

**Note:** Admins (in server config) or Owner (in `ownerToken.bin`) can use the command without password.

**Syntax**

```
#login
#login <password>
```

**Example**

```
#login
#login myServersPassword
```

### logout

**Example**

```
#logout
```

### roles

Print the list of server roles the player possesses.
**Example**

```
#roles
```

### restart

Restart the actually running scenario. The server keeps the clients connected.
**Example**

```
#restart
```

### shutdown

Shut down the server, disconnecting all the clients.
**Example**

```
#shutdown
```

### kick

Kick (eject) the player out of the server through their playerId (obtained by players). Said player can still rejoin the server.
**Syntax**

```
#kick <playerId>
```

**Example**

```
#kick 123456789
```

### ban

This command prefixes ban-related commands: "create", "remove", "list"
Ban (eject and banish forever or for a certain duration in seconds) the player out of the server through their playerId (obtained by players). Said player cannot rejoin until ban expiration.

**Important:** This command can only be executed on a dedicated server.

#### create

Create a ban entry for the player related to the provided id.

**Syntax**

```
#ban create <playerId or identityId or Arma Reforger 1.2.0 playerName> <durationInSeconds> <reason (optional)>
```

if using `playerId`, the player must actually be connected to the server; use `identityId` otherwise.

**Example**
Ban for an hour

```
#ban create 123456789 3600
```

Ban forever

```
#ban create 123456789 0
```

Ban with a reason

```
#ban create 123456789 86400 teamkilling
```

#### remove

Remove a ban entry linked to the provided identity, allowing the player to rejoin.

**Syntax**

```
#ban remove <identityId>
```

**Example**

```
#ban remove 123456789
```

#### list

List current bans in a format `BanID ; Player UID ; Duration`.

**Important:**
- If the *page* parameter is not provided then the first page is shown.
- RCON can show 25 bans per page while game only 10.

**Syntax**

```
#ban list <page (optional)>
```

**Example**

```
#ban list
#ban list 2
```

### id

Gives the local player's id.
**Example**

```
#id
```

### players

Lists session's players and their playerId.
**Example**

```
#players
```

## Custom RCON Commands

**Note:** Custom RCON commands are BI-specific and are not part of RCON standards. They start with `@` to differenciate them from standard commands.

### logout

Arma Reforger 1.2.1

De-authenticate RCON client on server server side which immediately frees the connection for new RCON client.
If the RCON client is killed without issuing this command, it is de-authenticated automatically after 45s due to connection timeout.
