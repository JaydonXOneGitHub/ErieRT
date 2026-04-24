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