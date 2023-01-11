
local font_size = 16

-- mk.entity.EntityBuilder.new()
--   :size(mk.structure.Size.new(50, 50))
--   :sprite_renderer({
--     layer = mk.render.Layer.new(2),
--     order = 10,
--     color = mk.render.Color.parse_hex("#2f3133"),
--     shader = mk.asset.load_shader("sprite"),
--     sprite = mk.asset.load_sprite("empty"),
--   })
--   :build()

local ui_root = require("assets/scripts/ui/ui-root")
local background = mk.entity.EntityBuilder.new()
  :transform_parent(ui_root.transform)
  :ui_element({
    anchor = mk.ui.Anchor.new(
      mk.structure.Vec2.new(0.5, 0.5),
      mk.structure.Vec2.new(0.5, 0.5)
    ),
    margin = mk.ui.Margin.from_size(
      mk.structure.Vec2.new(1, 1),
      mk.structure.Vec2.new(0, 0),
      mk.structure.Size.new(50, 50)
    ),
    is_interactible = false,
    order_index = 10,
  })
  :sprite_renderer({
    layer = mk.render.Layer.new(2),
    order = 10,
    color = mk.render.Color.parse_hex("#2f3133"),
    shader = mk.asset.load_shader("sprite"),
    sprite = mk.asset.load_sprite("empty"),
  })
  :build()

-- local ui_status_indicator
-- ui_status_indicator = {
--   entity = ,
--   update = function(self)

--   end,
-- }

-- return ui_status_indicator
