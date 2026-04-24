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