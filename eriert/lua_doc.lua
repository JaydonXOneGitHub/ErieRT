---@class WebRequest
--- The interface to make web requests from Lua scripting.
--- For the blocking version, see `BlockingWebRequest`.
WebRequest = {}

---@param url string The URL to request
---@param metadata WebRequestMetadata Request metadata
---@param body string? Optional JSON body - defaults to "{}" internally
---@return WebRequest
---@see WebRequestMetadata
---@see Json.asString
--- Prepares a WebRequest instance. Body must be a JSON representation.
function WebRequest.make(url, metadata, body)
    return {}
end

---@param resolveCallback fun(resolve: any) Sets callback for once the data has been received.
function WebRequest:onResolve(resolveCallback) end

---@param errorCallback fun(error: any) Sets callback for when an error is returned.
function WebRequest:onError(errorCallback) end

--- Executes the WebRequest asynchronously.
function WebRequest:pull() end




---@class Match
--- The type for pattern matching.
Match = {}

---@return Match
--- Creates a new Match instance.
function Match.new()
    return {}
end

---@param key any
---@param value any
--- Adds key-value pair to Match. Note: tables passed in for key should have `compare` methods.
function Match:push(key, value) end

---@param fallback any
--- Sets the fallback value for when the pattern matching fails.
function Match:setFallback(fallback) end

---@param value any
---@return any
--- Executes the pattern-matching and returns proper value. Invalidates current statement.
function Match:exec(value) end




---@class Promise
--- The class used to handle work on another thread asynchronously.
Promise = {}

---@param callback fun()
---@return Promise
--- Initializes `Promise` with a specified callback.
function Promise.make(callback)
    return {}
end

---@param successCallback fun(success: any)
--- Sets the `Promise`'s success callback.
function Promise:onSuccess(successCallback) end

---@param errorCallback fun(error: any)
--- Sets the `Promise`'s error callback.
function Promise:onError(errorCallback) end

--- Invokes the `Promise`'s callback and either invokes `onSuccess` or `onError`.
function Promise:pull() end




---@alias WebRequestType "GET" | "PUT" | "POST" | "PATCH" | "HEAD" | "DELETE"

---@class WebRequestMetadata
---@field http_method WebRequestType Can be "GET", "PUT", "POST", "PATCH", "DELETE", or "HEAD".
---@field headers table? Represents the HTTP headers sent.
--- The type to pass into WebRequest.make or BlockingWebRequest.make
---@see WebRequest.make
---@see BlockingWebRequest.make
WebRequestMetadata = {}

WebRequestMetadata.http_method = "GET"
WebRequestMetadata.headers = nil




---@class Json
--- The class used to handle interop between Lua types and JSON.
Json = {}

---@param value any
---@return string
--- Converts a Lua value to JSON.
function Json.asString(value)
    return ""
end

---@param json string
---@return any
--- Converts JSON to a Lua value.
function Json.asValue(json) end

---@param str string
---@return string
--- Adds a set of quotation marks around the specified string for JSON.
function Json.jsonifyString(str)
    return ""
end




---@class BlockingWebRequest
--- The interface to make synchronous web requests from Lua scripting.
--- For the asynchronous version, see `BlockingWebRequest`.
BlockingWebRequest = {}

---@param url string The URL to request
---@param metadata WebRequestMetadata Request metadata
---@param body string? Optional JSON body - defaults to "{}" internally
---@return BlockingWebRequest
---@see WebRequestMetadata
---@see Json.asString
--- Prepares a WebRequest instance. Body must be a JSON representation.
function BlockingWebRequest.make(url, metadata, body)
    return {}
end

--- Executes the WebRequest asynchronously.
---@return Result
function BlockingWebRequest:pull()
    return {}
end




---@class Result
--- The class meant to represent an operation.
Result = {}

---@param value any
---@return Result
--- Constructs a successful `Result` object.
function Result.ok(value)
    return {}
end

---@param error any
---@return Result
--- Constructs a failure `Result` object.
function Result.error(error)
    return {}
end

---@return boolean
--- Returns whether or not it was constructed with the `ok` function.
function Result:isOk()
    return false
