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