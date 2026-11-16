---
title: Arma Reforger:Audio Editor
url: https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor
---

The **Audio Editor** is the tool that defines how sounds behave in-game. It allows the creation of graphical signal chains that define the logic for how sounds are triggered based on in-game events, and how they respond to changes in various game parameters.

It allows listening to how sounds behave inside the editor with the built-in tools.

Projects created in the audio editor are saved as files with an .acp extension; outside of the editor, they can later be used to control the sound that in-game entities make by applying the file to the entity's respective SoundComponent inside the World Editor.

## User Interface

The Audio Editor shows the following interface on first launch:

### Panels

The audio editor UI is composed of the following panels:

1. **Design canvas:** This is the area where the user can create audio signal chains by placing nodes and creating connections between them.
1. **Nodes palette:** This section contains the names of all the types of nodes that can be placed on the design canvas. Clicking on a name creates a new instance of the corresponding node on the canvas.
1. **Item detail:** This section contains the details/parameters of the currently selected object. By default, the details of the current project are displayed.
1. **Master level monitor:** The levels of the currently-playing sound are displayed here.
1. **Listener setup:** This section gives a visual depiction of the virtual audio source (emitter) in 3D space from the listener's perspective. The spatial relation between the listener and emitter can be adjusted here.
1. **Playlist:** Displays a list of previously-played sounds from the current session.
1. **Log console:** Displays relevant info about the actions performed in the current session.
1. **Output tracker:** Displays the output waveform of the current and previously played sounds over time.
1. **Item explorer:** Displays the nodes present in the current project, grouped by type.
1. **Resource browser:** Displays the location of the open project in the context of the resource database hierarchy.

**Note:** Panels are able to be un-docked into a separate window and also hidden from view.
To make a closed panel visible again, right-click on the background of the main window for a list of panels and their visibility status.

For a full description of the usage of each panel in the context of designing sounds, see the Getting Started Tutorial.

### Node Groups

Groups in the Enfusion Audio Editor are a great way to keep your audio files organised and clean.
They also improve workflow by moving all included nodes relative to the group's location.

#### Group Creation

- Select multiple nodes.
  - Hold [Shift+LMB] drag to make a selection region.
  - Hold [Ctrl] and click on every node you want included in the group.
- Right click on one of the nodes included in your selection.
- Select "Group".

#### Add/Remove Node from Group

Nodes can easily be removed from a group by either right-clicking them and selecting "remove from group" or holding [Alt] while dragging them out of the group.

#### Group Options

| Option | Functionality |
| --- | --- |
| Ungroup | Removes all nodes from the group and destroys the group. |
| Select All | Selects all nodes included in the group. |
| Lock | Locks the group so that it can not be modified or moved. |
| Color | Opens a dialog where the color of the group can be adjusted. |

### Opus Ogg Import

Arma Reforger 1.6.0

