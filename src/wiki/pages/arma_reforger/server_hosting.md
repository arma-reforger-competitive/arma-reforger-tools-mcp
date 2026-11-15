---
title: Arma Reforger:Server Hosting
url: https://community.bistudio.com/wiki/Arma_Reforger:Server_Hosting
---

Server Hosting is the fact of hosting a game instance accessible over the network to other players.
There are two possible modes in Arma Reforger: **player-hosted** and **dedicated**.

**Important:**
For a hosted server to be visible from outside the local network (e.g from internet), [Port Forwarding](https://en.wikipedia.org/wiki/Port_forwarding) must be used.

Please refer to your router's user guide for assistance.

Main router brands's documentation links:
- [Cisco router](https://www.cisco.com/c/en/us/support/docs/smb/routers/cisco-rv-series-small-business-routers/smb5818-configure-port-forwarding-port-triggering-nat-on-rv34x-serie.html)
- [D-Link router](https://www.dlink.com/uk/en/support/faq/routers/mydlink-routers/dir-810l/how-do-i-configure-port-forwarding-on-my-router)
- [Netgear router](https://kb.netgear.com/24290/How-do-I-add-a-custom-port-forwarding-service-on-my-NETGEAR-router)
- [TP-Link router](https://www.tp-link.com/us/support/faq/1379/)

## Dedicated Server

A **Dedicated Server** is a server without any game instance launched; it purely processes game information and network synchronisation.

**Note:** stable is app id: **1874900** - https://steamdb.info/app/1874900/

experimental: **1890870** - https://steamdb.info/app/1890870/

### BattlEye

**Note:** See [BattlEye](http://www.battleye.com/)'s [documentation](https://www.battleye.com/support/documentation/) and [FAQ](https://www.battleye.com/support/faq/) - there is also a [BattlEye](https://community.bistudio.com/wiki/BattlEye) wiki article.

It is possible to modify BattlEye's RCon port and password by **adding** the following settings to `Arma Reforger\BattlEye\BEServer_x64.cfg`:

```
RConPort 5678
RConPassword myNewBEPassword
```

**Warning:** When editing Arma Reforger's BattlEye config, make sure to **append** new settings to it and not *erase* or even *edit* existing information.<br>
Missing information will have the kick message "Missing GameID/MasterPort server config settings" welcome (and eject) players, whereas modified values will prevent BattlEye from working.<br>
In the event of the file being already incorrectly edited, verify the game's files on [Steam](https://steampowered.com/) (see [Steam's tutorial](https://help.steampowered.com/en/faqs/view/0C48-FCBD-DA71-93EB)):
- delete `BEServer_x64.cfg`
- in Steam, right-click on Arma Reforger
- select "Properties"
- click on "Local Files"
- press "Verify integrity of game files"
Steam will now verify and repair modified files. This will not erase any personal files, game progress, controls and other configurations will remain unchanged.

### Startup Parameters

See [Arma Reforger:Startup Parameters#Hosting](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#Hosting) and the [Server Config](https://community.bistudio.com/wiki/Arma_Reforger:Server_Config) page for more information.

#### config

The Server exe uses the `-config` [startup parameter](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters) to target the configuration file.

```
ArmaReforgerServer.exe -config ".\Configs\Campaign_SWCoast.json"
```

In above example, Campaign_SWCoast.json is expected to be locted in Configs folder next to the exe.

#### maxFPS

**Warning:** As of **0.9.8** it is **heavily recommended** to use [this](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#maxFPS) [startup parameter](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters), set to a value in the 60..120 range; otherwise, the server can try to use all the available resources!

```
ArmaReforgerServer.exe -maxFPS 60
```

#### server

This parameter instructs the executable to launch **local** server and **load selected world**. When this parameter is used, config is ignored. Server parameter can be combined with [addons](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#addons) and [addonsDir](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#addonsDir) parameters to start a server with local mods, which can be useful when testing addon before uploading it Workshop.

```
ArmaReforgerServer.exe -server "worlds/MP/MPTest.ent" -addonsDir "C:\MyModsDir" -addons MyCustomMod
```

#### Others

The below [Arma Reforger:Startup Parameters](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters) are optional but may prove useful upon some cases:
- [logStats](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#logStats) - allows to log server's FPS every x milliseconds
- [logLevel](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#logLevel) - sets the log detail level
- [listScenarios](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#listScenarios) - logs available scenario `.conf` file paths on startup

### Configuration File

See [Arma Reforger:Server Config](https://community.bistudio.com/wiki/Arma_Reforger:Server_Config).

## Player-Hosted Server

Arma Reforger 0.9.7

Also known as **Listen Server**, a **Player-Hosted Server** is a server also hosting a local player.
Such server is started from within the game, in `Multiplayer > Host tab > Host new server`.

**PC:** Player-hosting is only available on PC.

### Settings

The settings are all self-explanatory and Dedicated Server can be used.

#### Scenario Selection

**Scenario** and **Source** are two related fields:
- **Scenario** is the list of all available scenarios
- **Source** is a read-only field telling from which mod (or Arma Reforger) the selected Scenario is.

#### Crossplay

This option allows console players to join - see supportedGameClientTypes.

### Mods

This tab allows to enable or disable local mods to make them available to the hosted game (or not). The [Arma Reforger:Workshop](https://community.bistudio.com/wiki/Arma_Reforger:Workshop) is accessible from here.

## Linux Server

The game server will by default use Docker container's IP for server browser registration and client connection which will cause failure during client connection attempt.<br>
To avoid it use:
- Run the "ipconfig" command in cmd to list the local IPs
- "IP Connect" option in the server browser and insert one of the server's local IPs
- Custom server config (.json file) with "gameHostRegisterBindAddress" and "gameHostRegisterPort" parameters set to one of the local IP:Port combinations
Example:

```
-config "./My_Config.json"
ClientConnectAddress 192.168.39.98
```

### SteamCMD Setup

**Important:** This tutorial has been tested on **Ubuntu**.

1. Install SteamCMD - for the latest documentation, see https://developer.valvesoftware.com/wiki/SteamCMD
   1. Download and install it (link on the [SteamCMD](https://developer.valvesoftware.com/wiki/SteamCMD) page) - it will auto-update to the latest version
   1. Set the install path with the `force_install_dir` command (otherwise the default location will be used, `home/<username>/.steam/steam/steamcmd`)
      1. You can name the folder *arma-reforger, armaR, armarserver, armarexpserver* and etc
   1. Login as anonymous - type in `login anonymous`
1. Download and install the server `app_update 1874900`
   1. *1890870* for experimental version
1. Quit SteamCMD `quit`

**Note:** To run Arma Reforger's server, run `./ArmaReforgerServer` in the installation directory.

#### Example Script

**update_armar_ds.txt**
Based on the [SteamCMD](https://developer.valvesoftware.com/wiki/SteamCMD) page's example

```bash
// update_armar_ds.txt
//
@ShutdownOnFailedCommand 1
@NoPromptForPassword 1
force_install_dir ../armar_ds
login anonymous
app_update 1874900 validate
quit
```

**Execution**

```bash
steamcmd +runscript update_armar_ds.txt
```

#### With Bash Scripts

1. Install SteamCMD
```bash
sudo apt install steamcmd
```

1. create a directory lets say `armaR` somewhere, lets say `/home/<username>/armaR`
1. Create in it a steam shell script that will install reforger server `steamShell.txt` with something like
```bash
@ShutdownOnFailedCommand 1
login anonymous
// Stable ID: 1874900 EXP: 1890870
app_update 1874900 validate
quit
```

1. then create bash script that will install/update game `steamInstall.sh` with:
```bash
/usr/games/steamcmd +force_install_dir /home/<username>/armaR +runscript /home/<username>/armaR/steamShell.txt
```

1. after that, server startup script `start.sh` with:
```bash
// Install or Update game
./steamInstall.sh
// Start server
./ArmaReforgerServer -config /home/<username>/.config/ArmaReforgerServer/config.json -profile /home/<username>/.config/ArmaReforgerServer -maxFPS 60
```

1. So the actual file tree will look something like this:
```bash
/home/<username>/armaR
				steamShell.txt
				steamInstall.sh
				start.sh
```

1. Then create actual `config.json` at `/home/<username>/.config/ArmaReforgerServer/config.json`, create folders if they don't exist
1. Run `start.sh` to install the server and start it.

Console logs etc can be found in above mentioned profile folder, addons will also install there by default, you can again change that.

You can also change profile/config directories as desired.
**Note:** You can then create new service for systemctl to run start.sh and the thing will run in the background and auto restart after crashes etc

### LinuxGSM

The [Arma LinuxGSM Tool](https://linuxgsm.com/servers/armarserver/) can also be used.

1. It can monitor the game server by checking that the proccess is running and querying it. Should the server go offline LinuxGSM can restart the server and send you an alert. You can use cronjobs to setup monitoring.
1. Update checks for any server updates and applies them. The server will update and restart only if required. **This also needs to be set up on a schedule.**

**Note:** For server's experimental version, add `appid="1890870"` to your `./lgsm/config-lgsm/armarserver/armarserver.cfg` and validate files by `./armarserver v`.

### Docker Setup

**Important:** This Docker configuration is adapted to **Ubuntu**. Other distributions such as Fedora or Arch Linux may store their certificates at the following location: `/etc/pki/ca-trust/` - be sure to edit the configuration accordingly.

**Note:** A community Docker image is available on GitHub: https://github.com/acemod/docker-reforger

1. Install the latest Docker:
   1. [Download](https://docs.docker.com/desktop/windows/install/) and install Docker
   1. [Enable Hyper-V in Windows](https://docs.microsoft.com/en-us/virtualization/hyper-v-on-windows/quick-start/enable-hyper-v) if it is not already
   1. Assign HW resources in `Docker/Settings/Resources/Advanced`:
      1. CPU: 4 cores
      1. Memory: 6 GB
1. Download Ubuntu 18.04 image *via* batch or powershell cmd: `docker pull ubuntu:18.04`
1. Run Ubuntu image:
   1. mount volume with server data
   1. expose client connection UDP port
   1. Example: `docker container run -t -d -p 2001:2001/udp -v D:\server_data\linux_packed:/home/packed --name ubuntu_test ubuntu:18.04`
1. Connect to bash console: `docker exec -it ubuntu_test /bin/bash`
1. Install necessary software:
   1. libcurl4 - required by server app
   1. net-tools - for debug purposes (enables ifconfig etc)
   1. Installation:
      1. apt-get update
      1. apt-get install libcurl4
      1. apt-get install net-tools
      1. apt-get install libssl1.1
1. Create logs directory, e.g: `mkdir /home/profile`
1. Run server:
   1. server executable needs proper execution/access rights:
      1. cd server_root_folder
      1. chmod +x ArmaReforgerServer
   1. Example: `./ArmaReforgerServer -gproj ./addons/data/ArmaReforger.gproj -config Configs/ServerConfig/Campaign.json -backendlog -nothrow -profile /home/profile`

**Note:** If you are getting curl errors `Curl error=Failed writing received data to disk/application` and `error curl_easy_perform, code: 23` when downloading large mods in a docker container, you might need to set -addonTempDir ./tmp or some other location where the container has write permissions.