end

---@return boolean
--- Returns whether or not it was constructed with the `error` function.
function Result:isError()
    return false
end

---@return any
--- Returns the ok value, or `nil`.
function Result:getValue() end

---@return any
--- Returns the error value, or `nil`.
function Result:getError() end




---@class Listener
--- The class meant to house an HTTP listener before being passed to `Server`.
Listener = {}

---@param address string
---@return Listener
--- Tries to initiate a `Listener` object from a binding IP address.
function Listener.new(address)
    return {}
end




---@class Server
--- The class used for the core server.
Server = {}

--- Creates the server.
---@param listener Listener
---@return Server
function Server.new(listener)
    return {}
end

---@alias Request { body?: {}, headers: { [string]: string }, query?: {}, path?: string }

--- Adds a "GET" endpoint.
---@param endpoint string
---@param callback fun(req: Request, res: Response) The callback used for this endpoint
function Server:get(endpoint, callback) end

--- Adds a "PUT" endpoint.
---@param endpoint string
---@param callback fun(req: Request, res: Response) The callback used for this endpoint
function Server:put(endpoint, callback) end

--- Adds a "PATCH" endpoint.
---@param endpoint string
---@param callback fun(req: Request, res: Response) The callback used for this endpoint
function Server:patch(endpoint, callback) end

--- Adds a "POST" endpoint.
---@param endpoint string
---@param callback fun(req: Request, res: Response) The callback used for this endpoint
function Server:post(endpoint, callback) end

--- Adds a "HEAD" endpoint.
---@param endpoint string
---@param callback fun(req: Request, res: Response) The callback used for this endpoint
function Server:head(endpoint, callback) end

--- Adds a "DELETE" endpoint.
---@param endpoint string
---@param callback fun(req: Request, res: Response) The callback used for this endpoint
function Server:delete(endpoint, callback) end

--- Runs the server at the port specified in the `Listener` passed into `Server.new`.
---@see Listener
---@see Server.new
function Server:run() end




---@class StateType
---@field Stateless number
---@field Body number
---@field Query number
---@field BodyAndQuery number
--- An enum which represents which states `Server` takes in.
StateType = {
    Stateless = 0,
    Body = 1,
    Query = 2,
    BodyAndQuery = 3,
}




---@class FileContents
--- The class meant to represent many file types. Used mainly in Rust-side code.
FileContents = {}

---@param path string
---@return FileContents
--- Tries to read a file and creates a FileContents object.
function FileContents.read(path)
    return {}
end

---@param path string
---@return FileContents
--- Tries to read a file and creates a FileContents object. Only uses `FileContents::Plain` internally.
function FileContents.readAsString(path)
    return {}
end

---@param path string
---@return FileContents
--- Tries to read a file and creates a FileContents object. Only uses `FileContents::Bytes` internally.
function FileContents.readAsBytes(path)
    return {}
end




---@class Response
--- The class passed into `Server`'s methods to send a response to HTTP requests.
Response = {}

---@alias MessageResponse { code: integer, message: string }
---@alias JsonResponse { json: string }

---@param value FileContents | Redirect | MessageResponse | JsonResponse
---@see FileContents
--- Places value into `Response.`
function Response:send(value) end




---@class Reloadable
--- A stateful way to reload project scripts
Reloadable = {}

--- Loads the Lua script path.
---@param path string
---@return Reloadable
function Reloadable.load(path)
    return {}
end

--- Runs the script and returns the value returned in the script, if any.
---@return any
function Reloadable:reload() end




---@class Redirect
--- The class used in `Response` to make a redirect over HTTP.
---@see Response.send
Redirect = {}

---Makes a Redirect with `303 See Other`.
---@param uri string
---@return Redirect
function Redirect.to(uri)
    return {}
end

---Makes a Redirect with `307 Temporary Redirect`.
---@param uri string
---@return Redirect
function Redirect.temporary(uri)
    return {}
end

---Makes a Redirect with `308 Permanent Redirect`.
---@param uri string
---@return Redirect
function Redirect.permanent(uri)
    return {}
end




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