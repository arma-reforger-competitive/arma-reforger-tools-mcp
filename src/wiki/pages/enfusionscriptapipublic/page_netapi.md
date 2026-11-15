---
title: Workbench NET API
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_NetApi.html
---

## About

Enfusion Workbench NET API provide network protocol for communication between Workbench and external tools. External tool connects to the Workbench NET API endpoint using TCP/IP socket and sends / recieves data according to the Workbench protocol. Sending and receiving integer and string (UTF-8 encoded) values are supported and each data transaction requires a new TCP/IP connection. Integer is represented by 4-byte little endian and string is in the Pascal-style string format (first 4-byte integer information repsesents the string length).

Communication is always initiated from the client side. When all the data are passed (based on the protocol) the Workbench sends a response.

## Protocol

The protocol is proprietary and very simple.

### Request

Whole message consist of four parts as visualized on the following diagram.

```
┌──────────────┬─────────────────┬──────────────────┬──────────────────────────────────┐
│ Protocol Ver.│   Client ID     │   Content Type   │       Payload                    │
│              │                 │                  │                                  │
│  (Integer)   │   (String)      │    (String)      │       (String)                   │
└──────────────┴─────────────────┴──────────────────┴──────────────────────────────────┘
```

- **Protocol Ver.** - integer values of the protocol version, currently only version 1 is supported.
- **Client ID** - string representation of the client making the request, for example PythonClient.
- **Content Type** - the format of the payload, currently only JsonRPC is supported.
- **Payload** - JSON message encoded as UTF-8 string.

#### Payload format

The JsonRPC payload format is a JSON encoded as a string, defining what API endpoint to call by its name (APIFunc) as well as parameter names and their respective values.

```
{
    "APIFunc": "Enforce Script Class implementing NetApiHandler or built-in function name",
    "PARAM1" : "Value1",
    "PARAM2" : 0,
    "PARAM3": {
        "A": true,
        "B": "false"
  },
}
```

Following list contains all the built-in function you can use in the APIFunc.

- **OpenResource**
  - required parameters for input:
```
{
    "APIFunc": "OpenResource",
    "ResourceName": "string", // name of a resource file to open in Workbench
}
```
  - response:
```
{
    "Opened": "bool", // result of the operation
}
```
- **BringModuleWindowToFront**
  - required parameters for input:
```
{
    "APIFunc": "BringModuleWindowToFront",
    "ModuleName": "string", // name of a workbench module
}
```
  - response:
```
{
    // none
}
```
- **IsWorkbenchRunning**
  - required parameters for input:
```
{
    "APIFunc": "IsWorkbenchRunning",
}
```
  - response:
```
{
    "IsRunning": "bool", // is workbench running
    "ScriptsCompiled": "bool", // are scripts compiled successfully
}
```
- **IsWorldEditorRunning**
  - required parameters for input:
```
{
    "APIFunc": "IsWorldEditorRunning",
}
```
  - response:
```
{
    "IsRunning": "bool", // is workbench running
    "ScriptsCompiled": "bool", // are scripts compiled successfully
}
```
- **ValidateScripts**
  - required parameters for input:
```
{
    "APIFunc": "ValidateScripts",
    "Configuration": "string", // script configuration to validate (see project settings, e.g. WORKBENCH, PC, PLAYSTATION, XBOX...)
}
```
  - response:
```
{
  // array of errors
  "Errors": [
    {
      "error": "Incompatible parameter 'b'", // text of error
      "file": "scripts/Game/game.c", // file where error occurs
      "fileAbs": "F:\\DATA\\scripts\\Game\\game.c", // optional, present just for scripts which are unpacked
      "addon": "MyAddon", // optional, present just for scripts which are unpacked
      "line": 147 // position in file where error occurs
    }
  ],
  // array of warninigs
  "Warnings": [
    {
      "error": "Variable 'a' is not used", // text of warning
      "file": "scripts/Game/game.c", // file where warning occurs
      "fileAbs": "F:\\DATA\\scripts\\Game\\game.c", // optional, present just for scripts which are unpacked
      "addon": "MyAddon", // optional, present just for scripts which are unpacked
      "line": 146 // position in file where warning occurs
    }
  ],
  "Success": false
}
```
- **ReloadScripts**
  - required parameters for input:
```
{
    "APIFunc": "ReloadScripts",
}
```
  - response:
```
{
    // none, error code indicates the result ("Ok" on success, "Compilation failed" on failure)
}
```
- **EvaluateScript**
  - required parameters for input:
```
{
    "APIFunc": "EvaluateScript",
    "Script": "string", // Enforce Script source code to evaluate, argument-less function declaration is expected as entry point
}
```
  - response:
```
{
    "ReturnValue": "string", // return value of the entry point function, compiler errors when the script contains invalid Enforce syntax, or an error message if execution fails
}
```

### Response

Message consists of two parts.

```
┌──────────────┬────────────────────────────────────┐
│ Error Code   │       Payload                      │
│              │                                    │
│  (String)    │       (String)                     │
└──────────────┴────────────────────────────────────┘
```

- **Error Code** - string representation of the error code.
- **Payload** - JSON message encoded as UTF-8 string.

#### Payload format

The JsonRPC payload format is a JSON encoded as a string, the format is specific for each APIFunc which was called in the request.

## Custom endpoint

Most of the time you will want or need to implement your own logic of what should be done and return from the Workbench to the caller. In order to do so, you need to implement few things in the Enforce Script and have such script running in a live instance of the Workbench.

1. Implement your own API endpoint by extending NetApiHandler.
1. Define your own request (JsonRPC format) by extending JsonApiStruct.
1. Define your own response by extending JsonApiStruct.

### Example script

```
class ExampleRequest : JsonApiStruct
{
    string input;
 
    void ExampleRequest()
    {
        RegV("input");
    }
}
 
class ExampleResponse : JsonApiStruct
{
    string output;
    
    void ExampleResponse()
    {
        RegV("output");
    }
}
 
// Class that is being called
class ExampleCall : NetApiHandler
{
    override JsonApiStruct GetRequest()
    {
        return new ExampleRequest();
    }
 
    override JsonApiStruct GetResponse(JsonApiStruct request)
    {
        ExampleRequest req = ExampleRequest.Cast(request);
        ExampleResponse response = new ExampleResponse();
        
        Print(req.input);
        response.output = "Hello from Workbench";
 
        return response;
    }
    
}
```
