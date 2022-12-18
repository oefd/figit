use mlua::{Lua, Table};

mod table;

/// Preload the `util` module on the lua instance.
pub fn init(lua: &Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    let package: Table = globals.get("package")?;
    let loaded: Table = package.get("loaded")?;

    let util = lua.create_table()?;
    util.set("table", table::init(&lua)?)?;

    loaded.set("util", util)?;
    Ok(())
}
