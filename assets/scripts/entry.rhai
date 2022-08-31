
import "mk" as mk;
import "utils/fps-counter" as FPSCounter;

let audio1 = mk::Asset::load_audio_clip("sfx/jump");
let audio2 = mk::Asset::load_audio_clip("bgm/BabyElephantWalk60");

let source1 = mk::entity::EntityBuilder::create()
  .audio_source()
  .build();
let source2 = mk::entity::EntityBuilder::create()
  .audio_source()
  .build();

source1.audio_source.clip = audio1;
// source1.audio_source.play();

source2.audio_source.clip = audio2;
// source2.audio_source.play();

let camera = mk::entity::EntityBuilder::create()
  .name("camera")
  .camera(#{
    layer: mk::render::Layer::all(),
    order: 0,
  })
  .build();

mk::entity::EntityBuilder::create()
  .name("root")
  .transform_parent(camera.transform)
  .ui_element(#{
    anchor: mk::ui::UIAnchor::full(),
    margin: mk::ui::UIMargin::zero(),
    order_index: 0,
    is_interactible: false,
  })
  .ui_scaler(#{
    mode: mk::component::UIScaleMode::Stretch,
    reference_size: mk::structure::Size::create(1024, 768),
  })
  .build();

FPSCounter::create();
