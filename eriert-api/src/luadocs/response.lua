---@class Response
--- The class passed into `Server`'s methods to send a response to HTTP requests.
Response = {}

---@alias MessageResponse { code: integer, message: string }
---@alias JsonResponse { json: string }

---@param value FileContents | Redirect | MessageResponse | JsonResponse
---@see FileContents
--- Places value into `Response.`
function Response:send(value) end