use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::env::{var as env_var, VarError};
use std::fs::File;
use std::io::Error as IOError;
use std::path::Path;

#[derive(Debug)]
enum Error {
    VarError(VarError),
    IOError(IOError),
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        Self::VarError(err)
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}

fn main() -> Result<(), Error> {
    let dest = env_var("OUT_DIR")?;
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)?;

    Ok(())
}
