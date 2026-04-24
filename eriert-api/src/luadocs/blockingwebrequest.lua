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