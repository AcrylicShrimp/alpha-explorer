use crate::asset::*;
use crate::emit_diagnostic_info;
use crate::event::*;
use crate::log_diagnostic_event;
use crate::script::event::Diagnostic;
use crate::structure::Vec2;
use crate::system::*;
use crate::EngineContext;
use crate::GfxContext;
use anyhow::Context;
use anyhow::Result;
use specs::RunNow;
use std::fs::read_to_string;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use winit::window::WindowBuilder;

static mut CONTEXT: MaybeUninit<Arc<EngineContext>> = MaybeUninit::uninit();

pub fn use_context() -> &'static EngineContext {
    unsafe { CONTEXT.assume_init_ref() }.as_ref()
}

pub async fn run(
    title: &str,
    width: u32,
    height: u32,
    resizable: bool,
    asset_base: impl Into<PathBuf>,
    entry_script_path: impl AsRef<Path>,
    once_engine_initialized: impl FnOnce(&Window, &EngineContext) -> Result<()>,
) -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .with_resizable(resizable)
        .with_inner_size(LogicalSize::new(width, height))
        .build(&event_loop)?;

    let gfx_context = GfxContext::new(&window).await?;
    let context = Arc::new(EngineContext::new(
        gfx_context,
        width,
        height,
        asset_base.into(),
    )?);

    unsafe {
        CONTEXT.write(context.clone());
    }

    #[cfg(debug_assertions)]
    {
        context.event_mgr().add_handler(
            Diagnostic::name(),
            EventHandler::native(|event| {
                if let Some(event) = event.downcast_ref::<Diagnostic>() {
                    log_diagnostic_event(event);
                }
                Ok(())
            }),
        );
    }

    emit_diagnostic_info!(format!("configuring built-in systems."));

    let mut audio_system = AudioSystem;
    let mut render_system = RenderSystem::new(&mut context.render_mgr_mut());

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

            context.event_mgr().emit(
                &crate::script::event::PreUpdate {
                    dt: context.time_mgr().dt_f64(),
                },
                context.script_mgr().lua(),
            );
            context.event_mgr().emit(
                &crate::script::event::Update {
                    dt: context.time_mgr().dt_f64(),
                },
                context.script_mgr().lua(),
            );
            context.event_mgr().emit(
                &crate::script::event::PostUpdate {
                    dt: context.time_mgr().dt_f64(),
                },
                context.script_mgr().lua(),
            );
        }
    };

    let mut systems_render = {
        let context = context.clone();
        move |skip_render: bool| {
            context.event_mgr().emit(
                &crate::script::event::PreRender {
                    dt: context.time_mgr().dt_f64(),
                },
                context.script_mgr().lua(),
            );

            if !skip_render {
                context.render_mgr().update_uniforms(&context);
                render_system.run_now(&context.world());
            }

            context.event_mgr().emit(
                &crate::script::event::PostRender {
                    dt: context.time_mgr().dt_f64(),
                },
                context.script_mgr().lua(),
            );

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
        // asset_mgr.register_loader(loader::sprite_atlas_loader());
        // asset_mgr.register_loader(loader::sprite_atlas_grid_loader());
        // asset_mgr.register_loader(loader::tilemap_loader());
    }

    {
        emit_diagnostic_info!(format!("abjusting scale factor."));

        let scale_factor = window.scale_factor();
        let mut screen_mgr = context.screen_mgr_mut();
        let mut render_mgr = context.render_mgr_mut();
        screen_mgr.update_scale_factor(
            scale_factor,
            LogicalSize::new(width, height).to_physical(scale_factor),
        );
        render_mgr.resize_gfx(PhysicalSize::new(
            screen_mgr.physical_width() as u32,
            screen_mgr.physical_height() as u32,
        ));
    }

    {
        emit_diagnostic_info!(format!("executing engine initialization callback."));
        once_engine_initialized(&window, &context).with_context(|| "failed to execute callback")?;
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

    window.set_visible(true);

    let window_id = window.id();
    let mut window_occluded = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                systems_pre_render();
                systems_render(window_occluded);

                if window_occluded {
                    sleep(Duration::from_millis(60));
                }

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::Occluded(occluded),
                window_id: id,
            } if id == window_id => {
                window_occluded = occluded;

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                window_id: id,
            } if id == window_id => {
                context.input_mgr_mut().handle_event(&input);

                if let Some(key) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            context.event_mgr().emit(
                                &crate::script::event::KeyDown::from_key(key),
                                context.script_mgr().lua(),
                            );
                        }
                        ElementState::Released => {
                            context.event_mgr().emit(
                                &crate::script::event::KeyUp::from_key(key),
                                context.script_mgr().lua(),
                            );
                        }
                    }
                }

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorEntered { .. },
                window_id: id,
            } if id == window_id => {
                context.event_mgr().emit(
                    &crate::script::event::PointerEnter,
                    context.script_mgr().lua(),
                );

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorLeft { .. },
                window_id: id,
            } if id == window_id => {
                context.event_mgr().emit(
                    &crate::script::event::PointerExit,
                    context.script_mgr().lua(),
                );
                context.ui_event_mgr_mut().handle_mouse_exit();

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                window_id: id,
            } if id == window_id => {
                let position = position.to_logical(context.screen_mgr().scale_factor());

                context.event_mgr().emit(
                    &crate::script::event::PointerMove {
                        pointer_x: position.x,
                        pointer_y: position.y,
                    },
                    context.script_mgr().lua(),
                );
                context
                    .ui_event_mgr_mut()
                    .handle_mouse_move(Vec2::new(position.x as f32, position.y as f32));

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
                        context.event_mgr().emit(
                            &crate::script::event::PointerDown {
                                button: button_name,
                            },
                            context.script_mgr().lua(),
                        );
                        context.ui_event_mgr_mut().handle_mouse_button_down(button);
                    }
                    ElementState::Released => {
                        context.event_mgr().emit(
                            &crate::script::event::PointerUp {
                                button: button_name,
                            },
                            context.script_mgr().lua(),
                        );
                        context.ui_event_mgr_mut().handle_mouse_button_up(button);
                    }
                }

                return;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(inner_size),
                window_id: id,
            } if id == window_id => {
                context.screen_mgr_mut().update_size(inner_size);
                context.render_mgr_mut().resize_gfx(inner_size);

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
                    .update_scale_factor(scale_factor, *new_inner_size);
                context.render_mgr_mut().resize_gfx(*new_inner_size);

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
    });
}
