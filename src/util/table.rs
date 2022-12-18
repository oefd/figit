use mlua::{Function, Lua, Table, Value};

pub fn init(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("len", len(&lua)?)?;
    table.set("merge", merge(&lua)?)?;

    Ok(table)
}

/// Count of elements in a table.
fn len(lua: &Lua) -> mlua::Result<Function> {
    lua.create_function(move |_lua, tbl: Table| Ok(tbl.clone().pairs::<Value, Value>().count()))
}

/// Return a new table which is the union of two other tables.
fn merge(lua: &Lua) -> mlua::Result<Function> {
    lua.create_function(move |lua, args: (Table, Table)| {
        let (t1, t2) = args;
        let merged = lua.create_table()?;
        for pair in t1.pairs::<Value, Value>() {
            let (key, value) = pair?;
            merged.set(key, value)?;
        }
        for pair in t2.pairs::<Value, Value>() {
            let (key, value) = pair?;
            merged.set(key, value)?;
        }

        Ok(merged)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn table_len() {
        let lua = Lua::new();
        lua.globals().set("len", len(&lua).unwrap()).unwrap();

        let res: i32 = lua.load(r#"len({})"#).eval().unwrap();
        assert!(res == 0);
        let res: i32 = lua.load(r#"len({1})"#).eval().unwrap();
        assert!(res == 1);
        let res: i32 = lua.load(r#"len({1, one = 1 })"#).eval().unwrap();
        assert!(res == 2);
    }

    #[test]
    fn merge_returns_new_table() {
        let lua = Lua::new();
        lua.globals().set("merge", merge(&lua).unwrap()).unwrap();

        let checks: bool = lua
            .load(
                r#"
                    local a = { one = 1, two = 2 }
                    local b = { three = 3, four = 4 }
                    local res = merge(a, b)
                    return a["three"] == nil
                        and b["two"] == nil
                        and res["one"] == 1
                        and res["three"] == 3
                "#,
            )
            .eval()
            .unwrap();

        assert!(checks);
    }
}
