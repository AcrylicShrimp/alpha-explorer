use rodio::{OutputStream, OutputStreamHandle};

pub struct AudioChannel {
    handle: OutputStreamHandle,
    _stream: OutputStream,
}

impl AudioChannel {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();
        Self {
            handle,
            _stream: stream,
        }
    }

    pub fn handle(&self) -> &OutputStreamHandle {
        &self.handle
    }
}
