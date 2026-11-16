---
title: Arma Reforger:Game Identity
url: https://community.bistudio.com/wiki/Arma_Reforger:Game_Identity
---

A **Game Identity** is a game-related identity that is attached to the person playing.

## Definitions

- **Game Identity**: a Game Identity is created for each game-platform combination on first network connection ([Steam](https://community.bistudio.com/wiki/Steam) or Xbox) - there can be multiple ones for one player using multiple platforms.
- **Bohemia Interactive Account**: a Bohemia Interactive Account is the general [https://www.bohemia.net](https://www.bohemia.net) Bohemia Interactive account - there can be only one per player.

A **Game Identity** can be linked to a **Bohemia Interactive account**.

**Important:**
- Two Game Identities (from two different platforms) linked to the same Bohemia Interactive count as one and the same person and can therefore not connect to the same server.
- It is **not** possible to link the same platform type multiple times to a Bohemia Interactive account (e.g only one PC Game Identity, one Xbox Game Identity).
- It is **not** possible for the end user to unlink identities from a Bohemia Interactive account.

Linking example:

| Step | Event |
| --- | --- |
| Play Arma Reforger on Xbox | A **Game Identity** is created on backend's side |
| Link the Bohemia Interactive account | The Bohemia Interactive account is linked to the Game Identity |
| Play Arma Reforger on PC | A **Game Identity** is created on backend's side, without BI account information |
| Link the Bohemia Interactive account | Bohemia Interactive account identical information is detected and the **Game Identities** are now related.<br>As of now, the Arma Reforger Steam Identity takes precedence over the Xbox one for [Workshop](https://community.bistudio.com/wiki/Arma_Reforger:Workshop) publishing purpose. |

## Scripting

The local player's Game Identity ID can be obtained (on clients only; dedicated servers do not have identities) using `BackendApi.GetLocalIdentityId()`.

A player's Game Identity's ID can be obtained **server-side** in script using `BackendApi.GetPlayerIdentityID(int iPlayerId)`.

**Note:** Bohemia Interactive Account ID and platformUID (SteamID, XboxUID) are **not** accessible by script.
