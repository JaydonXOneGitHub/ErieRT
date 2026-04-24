---@class ErieRT
--- The interface to directly talk to the core.
ErieRT = {}

--- Holds the args passed from the command line.
---@type { [integer]: string }
ErieRT.args = {}


---@param msTime integer
---@param callback fun()
---@param loopAmount integer Set to 0 for infinite loops.
--- Runs a callback at a set interval for a set amount of times without pausing execution.
function ErieRT.setTimeout(msTime, callback, loopAmount) end

---@param msTime integer
--- Sleeps execution for amount of time specified in msTime.
function ErieRT.sleep(msTime) end

---@param path string
---@return any
--- The de facto way to load ErieRT scripts.
function ErieRT.load(path) end

--- Detects whether ErieRT is running as an app.
--- Returns true if, as an example, `eriert`/`eriert.exe` is running with `eriert.ertpk` in the folder.
--- Otherwise returns false.
---@return boolean
function ErieRT.isApp()
    return false
end

--- Activates the Lua VM garbage collector.
function ErieRT.gcCollect() end


ErieRT.WebRequest = WebRequest
ErieRT.BlockingWebRequest = BlockingWebRequest
ErieRT.Promise = Promise
ErieRT.Server = Server
ErieRT.Listener = Listener
ErieRT.FileContents = FileContents
ErieRT.Match = Match
ErieRT.WebRequestMetadata = WebRequestMetadata
ErieRT.Result = Result
ErieRT.Json = Json
ErieRT.Reloadable = Reloadable
ErieRT.Redirect = Redirect