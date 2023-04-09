Game = {}

local utils = require("game/utils")

function print(...)
    local args = {...}
    
    for _, arg in ipairs(args) do
        if type(arg) == "table" then
            __rust_bindings_print(utils.dump(arg))
        elseif type(arg) == "string" then
            __rust_bindings_print(arg)
        else
            __rust_bindings_print(tostring(arg))
        end
    end
end

function Game:new()
    o = {}
    self.__index = self
    return setmetatable(o, self)
end

function Game:load()
    -- Starts loading the game
    print("⚡️ Loading game")

    -- Load game mods
    print("🚧 Loading " .. #mods .. " mod(s)")
    
    for _, mod in ipairs(mods) do
        print("✅ Loading mod " .. mod.name)
        local f, err = loadfile("game/mods/" .. mod.name .. "/mod.lua")

        if f then
            m = f()
            m.init(mod)
        else
            print("🚨 Error loading mod " .. mod.name .. ": " .. err)
        end
    end
end

game = Game:new()
game:load()
