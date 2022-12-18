use mlua::{Function, Lua, Table};

use super::state::Outcome;
use super::util::ansi;

/// Preload the `output` module on the lua instance.
pub fn init(lua: &Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    let package: Table = globals.get("package")?;
    let loaded: Table = package.get("loaded")?;

    let output = lua.create_table()?;
    output.set("salt_like", salt_like(&lua)?)?;

    loaded.set("output", output)?;
    Ok(())
}

/// Saltstack-like outputter.
fn salt_like(lua: &Lua) -> mlua::Result<Function> {
    lua.create_function(move |_lua, outcome: Outcome| {
        if outcome.success {
            print!("{}", ansi::GREEN);
        } else {
            print!("{}", ansi::RED);
        }
        println!("----------");
        if let Some(id) = outcome.id {
            println!("{: >12} {}", "id:", id);
        }
        println!("{: >12} {}", "state:", outcome.state);
        println!("{: >12} {}", "success:", outcome.success);
        println!("{: >12} {}", "changes:", outcome.changes);
        if outcome.comments.len() == 1 {
            println!("{: >12} {}", "comment:", outcome.comments[0]);
        } else if outcome.comments.len() > 1 {
            println!("{: >12} {}", "comments:", outcome.comments[0]);
            for comment in outcome.comments.iter().skip(1) {
                println!("{: >12} {}", "", comment);
            }
        }
        print!("{}", ansi::RESET);
        Ok(())
    })
}
