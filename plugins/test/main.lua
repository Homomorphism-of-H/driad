local function init()
    print("Hello From Inside a Lua Function!")
    return true
end

local plugin = {
    init = init,
}

print("Hello From Lua!")

return plugin