Arma Reforger received support for the [Opus](https://en.wikipedia.org/wiki/Opus_(audio_format)) audio format in v1.6.0.
Opus is a lossy format which trades lower quality with a significantly decreased file size and slightly increased decoding time;
it is therefore primarily recommended for music tracks.

Please note that you can **not** just drop an Ogg file into your project.
Instead, the original `.wav` file(s) must be converted into `.snd` (Sound) file(s), a new container format which supports both wav and ogg.

#### Ogg Conversion

- Click on **Tools -> File Converter**
- **Drag & Drop** the Wav files you want to convert into the conversion panel
- Select OPUS as the file format and click Convert. This will create Snd (not Ogg!) files with the same names in the same folders as the original files
- Replace the wav files with the snd files in your banks
**Important:** When building/publishing an addon the original wav file will be excluded and only the snd file will be built,<br>which is why it is **very** important to make sure you replace all the original wav file references in your banks.

## Keyboard Shortcuts

The Enfusion Audio Editor has many shortcuts and hotkeys that are specifically designed to improve workflow and boost productivity of Audio Designers.<br>
This section gives an overview of most of the available shortcuts and hotkeys in the Audio Editor and its Signal Editor.

| File |  |
| --- | --- |
| New Project | [Ctrl+N] |
| New Signal | [Ctrl+Shift+N] |
| Open | [Ctrl+Shift+O] |
| Close | [Ctrl+Shift+C] |
| Save | [Ctrl+S] |
| Save As | [Ctrl+Shift+A] |
| Save All | [Ctrl+Shift+S] |
| Exit | [Alt+F4] |

| Window |  |
| --- | --- |
| Item Explorer | [Shift+F1] |
| Resource Browser | [Shift+F2] |
| Item Detail | [Shift+F3] |
| Master Level Monitor | [Shift+F4] |
| Listener Setup | [Shift+F6] |
| Playlist | [Shift+F7] |
| Output Tracker (Master) | [Shift+F8] |
| Log Console | [Shift+F9] |
| Signals Simulation | [Shift+F10] |

| Edit |  |
| --- | --- |
| Undo | [Ctrl+Z] |
| Redo | [Ctrl+Y] |
| Copy | [Ctrl+C] |
| Cut | [Ctrl+X] |
| Paste | [Ctrl+V] |

| Groups |  |
| --- | --- |
| *Create Group* | [Ctrl+G] |
| Add Node | [Alt+LMB] *(Drag & drop)* |
| Remove Node | [Alt+LMB] *(Click)* |

| Debug |  |
| --- | --- |
| Toggle Debug | [F5] |

| Listener Setup |  |
| --- | --- |
| Move Tool | [W]/[A]/[S]/[D] |
| Front View | [1] |
| Right View | [3] |
| Top View | [7] |

| Scene |  |
| --- | --- |
| Pan | [LMB] *(Drag)* |
| Zoom In/Out | [MMB] *(Scroll Wheel)* |
| Region Select | [Shift+LMB] *(Drag)* |
| Multiselect Nodes | [Ctrl+LMB] *(Click)* |

| Nodes |  |
| --- | --- |
| Play Selected Node | [Space] |
| Start Quick Connection<br>*(on port)* | [Alt+LMB] *(Click)* |
| End Quick Connection<br>*(on port)* | [Alt+LMB] *(Click)* |
| Node Dialog | [MMB] *(Viewport Click)* |
| Move nodes only horizontally/vertically | [Shift+LMB] *(Drag)* |

| Connections |  |
| --- | --- |
| Duplicate Connection | [Shift+LMB] *(Drag)* |
| Reconnect Connection | [Ctrl+LMB] *(Drag)* |

### Tips and Tricks

**Automatic constant node variables**
Drag and dropping a signal file on top of a **constants** node will automatically rename and link the constants node's ports with that signal.<br>
Using [Ctrl] *replaces* existing connections.

**Audio amplitude representation inside Enfusion**
Amplitude is represented as 32bit floating point inside the editor, meaning 1528 dB of dynamic range. So there is barely any chance of clipping audio inside the engine. Simple example: Bank volume = 12dB, SOUND event volume = -12dB, no clipping.

The output tracker & master meter will show clipping if amplitude goes above 1 (0dbFS) but this does not mean that user will hear clipped audio necessarily. User will only clipped/distorted audio, if amplitude is greater than 0dbFS at the final master output stage or if there are DSPs such as compressors & limiters in the signal chain which might react weirdly to unnecessarily high amplitudes. You can technically create a square wave synthesizer by using sine wave generator nodes with enormous amplitude values and using a limiter that has very short attack & release times in the sound's DSP chain :)

**Shortcut tricks**

## Signal Editor

The Signal Editor is a sub-component of the Audio Editor - BIKI, where the user can edit the internal configurations of Signal nodes. Using the available node types in the signal editor, the user can define the input/output behavior of the Signal resource opened in the editor, transforming one or more inputs into one or more outputs.

### User Interface

The Signal Editor UI is identical to that of the parent Audio Editor, save for the types of nodes available in the **Nodes palette**.

## See Also

- [Arma Reforger:Audio Editor: Getting Started Tutorial](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_Getting_Started_Tutorial)
- [Arma Reforger:Audio Editor: Nodes](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_Nodes)
- [Arma Reforger:Audio Editor: DSP Nodes](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_DSP_Nodes)
- [Arma Reforger:Audio Editor: Signal Editor: Nodes](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_Signal_Editor:_Nodes)
