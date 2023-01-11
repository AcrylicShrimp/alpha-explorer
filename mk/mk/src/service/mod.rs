use crossbeam::{channel::unbounded, select};
use std::thread::spawn;

mod gpu_service;

pub fn test() {
    let (_, object_request) = unbounded::<usize>();
    let (_, render_request) = unbounded::<usize>();

    spawn(move || {
        select! {
            recv(object_request) -> object_request => {

            }
            recv(render_request) -> render_request => {

            }
        }
    });
}
