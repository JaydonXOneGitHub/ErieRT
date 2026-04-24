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