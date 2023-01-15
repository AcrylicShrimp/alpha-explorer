
local camera = mk.entity.EntityBuilder.new()
  :name("camera")
  :camera({
    layer = mk.gfx.Layer.all(),
    order = 0,
    clear_mode = mk.gfx.ClearMode.Color,
    clear_color = mk.gfx.Color.parse_hex("#61470000"),
  })
  :build()

return camera
