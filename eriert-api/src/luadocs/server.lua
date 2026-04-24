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