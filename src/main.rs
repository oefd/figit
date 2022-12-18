use std::io;

use mlua::Lua;

mod output;
mod state;
mod util;

const BASE_FILE: &str = "base.lua";

fn main() {
    match main_() {
        Ok(_) => {}
        Err(err) => {
            match err.downcast_ref::<io::Error>() {
                Some(ioerr) if ioerr.kind() == io::ErrorKind::NotFound => {
                    eprintln!("error: unable to open {}", BASE_FILE);
                    std::process::exit(1);
                }
                _ => {
                    eprintln!("io error: {}", err);
                    std::process::exit(2);
                }
            };
        }
    }
}

fn main_() -> Result<(), Box<dyn std::error::Error>> {
    let lua = lua_init()?;
    let source = std::fs::read_to_string(BASE_FILE)?;
    lua.load(source.as_str()).exec().unwrap();

    Ok(())
}

fn lua_init() -> mlua::Result<Lua> {
    let lua = Lua::new();
    util::init(&lua)?;
    state::init(&lua)?;
    output::init(&lua)?;
    Ok(lua)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lua_preloads() {
        let lua = lua_init().unwrap();
        lua.load(r#"require("state")"#).exec().unwrap();
        lua.load(r#"require("util")"#).exec().unwrap();
    }
}
