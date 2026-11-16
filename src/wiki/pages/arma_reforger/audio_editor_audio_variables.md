---
title: Arma Reforger:Audio Editor: Audio Variables
url: https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_Audio_Variables
---

The Audio Variable system allows it to process the properties of multiple playing sounds by storing and processing them into a "Variable".

This is done in the "Variable" Item Detail category of nodes that support feeding into the Variable system.

This processed value can then be used in other Audio Editor or Mixer projects as a signal output by utilising the **Variable Node**.

The system is Resource-based, meaning it utilizes special Config files (.conf) to store and read from multiple Variables.

## Variable Creation

A new Audio Variable config file can be created by opening the Audio Editor, then clicking on **File → New → Variable**. After naming file, it will be created but not open on its own.

When opening the Variable Config file, the actual Variables can be created by clicking on the "+" icon. Each Variable needs a **Source** and an **Operator**:

### Source Types

- **Volume:** Tracks the volume of all sounds that feed into the variable
- **Amplitude:** Tracks the calculated Amplitude of all sounds that feed into the variable
- **PlayTime:** Tracks the PlayTime of all sounds that feed into the variable. E.g: If sound B starts while sound A is playing
- **Constant:** Variable value will always output the specified static value
- **External:** Variable value is set via script/gamecode.

### Operator Types

- **Sum:** Variable outputs the current sum of the source
- **Max:** Variable value outputs the maximum collected value in the current frame
- **Min:** Variable value outputs the minimum of collected values in the current frame

## Variable Feeding

Variables are fed via **nodes** in the **Audio Editor**. If a node supports feeding into a Variable, the **"Variable" Category** can be found it its **Item Details**.
Here, the desired Variable Config file can be defined and, afterwards, which variable should be fed into..

### Compatible Nodes

- [Sound](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_Nodes#Sound)
- [Bus](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_Nodes#Bus)
- [Amplitude](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_Nodes#Amplitude)

## Variable Usage

The processed value of the Variables can be accessed by using the **Variable node**. It's available within both `.acp` and `.afm` (mixer) files.

In order to do so, a **Variable node** must be placed, then a **Variable Config file** and finally the actual **Variable** must be selected.

Within the Variable node, a **Curve** can be created that defines the relation between the inputted variable value and the output value.

This output can then be used like a regular signal output.

### Example

The VON_Direct variable directly influences the volume of the VEH 1 Bus in the Mixer.
The curve reads as follows: If the variable value is below 0.2, the output will be 1.
Between 0.2 and 0.7, the output interpolates down to 0.8 and remains there.

## Scripting

Variables of type **External** can be set *via* Script. All related methods belong to the `AudioSystem` class. The following methods are available:

```enforce
// return variable value
float AudioSystem.GetVariableByID(int ID, string filename)

// return variable value
float AudioSystem.GetVariableValue(string varName, string filename)

// return variable name
string AudioSystem.GetVariableNameByID(int ID, string filename)

// return variable ID
int AudioSystem.GetVariableIDByName(string varName, string filename)

// return true if the variable exists, false otherwise
bool AudioSystem.SetVariableByID(int ID, float value, string filename)

// return true if the variable exists, false otherwise
bool AudioSystem.SetVariableByName(string varName, float value, string filename)
```

**Note:** The filename string expects the full resource path of the variable config file.
For the vanilla Arma Reforger's `enfusion://ResourceManager/~ArmaReforger:Sounds/_SharedData/Variables/GlobalVariables.conf`, this would be "{A60F08955792B575}Sounds/_SharedData/Variables/GlobalVariables.conf".
