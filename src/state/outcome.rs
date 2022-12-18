use mlua::{FromLua, Lua, ToLua, Value};

/// Outcome of a state application.
pub struct Outcome {
    pub id: Option<String>,
    pub state: String,
    pub success: bool,
    pub changes: bool,
    pub comments: Vec<String>,
}

impl Outcome {
    pub fn new(id: Option<String>, state: String) -> Self {
        Self {
            id,
            state,
            success: true,
            changes: false,
            comments: Vec::new(),
        }
    }

    pub fn add_comment(&mut self, comment: String) {
        self.comments.push(comment);
    }
}

impl<'lua> ToLua<'lua> for Outcome {
    fn to_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let table = lua.create_table()?;
        table.set("id", self.id)?;
        table.set("state", self.state)?;
        table.set("success", self.success)?;
        table.set("changes", self.changes)?;
        table.set("comments", self.comments)?;
        Ok(Value::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Outcome {
    fn from_lua(lua_value: Value<'lua>, _lua: &'lua Lua) -> mlua::Result<Self> {
        if let Value::Table(table) = lua_value {
            let id = table.get::<&str, Option<String>>("id")?;
            let state = table.get::<&str, String>("state")?;
            let success = table.get::<&str, bool>("success")?;
            let changes = table.get::<&str, bool>("changes")?;
            let comments = table.get::<&str, Vec<String>>("comments")?;
            Ok(Self {
                id,
                state,
                success,
                changes,
                comments,
            })
        } else {
            Err(mlua::Error::RuntimeError(
                "Outcome values must be a table".to_string(),
            ))
        }
    }
}
