local function init()
    print("Hello From Inside a Lua Function!")
end

local plugin_funcs = {
    init = init
}

print("Hello From Lua!")

return plugin_funcs
