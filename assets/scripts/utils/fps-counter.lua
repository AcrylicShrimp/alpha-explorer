local Queue = require('assets/scripts/collections/queue')

local font_size = 16

local root = mk.Entity.get_by_name("root")
local indicator = mk.Entity.build {
    name = "fps-indicator",
    transform = {
        parent = root.transform
    },
    ui_element = {
        anchor = {
            min = {
                x = 0,
                y = 0
            },
            max = {
                x = 1,
                y = 1
            }
        },
        margin = {
            left = 15,
            right = 0,
            top = 5,
            bottom = 0
        },
        is_interactible = false,
        order_index = 999999,
    },
    glyph_renderer = {
        order = 999999,
        color = mk.Color.white(),
        shader = mk.Shader.load("glyph"),
        font = mk.Font.load("Courier Prime Sans"),
        font_size = font_size,
        thickness = 0.5,
        smoothness = 2 / font_size,
        config = {
            horizontal_align = "left",
            vertical_align = "top"
        },
        text = "FPS: 0"
    }
}

local fps = 0
local fps_time_queue = Queue:new()

mk.Event.Update.listen(function(event)
    local now = mk.Time.time()

    fps = fps + 1
    fps_time_queue:push_right(now + 1)

    while fps_time_queue:peek_left() < now do
        fps = fps - 1
        fps_time_queue:pop_left()
    end

    indicator.glyph_renderer.text = "FPS(counter): " .. fps
end)
