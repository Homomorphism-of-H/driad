local function start()
    print("Hello From Inside a Lua Function!")
end

local plugin_funcs = {
    init = start
}

print("Hello From Lua!")

return plugin_funcs