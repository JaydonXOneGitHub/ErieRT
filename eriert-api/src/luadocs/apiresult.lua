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