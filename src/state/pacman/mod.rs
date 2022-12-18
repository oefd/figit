use mlua::{Lua, Table};

mod alpm;

use super::{state_fn, Outcome};

pub fn init(lua: &Lua) -> mlua::Result<Table> {
    let pacman = lua.create_table()?;
    pacman.set(
        "installed",
        state_fn(lua, "pacman.installed".to_string(), installed())?,
    )?;

    Ok(pacman)
}

/// Ensure a set of pacman packages are installed.
fn installed() -> impl Fn(&Lua, &mut Outcome, Table) -> mlua::Result<()> {
    |_lua, outcome: &mut Outcome, args: Table| {
        if let Ok(pkgs) = args.get::<&str, Table>("pkgs") {
            let pkgs: Vec<String> = pkgs
                .sequence_values::<String>()
                .map(Result::unwrap)
                .collect();

            let newly_installed = alpm::ensure_installed(&pkgs);

            if newly_installed.is_empty() {
                outcome.changes = false;
            } else {
                outcome.changes = true;
                outcome.add_comment("installed packages".to_string());
                for pkg in newly_installed.iter() {
                    outcome.add_comment(format!("* {}", pkg));
                }
            };
            outcome.success = true;
        } else {
            outcome.success = false;
            outcome.changes = false;
            outcome.add_comment("missing required argument 'pkgs'".to_string());
        }
        Ok(())
    }
}
