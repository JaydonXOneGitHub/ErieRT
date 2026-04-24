use mlua::{
    Function, Lua, UserData, Value as LuaValue
};

use crate::{luaapi::LuaType, macros::macros::{add_docs, wipe_type}, mainapi::SharedEngineAPI};

pub struct Match {
    internal: Option<Vec<(LuaValue, LuaValue)>>,
    fallback: LuaValue
}

impl Match {
    pub fn new() -> Self {
        return Self {
            internal: Vec::new().into(),
            fallback: LuaValue::Nil
        };
    }
}

impl Match {
    fn get_value(&mut self, _: &Lua, match_value: LuaValue) -> mlua::Result<LuaValue> {
        let internal = self.internal.take();

        if internal.is_none() {
            return Result::Err(mlua::Error::RuntimeError(
                "Match instance is invalid!".into()
            ));
        }

        for pair in internal.unwrap().iter() {
            let (k, v) = pair;

            if match_value.is_table() && k.is_table() {
                let match_table = match_value.as_table().unwrap();

                let res = match_table.get::<Function>("compare");

                match res {
                    Result::Ok(fun) => {
                        let res = fun.call::<bool>((match_value.clone(), k));

                        if let Result::Err(err) = res {
                            return Result::Err(err);
                        };

                        let equal = res.ok().unwrap();

                        if equal {
                            return Result::Ok(v.clone());
                        }
                    },
                    Result::Err(_) => {}
                }
            }

            let key = k.clone();

            if match_value == key {
                return Result::Ok(v.clone());
            }
        }

        return Result::Ok(self.fallback.clone());
    }
}

impl UserData for Match {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("push", |_, this, (key, value): (LuaValue, LuaValue)| {
            let internal = this.internal.as_mut();

            match internal {
                Option::Some(vec) => vec.push((key, value)),
                Option::None => {
                    return Result::Err(mlua::Error::RuntimeError(
                        "Match instance is invalid!".into()
                    ));
                }
            }

            return Result::Ok(());
        });

        methods.add_method_mut("setFallback", |_, this, fallback: LuaValue| {
            if this.internal.is_none() {
                return Result::Err(mlua::Error::RuntimeError(
                    "Match instance is invalid!".into()
                ));
            }

            this.fallback = fallback;
            return Result::Ok(());
        });

        methods.add_method_mut("exec", |lua, this, value: LuaValue| {
            if this.internal.is_none() {
                return Result::Err(mlua::Error::RuntimeError(
                    "Match instance is invalid!".into()
                ));
            }
            return this.get_value(lua, value);
        });
    }
}

impl LuaType for Match {
    fn make_global_table(lua: &mlua::Lua, globals: Option<mlua::Table>, _: SharedEngineAPI) -> mlua::Result<()> {
        let res = lua.create_table();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let m = res.ok().unwrap();

        let res = lua.create_function(|_, ()| {
            return Result::Ok(Self::new());
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let new = res.ok().unwrap();

        let res = m.set("new", new);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let globals = globals.unwrap_or(lua.globals().clone());

        return globals.set("Match", m);
    }

    wipe_type!(Match);

    add_docs!(luamatch);
}
