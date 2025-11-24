use super::super::insert_dict_kv;
use anyhow::Result;
use starlark::{
    collections::SmallMap,
    const_frozen_string,
    values::{dict::Dict, Heap, Value},
};
use sysinfo::{System, SystemExt, User, UserExt};

fn create_dict_from_users<'a>(starlark_heap: &'a Heap, user: &User) -> Result<Dict<'a>> {
    let res: SmallMap<Value, Value> = SmallMap::new();
    let mut tmp_res: Dict<'_> = Dict::new(res);

    insert_dict_kv!(tmp_res, starlark_heap, "id", **user.id(), u32);
    insert_dict_kv!(tmp_res, starlark_heap, "name", user.name(), String);
    insert_dict_kv!(tmp_res, starlark_heap, "gid", *user.group_id(), u32);
    insert_dict_kv!(
        tmp_res,
        starlark_heap,
        "groups",
        Vec::from(user.groups()),
        Vec<_>
    );

    Ok(tmp_res)
}

pub fn get_users(starlark_heap: &Heap) -> Result<Vec<Dict>> {
    let mut final_res: Vec<Dict> = Vec::new();

    let sys = System::new_all();
    for user in sys.users() {
        let tmp_res: Dict<'_> = create_dict_from_users(starlark_heap, user)?;
        final_res.push(tmp_res);
    }
    Ok(final_res)
}
