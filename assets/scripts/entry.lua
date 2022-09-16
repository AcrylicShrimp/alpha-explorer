local audio1 = mk.asset.load_audio_clip("sfx/jump")
local audio2 = mk.asset.load_audio_clip("bgm/BabyElephantWalk60")

local source1 = mk.entity.EntityBuilder.new()
  :audio_source({})
  :build()
local source2 = mk.entity.EntityBuilder.new()
  :audio_source({})
  :build()

source1.audio_source.clip = audio1
source1.audio_source:play()

source2.audio_source.clip = audio2
source2.audio_source:play()

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

-- local generate_map = require("assets/scripts/map-gen/generate")
-- local map = generate_map(100, 100)

-- mk.Entity.build {
--   name = "map",
--   alpha_tilemap_renderer = {
--     order = 0,
--     fore_shader = mk.Shader.load("glyph"),
--     back_shader = mk.Shader.load("color"),
--     font = mk.Font.load("Courier Prime Sans"),
--     font_size = 16,
--     thickness = 0.5,
--     smoothness = 2 / 16,
--     tilemap = {
--       tile_width = 16,
--       tile_height = 16,
--       tile_count_x = 100,
--       tile_count_y = 100,
--       layer = map.map,
--       tileset = {
--         tiles = {
--           {
--             fore_color = mk.Color.black(),
--             back_color = mk.Color.white(),
--             character = "#"
--           },
--           {
--             fore_color = mk.Color.rgba(1, 1, 1, 0.5),
--             back_color = mk.Color.black(),
--             character = "."
--           }
--         }
--       }
--     }
--   }
-- }

-- local player = mk.Entity.build {
--   name = "player",
--   size = {
--     width = 16,
--     height = 16
--   },
--   glyph_renderer = {
--     order = 1,
--     color = mk.Color.rgb(0 / 255, 150 / 255, 105 / 255),
--     shader = mk.Shader.load("glyph"),
--     font = mk.Font.load("Courier Prime Sans"),
--     font_size = 16,
--     thickness = 0.5,
--     smoothness = 2 / 16,
--     text = "@",
--     config = {
--       horizontal_align = "center",
--       vertical_align = "middle"
--     }
--   }
-- }

-- local keys = {}

-- mk.Event.KeyDown.listen(function(event)
--   keys[event.key] = true

--   if event.key == "space" then
--     source1.audio_source:play()
--   end
-- end);

-- mk.Event.KeyUp.listen(function(event)
--   keys[event.key] = false
-- end);

-- mk.Event.PostUpdate.listen(function(event)
--   if not keys["left"] and not keys["right"] and not keys["up"] and not keys["down"] then
--     return
--   end

--   local position = player.transform.position

--   if keys["left"] then
--     position.x = position.x - event.dt * 400
--   end
--   if keys["right"] then
--     position.x = position.x + event.dt * 400
--   end
--   if keys["up"] then
--     position.y = position.y + event.dt * 400
--   end
--   if keys["down"] then
--     position.y = position.y - event.dt * 400
--   end

--   player.transform.position = position
-- end)

-- camera.transform.parent = player.transform

-- player.transform.position = {
--   x = (map.rooms[1].x + map.rooms[1].width / 2) * 16,
--   y = ((100 - map.rooms[1].y) - map.rooms[1].height / 2) * 16
-- }
