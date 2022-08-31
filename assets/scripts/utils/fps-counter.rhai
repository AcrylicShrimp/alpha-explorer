
import "mk" as mk;
import "collections/queue" as Queue;

fn create() {
  let font_size = 16;

  let root = mk::entity::Entity::find_by_name("root");
  let indicator = mk::entity::EntityBuilder::create()
    .name("fps-counter")
    .transform_parent(root.transform)
    .ui_element(#{
      anchor: mk::ui::UIAnchor::full(),
      margin: mk::ui::UIMargin::create(15, 0, 0, 0),
      is_interactible: false,
      order_index: 999999,
    })
    .glyph_renderer(#{
      layer: mk::render::Layer::all(),
      order: 999999,
      color: mk::render::Color::white(),
      shader: mk::Asset::load_shader("glyph"),
      thickness: 0.5,
      smoothness: 2 / font_size,
      font: mk::Asset::load_font("Courier Prime Sans"),
      font_size: font_size,
      text: "FPS: 0",
      config: mk::component::GlyphLayoutConfig::create(
        mk::component::HorizontalAlign::Left,
        mk::component::VerticalAlign::Top,
        mk::component::WrapStyle::Word,
        false,
      ),
    })
    .build();

  let fps = 0;
  let fps_time_queue = Queue::create();

  mk::Event::Update::listen(|event| {
    let now = mk::Time::time();

    fps += 1;
    fps_time_queue.push_right(now + 1);

    while fps_time_queue.peek_left() < now {
      fps -= 1;
      fps_time_queue.pop_left();
    }

    indicator.glyph_renderer.text = "FPS(counter): " + fps
  });

  indicator
}
