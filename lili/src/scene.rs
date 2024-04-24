use std::ffi::OsString;

pub struct BasicScene {}

pub struct BasicSceneBuilder {}

impl BasicSceneBuilder {
    pub fn parse_files(filenames: Vec<OsString>) -> BasicScene {
        BasicScene {}
    }
}
