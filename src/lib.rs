extern crate diecast;
extern crate sass_rs;

use std::path::PathBuf;

use sass_rs::sass_context::SassFileContext;

use diecast::{Handle, Bind, Item};

pub struct Scss {
    input: PathBuf,
    output: PathBuf,
}

impl Handle<Bind> for Scss {
    fn handle(&self, bind: &mut Bind) -> diecast::Result<()> {
        let source = bind.data().configuration.input.join(&self.input);
        let destination = bind.data().configuration.output.join(&self.output);

        let parent = try! {
            destination.parent()
            .ok_or(format!("[SCSS] path has no parent: {:?}", destination))
        };

        try!(diecast::support::mkdir_p(parent));

        let load_path = try! {
            source.parent()
            .ok_or(format!("[SCSS] path has no parent: {:?}", source))
        };

        let source_str = try! {
            source.to_str()
            .ok_or(format!("[SCSS] path is not UTF-8: {:?}", source))
        };

        let mut file_context = SassFileContext::new(source_str);

        {
            file_context.sass_context.sass_options
            .write().unwrap().set_include_paths(&[load_path]);
        }

        let mut css = Item::writing(&destination);
        css.body = try!(file_context.compile());
        bind.attach(css);

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

