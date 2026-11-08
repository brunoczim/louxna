suprise_isolation = false

local count

count = require("brunoczim/simple::countmod", { 1, 2, 3 })
print("#({ 1, 2, 3 }) = " .. count)

count = require("simple::countmod", { 9 })
print("#({ 9 }) = " .. count)

count = require("countmod", { 1, 2, 3, 4 })
print("#({ 1, 2, 3, 4 }) = " .. count)

count = require("countmod", { })
print("#({ }) = " .. count)

count = require("countmod", { })
print("#({ }) = " .. count)

print("#arguments = " .. #arguments)

for i, argument in ipairs(arguments) do
    print("arguments[" .. i .. "] = " .. argument)
end

return 0
