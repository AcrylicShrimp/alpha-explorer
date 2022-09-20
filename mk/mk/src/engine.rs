use crate::asset::*;
use crate::emit_diagnostic_info;
use crate::event::*;
use crate::render::*;
use crate::script::event::DiagnosticLevel;
use crate::system::*;
use crate::util::*;
use crate::EngineContext;
use anyhow::Context;
use anyhow::Result;
use ash::vk;
#[cfg(debug_assertions)]
use colored::*;
use specs::RunNow;
use std::fs::read_to_string;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

static mut CONTEXT: MaybeUninit<Arc<EngineContext>> = MaybeUninit::uninit();

pub fn use_context() -> &'static EngineContext {
    unsafe { CONTEXT.assume_init_ref() }.as_ref()
}

pub fn run(
    title: &str,
    width: u32,
    height: u32,
    resizable: bool,
    asset_base: impl Into<PathBuf>,
    entry_script_path: impl AsRef<Path>,
) -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .with_resizable(resizable)
        .with_inner_size(LogicalSize::new(width, height))
        .build(&event_loop)?;

    let entry = unsafe {
        let entry = ash::Entry::linked();
        let surface_extensions = ash_window::enumerate_required_extensions(&window)?;
        let app_desc = vk::ApplicationInfo::builder()
            .api_version(vk::make_api_version(0, 1, 1, 0))
            .build();
        let instance_desc = vk::InstanceCreateInfo::builder()
            .application_info(&app_desc)
            .enabled_extension_names(surface_extensions)
            .build();

        let instance = entry.create_instance(&instance_desc, None)?;

        let surface = ash_window::create_surface(&entry, &instance, &window, None)?;
        let surface_fn = ash::extensions::khr::Surface::new(&entry, &instance);
    };

    // let gfx_context = ContextBuilder::new()
    //     .with_vsync(true)
    //     .with_gl_profile(GlProfile::Core)
    //     .with_double_buffer(Some(true))
    //     .build_windowed(
    //         WindowBuilder::new()
    //             .with_visible(false)
    //             .with_title(title)
    //             .with_resizable(resizable)
    //             .with_inner_size(LogicalSize::new(width, height)),
    //         &event_loop,
    //     )?;
    // let gfx_context = unsafe { gfx_context.make_current().map_err(|err| err.1)? };

    init(|s| gfx_context.context().get_proc_address(s));

    let context = Arc::new(EngineContext::new(width, height, asset_base.into())?);

    unsafe {
        CONTEXT.write(context.clone());
    }

    // context.event_mgr()
    //     .dispatcher()
    //     .add_listener(TypedEventListener::Native(BoxId::from_box(Box::new(
    //         |event: &crate::script::event::PerEntity| {
    //             if let Err(err) = use_context().entity_event_mgr().emit(use_context().lua_mgr().lua(), event) {
    //                 emit_diagnostic_error!(format!(
    //                     "an error occurred while handing entity event {{entity={:?}; event={}}}: {}",
    //                     event.entity, event.event, err
    //                 ));
    //             }
    //         },
    //     ))));

    #[cfg(debug_assertions)]
    {
        fn set_color(level: DiagnosticLevel, str: String) -> ColoredString {
            match level {
                DiagnosticLevel::Debug => str.green(),
                DiagnosticLevel::Info => str.blue(),
                DiagnosticLevel::Warn => str.yellow(),
                DiagnosticLevel::Error => str.red(),
                DiagnosticLevel::Fatal => str.magenta(),
            }
        }

        context
            .event_mgr()
            .dispatcher()
            .add_listener(TypedEventListener::Native(BoxId::from_box(Box::new(
                |event: &crate::script::event::Diagnostic| {
                    let prefix = format!("{:>6}: ", event.level.to_str());
                    let indent = prefix.len();
                    let lines = event.message.split('\n').collect::<Vec<_>>();
                    let (&first_line, context_lines) = lines.split_first().unwrap();
                    let message = format!(
                        "{}{} [{}:{}:{}]",
                        set_color(event.level, prefix),
                        first_line,
                        event.file,
                        event.line,
                        event.column
                    );
                    let message = if context_lines.is_empty() {
                        message
                    } else {
                        [
                            message,
                            context_lines
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
                        let (&first_line, context_lines) = lines.split_first().unwrap();
                        let message = format!(
                            "        {}{} [{}:{}:{}]",
                            set_color(sub_diagnostics.level, prefix),
                            first_line,
                            sub_diagnostics.file,
                            sub_diagnostics.line,
                            sub_diagnostics.column
                        );
                        let message = if context_lines.is_empty() {
                            message
                        } else {
                            [
                                message,
                                context_lines
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

    emit_diagnostic_info!(format!("configuring built-in systems."));

    let mut audio_system = AudioSystem;
    let mut render_system = RenderSystem::new();

    let mut systems_pre_render = {
        let context = context.clone();
        move || {
            context.time_mgr_mut().update();
            audio_system.run_now(&context.world());
            // animate_sigle_animations(
            //     &mut context.world_mut(),
            //     &context.time_mgr(),
            //     &mut context.transform_mgr_mut(),
            // );
            context.ui_mgr_mut().update_elements();
            context.transform_mgr_mut().update_world_matrices();

            context
                .event_mgr()
                .dispatcher()
                .emit(&crate::script::event::PreUpdate {
                    dt: context.time_mgr().dt_f64(),
                });
            context
                .event_mgr()
                .dispatcher()
                .emit(&crate::script::event::Update {
                    dt: context.time_mgr().dt_f64(),
                });
            context
                .event_mgr()
                .dispatcher()
                .emit(&crate::script::event::PostUpdate {
                    dt: context.time_mgr().dt_f64(),
                });
        }
    };

    let mut systems_render = {
        let context = context.clone();
        move || {
            context
                .event_mgr()
                .dispatcher()
                .emit(&crate::script::event::PreRender {
                    dt: context.time_mgr().dt_f64(),
                });

            context.render_mgr().update_uniforms(&context);
            render_system.run_now(&context.world());

            context
                .event_mgr()
                .dispatcher()
                .emit(&crate::script::event::PostRender {
                    dt: context.time_mgr().dt_f64(),
                });

            context.screen_mgr_mut().reset_dirty();
        }
    };

    {
        emit_diagnostic_info!(format!("registering asset loaders."));

        let mut asset_mgr = context.asset_mgr_mut();
        asset_mgr.register_loader(loader::audio_clip_loader());
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
        context.screen_mgr_mut().update_scale_factor(
            scale_factor,
            &LogicalSize::new(width, height).to_physical(scale_factor),
        );
    }

    {
        emit_diagnostic_info!(format!("executing entry script."));

        let path = entry_script_path.as_ref();
        let script_mgr = context.script_mgr();
        script_mgr
            .execute(read_to_string(path).with_context(|| "failed to read entry script")?)
            .with_context(|| "failed to execute entry script")?;
    }

    emit_diagnostic_info!(format!("engine is up and running."));

    {
        let screen_mgr = context.screen_mgr();
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

        match event {
            Event::MainEventsCleared => {}
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                window_id: id,
            } if id == window_id => {
                context.input_mgr_mut().handle_event(&input);

                if let Some(key) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            context
                                .event_mgr()
                                .dispatcher()
                                .emit(&crate::script::event::KeyDown::from_key(key));
                        }
                        ElementState::Released => {
                            context
                                .event_mgr()
                                .dispatcher()
                                .emit(&crate::script::event::KeyUp::from_key(key));
                        }
                    }
                }

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorEntered { .. },
                window_id: id,
            } if id == window_id => {
                context
                    .event_mgr()
                    .dispatcher()
                    .emit(&crate::script::event::PointerEnter);

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorLeft { .. },
                window_id: id,
            } if id == window_id => {
                context
                    .event_mgr()
                    .dispatcher()
                    .emit(&crate::script::event::PointerExit);
                context.ui_event_mgr_mut().handle_mouse_exit();

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                window_id: id,
            } if id == window_id => {
                let position = position.to_logical(context.screen_mgr().scale_factor());

                context
                    .event_mgr()
                    .dispatcher()
                    .emit(&crate::script::event::PointerMove {
                        pointer_x: position.x,
                        pointer_y: position.y,
                    });
                context
                    .ui_event_mgr_mut()
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
                        context
                            .event_mgr()
                            .dispatcher()
                            .emit(&crate::script::event::PointerDown {
                                button: button_name,
                            });
                        context.ui_event_mgr_mut().handle_mouse_button_down(button);
                    }
                    ElementState::Released => {
                        context
                            .event_mgr()
                            .dispatcher()
                            .emit(&crate::script::event::PointerUp {
                                button: button_name,
                            });
                        context.ui_event_mgr_mut().handle_mouse_button_up(button);
                    }
                }

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(inner_size),
                window_id: id,
            } if id == window_id => {
                context.screen_mgr_mut().update_size(&inner_size);
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
                context
                    .screen_mgr_mut()
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
            Event::LoopDestroyed => {}
            _ => return,
        }

        systems_pre_render();

        {
            let screen_mgr = context.screen_mgr();
            resize(
                screen_mgr.physical_width() as u32,
                screen_mgr.physical_height() as u32,
            );
        }

        clear();
        systems_render();

        gfx_context.swap_buffers().unwrap();
    });
}
