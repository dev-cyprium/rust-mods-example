use colored::*;
use logger::log;
use mods::Mod;
use rlua::{Context, Error, Lua, Result, Value};
use std::io::Write;
use std::{fs, thread};

mod logger;
mod mods;

fn lua_print<'lua>(ctx: Context<'lua>, value: Value<'lua>) -> Result<()> {
    let now = logger::ts();
    let mut str = String::from("nil");

    if let Some(lua_str) = ctx.coerce_string(value)? {
        str = lua_str.to_str()?.to_string();
    }

    match writeln!(std::io::stdout(), "[{}] {}", now.cyan(), str) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::RuntimeError("Unable to write to stdout".to_string())),
    }
}

fn exec_lua(mods: Vec<Mod>) -> Result<()> {
    let lua_code = fs::read_to_string("game/main.lua").expect("Unable to read the Lua script");

    let lua = Lua::new();
    lua.context(|lua_ctx| {
        log("🔧 Loading Lua bindings");
        lua_ctx
            .globals()
            .set("__rust_bindings_print", lua_ctx.create_function(lua_print)?)?;

        let mods_table = mods::items_to_lua_table(&lua_ctx, mods)?;
        lua_ctx.globals().set("mods", mods_table)?;

        log("🚀 Executing Lua script");
        lua_ctx.load(&lua_code).exec()?;
        Ok(())
    })
}

fn main() {
    log("🌙 Starting Lua thread");
    let handle = thread::spawn(|| {
        let mods = mods::load();
        if let Err(e) = exec_lua(mods) {
            println!("Error ~~> {}", e);
        }
    });

    if let Err(e) = handle.join() {
        println!("Error: {:?}", e);
    }
}
