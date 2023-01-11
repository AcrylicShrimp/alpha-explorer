
local theme = require("assets/scripts/editor/theme")

local camera = mk.entity.Entity.find_by_name("camera")
camera.camera.clear_color = theme.background

mk.entity.EntityBuilder.new()
  :size(mk.structure.Size.new(100, 100))
  :nine_patch_renderer({
    layer = mk.render.Layer.all(),
    order = 0,
    color = mk.render.Color.white(),
    shader = mk.asset.load_shader("nine-patch"),
    nine_patch = mk.asset.load_sprite_nine_patch("button"),
  })
  :build()
