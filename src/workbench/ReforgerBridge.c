// The handler script of the reforger Claude Code plugin. The Workbench bridge copies this file
// into the open mod project and reloads the scripts. The bridge calls it through the Workbench
// NET API with "APIFunc": "ReforgerBridge" and an "action".
//
// Do not change this copy in the mod project: the bridge writes it again when its version is
// not the version of the server.

class ReforgerBridgeRequest : JsonApiStruct
{
	string action;
	string id;

	void ReforgerBridgeRequest()
	{
		RegV("action");
		RegV("id");
	}
}

class ReforgerBridgeResponse : JsonApiStruct
{
	string version;
	string action;
	string error;

	// The answer to "reference_progress".
	string state;
	int percent;
	string name;
	string mod_version;
	string license;
	string reason;

	void ReforgerBridgeResponse()
	{
		RegV("version");
		RegV("action");
		RegV("error");
		RegV("state");
		RegV("percent");
		RegV("name");
		RegV("mod_version");
		RegV("license");
		RegV("reason");
	}
}

// The Workshop search for one ID: "search": "<ID>" finds exactly that mod.
class ReforgerBridgeSearch : PageParams
{
	string m_Id;

	override void OnPack()
	{
		StoreString("search", m_Id);
	}
}

// One download of a Workshop mod by its ID. The Workshop calls end later in callbacks, so the
// state stays in Workbench memory between calls of the bridge.
class ReforgerBridgeDownload
{
	string m_Id;
	string m_State = "searching";
	string m_Reason;
	string m_Version;
	ref WorkshopItem m_Item;
	ref ReforgerBridgeSearch m_Search;
	ref BackendCallback m_SearchCallback;
	ref BackendCallback m_DetailCallback;
	ref BackendCallback m_DownloadCallback;

	void ReforgerBridgeDownload(string id)
	{
		m_Id = id;
	}

	// Step 1: search the Workshop for the ID.
	void Search()
	{
		WorkshopApi workshop = ReforgerBridgeDownloads.Workshop();
		if (!workshop)
		{
			Fail("Workbench has no Workshop connection");
			return;
		}
		m_Search = new ReforgerBridgeSearch();
		m_Search.m_Id = m_Id;
		m_Search.limit = 10;
		m_Search.offset = 0;
		m_SearchCallback = new BackendCallback();
		m_SearchCallback.SetOnSuccess(OnSearched);
		m_SearchCallback.SetOnError(OnSearchFailed);
		workshop.SetPageSize(10);
		workshop.RequestPage(m_SearchCallback, m_Search, true);
	}

	// Step 2: keep the item with this exact ID and ask for its details.
	void OnSearched(BackendCallback callback)
	{
		WorkshopApi workshop = ReforgerBridgeDownloads.Workshop();
		if (workshop)
		{
			array<WorkshopItem> items = {};
			workshop.GetPageItems(items);
			foreach (WorkshopItem item : items)
			{
				if (item && ReforgerBridgeDownloads.SameId(item.Id(), m_Id))
				{
					m_Item = item;
					break;
				}
			}
		}
		ReforgerBridgeDownloads.SearchEnded();
		if (!m_Item)
		{
			Fail("the Workshop has no mod with this ID");
			return;
		}
		m_DetailCallback = new BackendCallback();
		m_DetailCallback.SetOnSuccess(OnDetail);
		m_DetailCallback.SetOnError(OnError);
		m_Item.AskDetail(m_DetailCallback);
	}

	void OnSearchFailed(BackendCallback callback)
	{
		ReforgerBridgeDownloads.SearchEnded();
		OnError(callback);
	}

	// Step 3: download the latest revision.
	void OnDetail(BackendCallback callback)
	{
		Revision revision = m_Item.GetLatestRevision();
		if (!revision)
		{
			Fail("the Workshop gave no version of this mod");
			return;
		}
		m_Version = revision.GetVersion();
		m_State = "downloading";
		m_DownloadCallback = new BackendCallback();
		m_DownloadCallback.SetOnSuccess(OnDownloaded);
		m_DownloadCallback.SetOnError(OnError);
		m_Item.Download(m_DownloadCallback, revision);
	}

