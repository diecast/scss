extern crate diecast;

use std::path::PathBuf;
use std::process::Command;

use diecast::{Handle, Bind};

pub struct Scss {
    input: PathBuf,
    output: PathBuf,
}

// TODO
// this should probably use a rule for watching scss
// and a separte rule css that depends on it for
// generating a single item?
impl Handle<Bind> for Scss {
    fn handle(&self, bind: &mut Bind) -> diecast::Result<()> {
        let conf = &bind.data().configuration;
        let source = conf.input.join(&self.input);
        let destination = conf.output.join(&self.output);

        if let Some(parent) = destination.parent() {
            try!(diecast::support::mkdir_p(parent));
        }

        // let mut command = Command::new("scss");
        let mut command = Command::new("./sassc-3.2.5/bin/sassc");

        if let Some(load_path) = source.parent() {
            command.arg("-I").arg(load_path.to_path_buf());
        }

        command.arg(source).arg(destination);

        try!(command.status());

        Ok(())
    }
}

#[inline]
pub fn scss<P, Q>(input: P, output: Q) -> Scss
where P: Into<PathBuf>, Q: Into<PathBuf> {
    Scss {
        input: input.into(),
        output: output.into(),
    }
}

