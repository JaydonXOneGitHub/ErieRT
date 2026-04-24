---@class StateType
---@field Stateless number
---@field Body number
---@field Query number
---@field BodyAndQuery number
--- An enum which represents which states `Server` takes in.
StateType = {
    Stateless = 0,
    Body = 1,
    Query = 2,
    BodyAndQuery = 3,
}