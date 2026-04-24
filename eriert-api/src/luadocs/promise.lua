---@class Promise
--- The class used to handle work on another thread asynchronously.
Promise = {}

---@param callback fun()
---@return Promise
--- Initializes `Promise` with a specified callback.
function Promise.make(callback)
    return {}
end

---@param successCallback fun(success: any)
--- Sets the `Promise`'s success callback.
function Promise:onSuccess(successCallback) end

---@param errorCallback fun(error: any)
--- Sets the `Promise`'s error callback.
function Promise:onError(errorCallback) end

--- Invokes the `Promise`'s callback and either invokes `onSuccess` or `onError`.
function Promise:pull() end