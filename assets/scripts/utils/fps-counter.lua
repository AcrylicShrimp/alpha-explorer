local Queue = require('assets/scripts/collections/queue')

local font_size = 16

local ui_root = require("assets/scripts/ui/ui-root")
local indicator = mk.entity.EntityBuilder.new()
  :name("fps-indicator")
  :transform_parent(ui_root.transform)
  :ui_element({
    anchor = mk.ui.Anchor.new(
      mk.structure.Vec2.new(0, 1),
      mk.structure.Vec2.new(0, 1)
    ),
    margin = mk.ui.Margin.new(15, 0, 5, 0),
    is_interactible = false,
    order_index = 999999,
  })
  :glyph_renderer({
    layer = mk.gfx.Layer.new(1),
    order = 999999,
    color = mk.gfx.Color.white(),
    shader = mk.asset.load_shader("glyph"),
    font = mk.asset.load_font("Courier Prime Sans"),
    font_size = font_size,
    thickness = 0.5,
    smoothness = 2 / font_size,
    config = mk.gfx.GlyphLayoutConfig.new(
      mk.gfx.HorizontalAlign.Left,
      mk.gfx.VerticalAlign.Top,
      mk.gfx.WrapStyle.Word,
      true
    ),
  })
  :build()

local fps = 0
local fps_time_queue = Queue:new()

mk.event.Update.listen(function(event)
  local now = mk.time.Time.time()
  
  fps = fps + 1
  fps_time_queue:push_right(now + 1)
  
  while fps_time_queue:peek_left() < now do
    fps = fps - 1
    fps_time_queue:pop_left()
  end
  
  indicator.glyph_renderer.text = "FPS(counter): " .. fps
end)
