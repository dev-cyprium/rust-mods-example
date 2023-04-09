use rlua::{Context, MetaMethod, UserData, UserDataMethods};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Mod {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

impl UserData for Mod {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |_, modd: &Mod, key: String| {
            match key.as_str() {
                "name" => Ok(modd.name.clone()),
                "version" => Ok(modd.version.clone()),
                "description" => Ok(modd.description.clone()),
                "author" => Ok(modd.author.clone()),
                _ => Err(rlua::Error::external(format!("unknown field '{}'", key))),
            }
        })
    }
}

pub fn items_to_lua_table<'lua>(
    ctx: &Context<'lua>,
    items: Vec<Mod>,
) -> rlua::Result<rlua::Table<'lua>> {
    let table = ctx.create_table()?;
    for (i, item) in items.iter().enumerate() {
        table.set(i + 1, item.clone())?;
    }
    Ok(table)
}

fn list_mods_root() -> Vec<String> {
    let mut mods = Vec::new();

    for entry in fs::read_dir("game/mods").expect("Unable to read the mods directory") {
        let entry = entry.expect("Unable to read the mods directory");
        let path = entry.path();

        if path.is_dir() {
            let mod_json_path = path.join("mod.json");
            if mod_json_path.exists() {
                mods.push(mod_json_path.to_str().unwrap().to_string());
            }
        }
    }

    mods
}

pub fn load() -> Vec<Mod> {
    let mod_paths = list_mods_root();
    let mut mods: Vec<Mod> = vec![];

    for mod_json_path in mod_paths {
        let mod_json = fs::read_to_string(mod_json_path).expect("Unable to read the mod.json file");
        let mod_json: Mod =
            serde_json::from_str(&mod_json).expect("Unable to parse the mod.json file");

        mods.push(mod_json);
    }

    mods
}
