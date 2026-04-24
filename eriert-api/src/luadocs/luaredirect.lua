---@class Redirect
--- The class used in `Response` to make a redirect over HTTP.
---@see Response.send
Redirect = {}

---Makes a Redirect with `303 See Other`.
---@param uri string
---@return Redirect
function Redirect.to(uri)
    return {}
end

---Makes a Redirect with `307 Temporary Redirect`.
---@param uri string
---@return Redirect
function Redirect.temporary(uri)
    return {}
end

---Makes a Redirect with `308 Permanent Redirect`.
---@param uri string
---@return Redirect
function Redirect.permanent(uri)
    return {}
end