
local camera = require("assets/scripts/camera")
local ui_root = mk.entity.EntityBuilder.new()
  :name("ui-root")
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

return ui_root
