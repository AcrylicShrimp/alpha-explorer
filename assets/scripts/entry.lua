local camera = mk.entity.EntityBuilder.new()
  :name("camera")
  :camera({
    layer = mk.render.Layer.all(),
    order = 0,
  })
  :build()

mk.entity.EntityBuilder.new()
  :name("root")
  :transform_parent(camera.transform)
  :ui_element({
    anchor = mk.ui.Anchor.full(),
    margin = mk.ui.Margin.zero(),
    is_interactible = false,
    order_index = 0,
  })
  :ui_scaler({
    mode = mk.ui.ScaleMode.Stretch,
    reference_size = mk.structure.Size.new(1024, 768),
  })
  :build()

require("assets/scripts/utils/fps-counter")

local generate_map = require("assets/scripts/map-gen/generate")
local map = generate_map(100, 100)

mk.entity.EntityBuilder.new()
  :name("map")
  :alpha_tilemap_renderer({
    layer = mk.render.Layer.new(1),
    order = 0,
    color = mk.render.Color.white(),
    fore_shader = mk.asset.load_shader("glyph"),
    back_shader = mk.asset.load_shader("color"),
    font = mk.asset.load_font("Courier Prime Sans"),
    font_size = 16,
    thickness = 0.5,
    smoothness = 2 / 16,
    tilemap = mk.render.AlphaTilemap.new(
      16, 16,
      100, 100,
      map.map,
      mk.render.AlphaTileset.new({
        mk.render.AlphaTile.new(mk.render.Color.black(), mk.render.Color.white(), "#"),
        mk.render.AlphaTile.new(mk.render.Color.from_rgba(1, 1, 1, 0.5), mk.render.Color.black(), "."),
      })
    ),
  })
  :build()

local player = mk.entity.EntityBuilder.new()
  :name("player")
  :size(mk.structure.Size.new(16, 16))
  :glyph_renderer({
    layer = mk.render.Layer.new(1),
    order = 1,
    color = mk.render.Color.from_rgb(0 / 255, 150 / 255, 105 / 255),
    shader = mk.asset.load_shader("glyph"),
    font = mk.asset.load_font("Courier Prime Sans"),
    font_size = 16,
    thickness = 0.5,
    smoothness = 2 / 16,
    text = "@",
    config = mk.glyph.GlyphLayoutConfig.new(
      mk.glyph.HorizontalAlign.Center,
      mk.glyph.VerticalAlign.Middle,
      mk.glyph.WrapStyle.Word,
      true
    ),
  })
  :build()

player.transform.position = mk.structure.Vec2.new(
  (map.rooms[1].x + map.rooms[1].width / 2) * 16,
  ((100 - map.rooms[1].y) - map.rooms[1].height / 2) * 16
)

camera.transform.parent = player.transform

local keys = {}

mk.event.KeyDown.listen(function(event)
  keys[event.key] = true
end);

mk.event.KeyUp.listen(function(event)
  keys[event.key] = false
end);

mk.event.PostUpdate.listen(function(event)
  if not keys["left"] and not keys["right"] and not keys["up"] and not keys["down"] then
    return
  end

  local position = player.transform.position

  if keys["left"] then
    position.x = position.x - event.dt * 400
  end
  if keys["right"] then
    position.x = position.x + event.dt * 400
  end
  if keys["up"] then
    position.y = position.y + event.dt * 400
  end
  if keys["down"] then
    position.y = position.y - event.dt * 400
  end

  player.transform.position = position
end)
