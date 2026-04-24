---@class FileContents
--- The class meant to represent many file types. Used mainly in Rust-side code.
FileContents = {}

---@param path string
---@return FileContents
--- Tries to read a file and creates a FileContents object.
function FileContents.read(path)
    return {}
end

---@param path string
---@return FileContents
--- Tries to read a file and creates a FileContents object. Only uses `FileContents::Plain` internally.
function FileContents.readAsString(path)
    return {}
end

---@param path string
---@return FileContents
--- Tries to read a file and creates a FileContents object. Only uses `FileContents::Bytes` internally.
function FileContents.readAsBytes(path)
    return {}
end