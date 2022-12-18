use mlua::{Lua, Table};

pub const BLACK: &str = "\u{001b}[30m";
pub const RED: &str = "\u{001b}[31m";
pub const GREEN: &str = "\u{001b}[32m";
pub const YELLOW: &str = "\u{001b}[33m";
pub const BLUE: &str = "\u{001b}[34m";
pub const MAGENTA: &str = "\u{001b}[35m";
pub const CYAN: &str = "\u{001b}[36m";
pub const WHITE: &str = "\u{001b}[37m";

pub const LIGHT_BLACK: &str = "\u{001b}[30;1m";
pub const LIGHT_RED: &str = "\u{001b}[31;1m";
pub const LIGHT_GREEN: &str = "\u{001b}[32;1m";
pub const LIGHT_YELLOW: &str = "\u{001b}[33;1m";
pub const LIGHT_BLUE: &str = "\u{001b}[34;1m";
pub const LIGHT_MAGENTA: &str = "\u{001b}[35;1m";
pub const LIGHT_CYAN: &str = "\u{001b}[36;1m";
pub const LIGHT_WHITE: &str = "\u{001b}[37;1m";

pub const RESET: &str = "\u{001b}[0m";

pub fn init(lua: &Lua) -> mlua::Result<Table> {
    let ansi = lua.create_table()?;
    ansi.set("black", BLACK)?;
    ansi.set("red", RED)?;
    ansi.set("green", GREEN)?;
    ansi.set("yellow", YELLOW)?;
    ansi.set("blue", BLUE)?;
    ansi.set("magenta", MAGENTA)?;
    ansi.set("cyan", CYAN)?;
    ansi.set("white", WHITE)?;

    ansi.set("light_black", LIGHT_BLACK)?;
    ansi.set("light_red", LIGHT_RED)?;
    ansi.set("light_green", LIGHT_GREEN)?;
    ansi.set("light_yellow", LIGHT_YELLOW)?;
    ansi.set("light_blue", LIGHT_BLUE)?;
    ansi.set("light_magenta", LIGHT_MAGENTA)?;
    ansi.set("light_cyan", LIGHT_CYAN)?;
    ansi.set("light_white", LIGHT_WHITE)?;

    ansi.set("reset", RESET)?;

    Ok(ansi)
}
