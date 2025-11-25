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

    #[cfg(target_os = "windows")]
    insert_dict_kv!(tmp_res, starlark_heap, "id", &*user.id().to_string(), u32);

    #[cfg(not(target_os = "windows"))]
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Ok;

    #[test]
    fn test_sys_get_users() -> anyhow::Result<()> {
        let test_heap = Heap::new();
        let res: Vec<Dict<'_>> = get_users(&test_heap)?;

        // make sure it is not empty
        assert!(!res.is_empty());

        // using the first dict for testing
        let keys: Vec<&str> = res
            .first()
            .unwrap()
            .keys()
            .map(|key| key.unpack_str().unwrap())
            .collect();
        assert!(keys.contains(&"id"));
        assert!(keys.contains(&"name"));
        assert!(keys.contains(&"gid"));
        assert!(keys.contains(&"groups"));

        Ok(())
    }
}
