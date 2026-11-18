// The handler script of the reforger Claude Code plugin. The Workbench bridge copies this file
// into the open mod project and reloads the scripts. The bridge calls it through the Workbench
// NET API with "APIFunc": "ReforgerBridge" and an "action".
//
// Do not change this copy in the mod project: the bridge writes it again when its version is
// not the version of the server.

class ReforgerBridgeRequest : JsonApiStruct
{
	string action;

	void ReforgerBridgeRequest()
	{
		RegV("action");
	}
}

class ReforgerBridgeResponse : JsonApiStruct
{
	string version;
	string action;
	string error;

	void ReforgerBridgeResponse()
	{
		RegV("version");
		RegV("action");
		RegV("error");
	}
}

class ReforgerBridge : NetApiHandler
{
	// The version of the server that ships this file. A test checks that it is the version in
	// Cargo.toml.
	static const string VERSION = "0.1.0";

	override JsonApiStruct GetRequest()
	{
		return new ReforgerBridgeRequest();
	}

	override JsonApiStruct GetResponse(JsonApiStruct request)
	{
		ReforgerBridgeRequest req = ReforgerBridgeRequest.Cast(request);
		ReforgerBridgeResponse response = new ReforgerBridgeResponse();
		response.version = VERSION;
		response.action = req.action;
		if (req.action != "ping")
			response.error = "Unknown action \"" + req.action + "\". Known actions: ping.";
		return response;
	}
}
