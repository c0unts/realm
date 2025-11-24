use super::super::insert_dict_kv;
use anyhow::Result;
use starlark::collections::SmallMap;
use starlark::const_frozen_string;
use starlark::values::dict::Dict;
use starlark::values::Heap;
use sysinfo::{System, SystemExt, UserExt};

pub fn get_users(starlark_heap: &Heap) -> Result<Dict> {
    let res = SmallMap::new();
    let mut dict_res = Dict::new(res);

    let sys = System::new_all();
    for user in sys.users() {
        insert_dict_kv!(dict_res, starlark_heap, "id", **user.id(), u32);
        insert_dict_kv!(dict_res, starlark_heap, "name", user.name(), String);
        insert_dict_kv!(dict_res, starlark_heap, "gid", *user.group_id(), u32);
        insert_dict_kv!(
            dict_res,
            starlark_heap,
            "groups",
            Vec::from(user.groups()),
            Vec<_>
        );
        Ok(());
    }

    return Ok(dict_res);
}
