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