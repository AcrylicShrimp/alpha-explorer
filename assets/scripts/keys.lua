
local keys = {}

mk.event.KeyDown.listen(function(event)
  keys[event.key] = true
end)

mk.event.KeyUp.listen(function(event)
  keys[event.key] = false
end)

return keys