	void OnDownloaded(BackendCallback callback)
	{
		m_State = "complete";
	}

	void OnError(BackendCallback callback)
	{
		string message = callback.GetErrorMessage();
		if (message.IsEmpty())
			message = "error code " + callback.GetApiCode().ToString();
		Fail("the Workshop failed: " + message);
	}

	void Fail(string reason)
	{
		m_State = "failed";
		m_Reason = reason;
	}

	bool IsFailed()
	{
		return m_State == "failed";
	}

	bool IsRunning()
	{
		return m_State == "searching" || m_State == "downloading";
	}

	void Answer(ReforgerBridgeResponse response)
	{
		response.state = m_State;
		response.reason = m_Reason;
		if (m_State == "downloading" && m_Item)
		{
			float progress = m_Item.GetProgress();
			response.percent = Math.Floor(Math.Clamp(progress, 0, 1) * 100);
		}
		if (m_State == "complete")
		{
			response.percent = 100;
			response.name = m_Item.Name();
			response.mod_version = m_Version;
			response.license = m_Item.License();
		}
	}
}

// All downloads that the bridge started since Workbench loaded this script. One Workshop search
// runs at a time, because the page items of the Workshop belong to the last search.
class ReforgerBridgeDownloads
{
	static ref map<string, ref ReforgerBridgeDownload> s_Downloads = new map<string, ref ReforgerBridgeDownload>();
	static ref array<ref ReforgerBridgeDownload> s_Waiting = {};
	static bool s_Searching;

	static WorkshopApi Workshop()
	{
		BackendApi backend = GetGame().GetBackendApi();
		if (!backend)
			return null;
		return backend.GetWorkshop();
	}

	static bool SameId(string a, string b)
	{
		string upperA = a;
		string upperB = b;
		upperA.ToUpper();
		upperB.ToUpper();
		return upperA == upperB;
	}

	static string Key(string id)
	{
		string key = id;
		key.ToUpper();
		return key;
	}

	// Starts the download of `id`. When a download of `id` searches or downloads, it starts nothing.
	// A complete or a failed download starts again, because its mod folder can be gone.
	static void Start(string id)
	{
		string key = Key(id);
		ReforgerBridgeDownload download = s_Downloads.Get(key);
		if (download && download.IsRunning())
			return;
		download = new ReforgerBridgeDownload(key);
		s_Downloads.Set(key, download);
		s_Waiting.Insert(download);
		SearchNext();
	}

	static void SearchEnded()
	{
		s_Searching = false;
		SearchNext();
	}

	static void SearchNext()
	{
		if (s_Searching || s_Waiting.IsEmpty())
			return;
		ReforgerBridgeDownload download = s_Waiting.Get(0);
		s_Waiting.RemoveOrdered(0);
		s_Searching = true;
		download.Search();
		if (download.IsFailed())
			s_Searching = false;
	}

	static void Progress(string id, ReforgerBridgeResponse response)
	{
		ReforgerBridgeDownload download = s_Downloads.Get(Key(id));
		if (!download)
		{
			response.state = "failed";
			response.reason = "Workbench started no download of this ID since it loaded the scripts";
			return;
		}
		download.Answer(response);
	}
}

class ReforgerBridge : NetApiHandler
{
	// The version of the server that ships this file. A test checks that it is the version in
	// Cargo.toml.
	static const string VERSION = "0.2.0";

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
		if (req.action == "ping")
			return response;
		if (req.action == "reference_download" || req.action == "reference_progress")
		{
			if (req.id.IsEmpty())
			{
				response.error = "The action \"" + req.action + "\" needs the parameter \"id\".";
				return response;
			}
			if (req.action == "reference_download")
				ReforgerBridgeDownloads.Start(req.id);
			else
				ReforgerBridgeDownloads.Progress(req.id, response);
			return response;
		}
		response.error = "Unknown action \"" + req.action + "\". Known actions: ping, reference_download, reference_progress.";
		return response;
	}
}
