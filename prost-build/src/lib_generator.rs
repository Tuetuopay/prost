use std::collections::BTreeMap;

use crate::{Config, Module};

#[derive(Debug)]
pub struct Mod {
    submodules: BTreeMap<String, Mod>,
    contents: Vec<Module>,
}

impl Mod {
    pub fn push(&mut self, module: &Module) {
        match module.as_slice() {
            [] => (),
            [name, left @ ..] => self.add(module.to_owned(), name, left),
        }
    }

    fn add(&mut self, module: Module, name: &str, left: &[String]) {
        let sub = self.submodules.entry(name.to_owned()).or_default();
        match left {
            [] => sub.contents.push(module),
            [name, left @ ..] => sub.add(module, name, left),
        }
    }
}

impl Default for Mod {
    fn default() -> Self {
        Self {
            submodules: BTreeMap::new(),
            contents: Vec::new(),
        }
    }
}

pub struct LibGenerator<'a> {
    config: &'a mut Config,
    depth: u8,
    buf: &'a mut String,
}

impl<'a> LibGenerator<'a> {
    pub fn generate_librs(config: &'a mut Config, mods: &Mod, buf: &'a mut String) {
        let mut generator = LibGenerator {
            config,
            depth: 0,
            buf,
        };
        generator.push_mod(mods);
    }

    fn push_mod(&mut self, mods: &Mod) {
        for (name, mods) in &mods.submodules {
            self.push_indent();
            self.buf.push_str("pub mod ");
            self.buf.push_str(name);
            self.buf.push_str(" {\n");
            self.depth += 1;
            self.push_mod(&mods);
            self.depth -= 1;
            self.push_indent();
            self.buf.push_str("}\n");
        }

        for package in mods.contents.iter().map(|content| content.join(".")) {
            self.push_indent();
            self.buf.push_str("include!(\"");
            self.buf.push_str(&package);
            self.buf.push_str(".rs\");\n");
        }
    }

    fn push_indent(&mut self) {
        for _ in 0..self.depth {
            self.buf.push_str("    ");
        }
    }
}
