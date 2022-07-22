use crate::animation::{AnimationKeyFrame, AnimationTimeLine, Interpolatable};
use crate::component::*;
use crate::time::TimeManager;
use crate::transform::TransformManager;
use legion::*;

pub fn animate_sigle_animations(
    world: &mut World,
    time_mgr: &TimeManager,
    transform_mgr: &mut TransformManager,
) {
    let mut query = <(Entity, &mut SingleAnimator)>::query();
    let (mut world, mut rest) = world.split_for_query(&query);

    for (&entity, animator) in query.iter_mut(&mut world) {
        let animation = if let Some(animation) = &mut animator.animation {
            animation
        } else {
            continue;
        };

        let in_reverse = animation.pingpong && animator.is_pong;

        if in_reverse {
            animator.time -= time_mgr.dt() * animator.speed;
        } else {
            animator.time += time_mgr.dt() * animator.speed;
        }

        let normalized_time = animator.time / animation.duration;

        for time_line in &animation.time_lines {
            let key_frame =
                if let Some(key_frame) = find_key_frame(time_line, normalized_time, in_reverse) {
                    key_frame
                } else {
                    continue;
                };
            let normalized_time_in_key_frame =
                (normalized_time - key_frame.begin) / (key_frame.end - key_frame.begin);

            let transform = if let Ok(&transform) =
                rest.entry_ref(entity).unwrap().get_component::<Transform>()
            {
                transform.index()
            } else {
                continue;
            };
            let child_transform = if let Some(transform) = transform_mgr.find_child_by_name(
                transform,
                time_line
                    .transform
                    .as_ref()
                    .map(|transform| transform.as_slice()),
            ) {
                transform
            } else {
                continue;
            };
            let target_entity = transform_mgr.entity(child_transform);
            let target_entry = if let Ok(entry) = rest.entry_mut(target_entity) {
                entry
            } else {
                continue;
            };

            match time_line.component.as_str() {
                "transform" => {
                    let transform =
                        if let Ok(&transform) = target_entry.get_component::<Transform>() {
                            transform
                        } else {
                            continue;
                        };
                    let transform = transform_mgr.transform_mut(transform.index());

                    match time_line.field.as_str() {
                        "position.x" => {
                            transform.position.x = <f64 as Interpolatable>::interpolate(
                                key_frame.from.as_float(),
                                key_frame.to.as_float(),
                                normalized_time_in_key_frame,
                            ) as _;
                            transform.mark_as_dirty();
                        }
                        "position.y" => {
                            transform.position.y = <f64 as Interpolatable>::interpolate(
                                key_frame.from.as_float(),
                                key_frame.to.as_float(),
                                normalized_time_in_key_frame,
                            ) as _;
                            transform.mark_as_dirty();
                        }
                        "scale.x" => {
                            transform.scale.x = <f64 as Interpolatable>::interpolate(
                                key_frame.from.as_float(),
                                key_frame.to.as_float(),
                                normalized_time_in_key_frame,
                            ) as _;
                            transform.mark_as_dirty();
                        }
                        "scale.y" => {
                            transform.scale.y = <f64 as Interpolatable>::interpolate(
                                key_frame.from.as_float(),
                                key_frame.to.as_float(),
                                normalized_time_in_key_frame,
                            ) as _;
                            transform.mark_as_dirty();
                        }
                        "angle" => {
                            transform.angle = <f64 as Interpolatable>::interpolate(
                                key_frame.from.as_float(),
                                key_frame.to.as_float(),
                                normalized_time_in_key_frame,
                            ) as _;
                            transform.mark_as_dirty();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if in_reverse {
            if animator.time <= 0f32 {
                animator.time = 0f32;
                animator.is_pong = false;

                if !animation.looping {
                    animator.animation = None;
                }
            }
        } else {
            if animation.duration <= animator.time {
                if animation.pingpong {
                    animator.time = animation.duration;
                    animator.is_pong = true;
                } else {
                    animator.time = 0f32;
                    animator.is_pong = false;

                    if !animation.looping {
                        animator.animation = None;
                    }
                }
            }
        }
    }
}

fn find_key_frame(
    time_line: &AnimationTimeLine,
    normalized_time: f32,
    in_reverse: bool,
) -> Option<&AnimationKeyFrame> {
    if in_reverse {
        for key_frame in time_line.key_frames.iter() {
            if normalized_time <= key_frame.end {
                return Some(key_frame);
            }
        }
    } else {
        for key_frame in time_line.key_frames.iter().rev() {
            if key_frame.begin <= normalized_time {
                return Some(key_frame);
            }
        }
    }

    None
}
