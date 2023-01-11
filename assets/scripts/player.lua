
local player = {
  hp = 10,
  speed = 250,
  entity = mk.entity.EntityBuilder.new()
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
    :build(),
}

local keys = require("assets/scripts/keys")

mk.event.PostUpdate.listen(function(event)
  if not keys["left"] and not keys["right"] and not keys["up"] and not keys["down"] then
    return
  end

  local movement = mk.structure.Vec2.zero()

  if keys["left"] then
    movement.x = movement.x - 1
  end
  if keys["right"] then
    movement.x = movement.x + 1
  end
  if keys["up"] then
    movement.y = movement.y + 1
  end
  if keys["down"] then
    movement.y = movement.y - 1
  end

  if 0.5 < movement:len_square() then
    player.entity.transform.position = player.entity.transform.position + (movement:norm() * player.speed * event.dt)
  end
end)

return player
