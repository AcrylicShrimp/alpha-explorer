
-- local a = mk.structure.Mat33.identity()
-- print(a:transposed())
-- print(a:inversed())
-- print(mk.structure.Vec3.new(10, -20, 1) * a)
-- print(mk.structure.Mat33.affine_translation(mk.structure.Vec2.new(-10, 10)) * mk.structure.Mat33.affine_translation(mk.structure.Vec2.new(-10, 10)):inversed())
-- print(mk.structure.Vec3.new(10, -20, 1) * mk.structure.Mat33.affine_translation(mk.structure.Vec2.new(-10, 10)))

-- local event = mk.event.Event.new()

-- local listener;
-- listener = event.listen(function(event)
--   print(event)
-- end)

-- event.emit({
--   a = nil,
--   b = "123",
--   c = 123
-- })

-- event.unlisten(listener)

local camera = require("assets/scripts/camera")

local shader = mk.asset.load_shader("sprite")
local sprite = mk.asset.load_sprite("arrow")

local sprite_renderer = mk.entity.EntityBuilder.new()
  :name("sprite-renderer")
  :size(mk.structure.Size.new(100, 100))
  :sprite_renderer {
    layer = mk.gfx.Layer.new(1),
    order = 0,
    color = mk.gfx.Color.white(),
    shader = shader,
    sprite = sprite
  }
  :build()

local handler
handler = sprite_renderer:listen("test", function (entity, event_name, event_param)
  print("event called from: ", entity.name)
  print("event name: ", event_name)
  print("event data: ", event_param.a)
  print("event data: ", event_param.b)
  print("event data: ", event_param.c)
  sprite_renderer:unlisten("test", handler)
end)
print("handler: ", handler)
sprite_renderer:emit("test", {
  a = nil,
  b = "123",
  c = 123
})
sprite_renderer:emit("test", {
  a = nil,
  b = "123",
  c = 123
})
sprite_renderer:listen("test", handler)
sprite_renderer:emit("test", {
  a = nil,
  b = "123",
  c = 123
})

-- for i = 1, 1000 do
--   mk.entity.EntityBuilder.new()
--     :transform_position(mk.structure.Vec2.new(math.random(-300, 300), math.random(-300, 300)))
--     :size(mk.structure.Size.new(100, 100))
--     :sprite_renderer {
--       layer = mk.gfx.Layer.new(1),
--       order = 0,
--       color = mk.gfx.Color.white(),
--       shader = shader,
--       sprite = sprite
--     }
--     :build()
-- end

-- local ui_status_indicator = require("assets/scripts/ui/ui-status-indicator")
require("assets/scripts/utils/fps-counter")

-- local generate_map = require("assets/scripts/map-gen/generate")
-- local map = generate_map(100, 100)

-- mk.entity.EntityBuilder.new()
--   :name("map")
--   :alpha_tilemap_renderer({
--     layer = mk.render.Layer.new(1),
--     order = 0,
--     color = mk.render.Color.white(),
--     fore_shader = mk.asset.load_shader("glyph"),
--     back_shader = mk.asset.load_shader("color"),
--     font = mk.asset.load_font("Courier Prime Sans"),
--     font_size = 16,
--     thickness = 0.5,
--     smoothness = 2 / 16,
--     tilemap = mk.render.AlphaTilemap.new(
--       16, 16,
--       100, 100,
--       map.map,
--       mk.render.AlphaTileset.new({
--         mk.render.AlphaTile.new(mk.render.Color.black(), mk.render.Color.white(), "#"),
--         mk.render.AlphaTile.new(mk.render.Color.from_rgba(1, 1, 1, 0.5), mk.render.Color.black(), "."),
--       })
--     ),
--   })
--   :build()

-- local player = require("assets/scripts/player")

-- player.entity.transform.position = mk.structure.Vec2.new(
--   (map.rooms[1].x + map.rooms[1].width / 2) * 16,
--   ((100 - map.rooms[1].y) - map.rooms[1].height / 2) * 16
-- )

-- camera.transform.parent = player.entity.transform

-- local keys = require("assets/scripts/keys")

-- local player_movement = mk.input.Action.new({
--   name = "PlayerMovement",
--   triggers = {}
-- })
-- player_movement.add_trigger(mk.input.Keyboard.key_down("ArrowLeft"))

-- local player_movement = mk.input.Action.new({
--   name = "PlayerMovement",
--   triggers = {
--     mk.input.Keyboard.key_down("ArrowLeft"),
--     mk.input.Keyboard.key_stay("ArrowLeft"),
--     mk.input.Keyboard.key_up("ArrowLeft"),

--     mk.input.Gamepad.button_down("DPadLeft"),
--     mk.input.Gamepad.button_stay("DPadLeft"),
--     mk.input.Gamepad.button_up("DPadLeft"),
--     mk.input.Gamepad.axis_1d("DPadHorizontal"),
--     mk.input.Gamepad.axis_2d("DPad"),
--     mk.input.Gamepad.axis_2d("DPadHorizontal", "DPadVertical"),

--     mk.input.Touch.tap(),
--     mk.input.Touch.long_tap(),
--     mk.input.Touch.double_tap(),
--     mk.input.Touch.slide(),

--     mk.input.Complex.all({
--       mk.input.Keyboard.key_down("ArrowLeft"),
--       mk.input.Keyboard.key_down("ArrowRight"),
--     }),
--     mk.input.Complex.any({
--       mk.input.Keyboard.key_down("ArrowLeft"),
--       mk.input.Keyboard.key_down("ArrowRight"),
--     }),
--     mk.input.Complex.sequence({
--       mk.input.Keyboard.key_down("ArrowLeft"),
--       mk.input.Keyboard.key_down("ArrowRight"),
--     }),
--   },
-- })

-- local action_map = mk.input.ActionMap.new()
-- action_map.add_action(player_movement)

-- local input = mk.input.Input.new()
-- input.bind_action_map("Field", action_map)

-- input.activate("Field")
-- input.activate({"Field", "Menu"})
