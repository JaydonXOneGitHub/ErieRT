-- ErieRT is the default namespace for types built into ErieRT.
-- Extensions may have different namespaces.

local Listener = ErieRT.Listener
local Server = ErieRT.Server

local listener = Listener.new("0.0.0.0:8000")
local server = Server.new(listener)

server:get("/", function(req, res)
    print("Got request!")
    res:send({ code = 200, message = "Hello, World!" })
end)

server:run()