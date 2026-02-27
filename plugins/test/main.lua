-- Initial entry point to the plugin, should run only once
local function init()
    print("Hello From Inside a Lua Function!")
end

local function draw_pass()
   return {
    x = 15,
    y = 5,
    glyph = 'Z',
   }
end

-- Code that is executed when the plugin is loaded, this should be considered bad practice.
print("Hello From Lua!")

return {
    init = init,
    draw_pass = draw_pass,
}
