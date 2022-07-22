use crate::api::use_context;
use crate::asset::*;
use crate::event::*;
use crate::render::*;
use crate::system::*;
use crate::util::*;
use crate::{emit_diagnostic_error, emit_diagnostic_info};
use crate::{EngineContext, EngineContextWithoutSystemManager, EngineError};
#[cfg(debug_assertions)]
use colored::*;
use glutin::dpi::LogicalSize;
use glutin::event::{ElementState, Event, MouseButton, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub fn run(
    title: &str,
    width: u32,
    height: u32,
    resizable: bool,
    asset_base: impl Into<PathBuf>,
    entry_script_path: impl AsRef<Path>,
) -> Result<(), EngineError> {
    let event_loop = EventLoop::new();
    let gfx_context = ContextBuilder::new()
        .with_vsync(true)
        .with_gl_profile(GlProfile::Core)
        .with_double_buffer(Some(true))
        .build_windowed(
            WindowBuilder::new()
                .with_visible(false)
                .with_title(title)
                .with_resizable(resizable)
                .with_inner_size(LogicalSize::new(width, height)),
            &event_loop,
        )?;
    let gfx_context = unsafe { gfx_context.make_current().map_err(|err| err.1)? };

    init(|s| gfx_context.context().get_proc_address(s));

    let context = EngineContext::new(width, height, asset_base.into())?;
    let (mut system_mgr, rest) = context.into_split();

    rest.event_mgr()
        .dispatcher()
        .add_listener(TypedEventListener::Native(BoxId::from_box(Box::new(
            |event: &events::PerEntity| {
                if let Err(err) = use_context().entity_event_mgr().emit(use_context().lua_mgr().lua(), event) {
                    emit_diagnostic_error!(format!(
                        "an error occurred while handing entity event {{entity={:?}; event={}}}: {}",
                        event.entity, event.event, err
                    ));
                }
            },
        ))));
    rest.lua_mgr().init_lua(rest.clone(), "mk")?;

    #[cfg(debug_assertions)]
    {
        fn set_color(level: events::DiagnosticLevel, str: String) -> ColoredString {
            match level {
                events::DiagnosticLevel::Debug => str.green(),
                events::DiagnosticLevel::Info => str.blue(),
                events::DiagnosticLevel::Warn => str.yellow(),
                events::DiagnosticLevel::Error => str.red(),
                events::DiagnosticLevel::Fatal => str.magenta(),
            }
        }

        rest.event_mgr()
            .dispatcher()
            .add_listener(TypedEventListener::Native(BoxId::from_box(Box::new(
                |event: &events::Diagnostic| {
                    let prefix = format!("{:>6}: ", event.level.to_str());
                    let indent = prefix.len();
                    let lines = event.message.split('\n').collect::<Vec<_>>();
                    let (&first_line, rest_lines) = lines.split_first().unwrap();
                    let message = format!(
                        "{}{} [{}:{}:{}]",
                        set_color(event.level, prefix),
                        first_line,
                        event.file,
                        event.line,
                        event.column
                    );
                    let message = if rest_lines.is_empty() {
                        message
                    } else {
                        [
                            message,
                            rest_lines
                                .iter()
                                .map(|&line| format!("{:indent$}{}", "", line, indent = indent))
                                .collect::<Vec<_>>()
                                .join("\n"),
                        ]
                        .join("\n")
                    };

                    println!("{}", message);

                    for sub_diagnostics in &event.sub_diagnostics {
                        let prefix = format!("> {:>6}: ", sub_diagnostics.level.to_str());
                        let indent = prefix.len();
                        let lines = sub_diagnostics.message.split('\n').collect::<Vec<_>>();
                        let (&first_line, rest_lines) = lines.split_first().unwrap();
                        let message = format!(
                            "        {}{} [{}:{}:{}]",
                            set_color(sub_diagnostics.level, prefix),
                            first_line,
                            sub_diagnostics.file,
                            sub_diagnostics.line,
                            sub_diagnostics.column
                        );
                        let message = if rest_lines.is_empty() {
                            message
                        } else {
                            [
                                message,
                                rest_lines
                                    .iter()
                                    .map(|&line| {
                                        format!("        {:indent$}{}", "", line, indent = indent)
                                    })
                                    .collect::<Vec<_>>()
                                    .join("\n"),
                            ]
                            .join("\n")
                        };

                        println!("{}", message);
                    }
                },
            ))));
    }

    emit_diagnostic_info!(format!("registering built-in systems."));

    system_mgr.register_system(isize::MIN, |context: &EngineContextWithoutSystemManager| {
        context.time_mgr_mut().update();
    });
    system_mgr.register_system(-11000, |context: &EngineContextWithoutSystemManager| {
        context.event_mgr().dispatcher().emit(
            context.lua_mgr().lua(),
            &events::PreUpdate {
                dt: context.time_mgr().dt(),
            },
        );
    });
    system_mgr.register_system(-10900, |context: &EngineContextWithoutSystemManager| {
        animate_sigle_animations(
            &mut context.world_mut(),
            &context.time_mgr(),
            &mut context.transform_mgr_mut(),
        );
    });
    system_mgr.register_system(-10800, |context: &EngineContextWithoutSystemManager| {
        context.event_mgr().dispatcher().emit(
            context.lua_mgr().lua(),
            &events::Update {
                dt: context.time_mgr().dt(),
            },
        );
    });
    system_mgr.register_system(-10700, |context: &EngineContextWithoutSystemManager| {
        context.event_mgr().dispatcher().emit(
            context.lua_mgr().lua(),
            &events::PostUpdate {
                dt: context.time_mgr().dt(),
            },
        );
    });
    system_mgr.register_system(-10600, |context: &EngineContextWithoutSystemManager| {
        context.ui_mgr_mut().update_elements();
    });
    system_mgr.register_system(-10500, |context: &EngineContextWithoutSystemManager| {
        context.transform_mgr_mut().update_world_matrices();
    });
    system_mgr.register_system(0, |context: &EngineContextWithoutSystemManager| {
        context.event_mgr().dispatcher().emit(
            context.lua_mgr().lua(),
            &events::PreRender {
                dt: context.time_mgr().dt(),
            },
        );
    });
    system_mgr.register_system(1, |context: &EngineContextWithoutSystemManager| {
        context.render_mgr().update_uniforms(context);
    });
    system_mgr.register_system(100, RendererSystem::new());
    system_mgr.register_system(200, |context: &EngineContextWithoutSystemManager| {
        context.event_mgr().dispatcher().emit(
            context.lua_mgr().lua(),
            &events::PostRender {
                dt: context.time_mgr().dt(),
            },
        );
    });
    system_mgr.register_system(isize::MAX, |context: &EngineContextWithoutSystemManager| {
        context.screen_mgr_mut().reset_dirty();
    });

    {
        emit_diagnostic_info!(format!("registering asset loaders."));

        let mut asset_mgr = rest.asset_mgr_mut();
        asset_mgr.register_loader(loader::font_loader());
        asset_mgr.register_loader(loader::shader_loader());
        asset_mgr.register_loader(loader::sprite_loader());
        asset_mgr.register_loader(loader::sprite_atlas_loader());
        asset_mgr.register_loader(loader::sprite_atlas_grid_loader());
        asset_mgr.register_loader(loader::sprite_nine_patch_loader());
        asset_mgr.register_loader(loader::tilemap_loader());
    }

    {
        emit_diagnostic_info!(format!("abjusting scale factor."));

        let scale_factor = gfx_context.window().scale_factor();
        rest.screen_mgr_mut().update_scale_factor(
            scale_factor,
            &LogicalSize::new(width, height).to_physical(scale_factor),
        );
    }

    {
        emit_diagnostic_info!(format!("executing entry script."));

        let path = entry_script_path.as_ref();
        let lua_mgr = rest.lua_mgr();
        lua_mgr.execute(path, read_to_string(path)?)?;
    }

    emit_diagnostic_info!(format!("engine is up and running."));

    {
        let screen_mgr = rest.screen_mgr();
        resize(
            screen_mgr.physical_width() as u32,
            screen_mgr.physical_height() as u32,
        );
    }
    clear();
    gfx_context.swap_buffers().unwrap();
    gfx_context.window().set_visible(true);

    let window_id = gfx_context.window().id();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        system_mgr.run(&rest, isize::MIN, -1);

        match event {
            Event::MainEventsCleared => {}
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                window_id: id,
            } if id == window_id => {
                rest.input_mgr_mut().handle_event(&input);

                if let Some(key) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            rest.event_mgr()
                                .dispatcher()
                                .emit(rest.lua_mgr().lua(), &events::KeyDown::from_key(key));
                        }
                        ElementState::Released => {
                            rest.event_mgr()
                                .dispatcher()
                                .emit(rest.lua_mgr().lua(), &events::KeyUp::from_key(key));
                        }
                    }
                }

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorEntered { .. },
                window_id: id,
            } if id == window_id => {
                rest.event_mgr()
                    .dispatcher()
                    .emit(rest.lua_mgr().lua(), &events::PointerEnter);

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorLeft { .. },
                window_id: id,
            } if id == window_id => {
                rest.event_mgr()
                    .dispatcher()
                    .emit(rest.lua_mgr().lua(), &events::PointerExit);
                rest.ui_event_mgr_mut().handle_mouse_exit();

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                window_id: id,
            } if id == window_id => {
                let position = position.to_logical(rest.screen_mgr().scale_factor());

                rest.event_mgr().dispatcher().emit(
                    rest.lua_mgr().lua(),
                    &events::PointerMove {
                        pointer_x: position.x,
                        pointer_y: position.y,
                    },
                );
                rest.ui_event_mgr_mut()
                    .handle_mouse_move(position.x as f32, position.y as f32);

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { button, state, .. },
                window_id: id,
            } if id == window_id => {
                let button_name = match button {
                    MouseButton::Left => "left",
                    MouseButton::Right => "right",
                    MouseButton::Middle => "middle",
                    _ => return,
                };

                match state {
                    ElementState::Pressed => {
                        rest.event_mgr().dispatcher().emit(
                            rest.lua_mgr().lua(),
                            &events::PointerDown {
                                button: button_name,
                            },
                        );
                        rest.ui_event_mgr_mut().handle_mouse_button_down(button);
                    }
                    ElementState::Released => {
                        rest.event_mgr().dispatcher().emit(
                            rest.lua_mgr().lua(),
                            &events::PointerUp {
                                button: button_name,
                            },
                        );
                        rest.ui_event_mgr_mut().handle_mouse_button_up(button);
                    }
                }

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(inner_size),
                window_id: id,
            } if id == window_id => {
                rest.screen_mgr_mut().update_size(&inner_size);
                gfx_context.resize(inner_size);
                return;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    },
                window_id: id,
            } if id == window_id => {
                rest.screen_mgr_mut()
                    .update_scale_factor(scale_factor, new_inner_size);
                gfx_context.resize(*new_inner_size);
                return;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: id,
            } if id == window_id => {
                *control_flow = ControlFlow::Exit;
                return;
            }
            _ => return,
        }

        {
            let screen_mgr = rest.screen_mgr();
            resize(
                screen_mgr.physical_width() as u32,
                screen_mgr.physical_height() as u32,
            );
        }
        clear();
        system_mgr.run(&rest, 0, isize::MAX);

        gfx_context.swap_buffers().unwrap();
    });
}
