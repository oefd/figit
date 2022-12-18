use std::ffi::CString;

use mlua::{Function, Lua, Table};

mod outcome;
mod pacman;

pub use outcome::Outcome;

/// Preload the `state` module on the lua instance.
pub fn init(lua: &Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    let package: Table = globals.get("package")?;
    let loaded: Table = package.get("loaded")?;

    let state = lua.create_table()?;
    let pacman = pacman::init(&lua)?;
    state.set("pacman", pacman)?;

    loaded.set("state", state)?;

    Ok(())
}

/// Wraps state logic with common functionality.
fn state_fn(
    lua: &Lua,
    state_name: String,
    func: impl Fn(&Lua, &mut Outcome, Table) -> mlua::Result<()> + 'static,
) -> mlua::Result<Function> {
    lua.create_function(move |lua, args: Table| {
        let id = args.get::<&str, Option<String>>("id")?;
        let su = args.get::<&str, Option<String>>("su")?;

        let revert_eids = seteids_for_su(&su)?;

        let mut outcome = Outcome::new(id, state_name.clone());
        func(lua, &mut outcome, args)?;

        if let Some((uid, gid)) = revert_eids {
            seteuid(uid);
            setegid(gid);
        }

        Ok(outcome)
    })
}

fn seteids_for_su(su: &Option<String>) -> mlua::Result<Option<(libc::uid_t, libc::gid_t)>> {
    if let Some(su_user) = su {
        let old_ids = unsafe { (libc::geteuid(), libc::getegid()) };

        let (uid, gid) = getpwnam(su_user);
        seteuid(uid);
        setegid(gid);

        Ok(Some(old_ids))
    } else {
        Ok(None)
    }
}

fn getpwnam(user: &str) -> (libc::uid_t, libc::gid_t) {
    let user_cstring = {
        let mut vec = user.as_bytes().to_vec();
        vec.push(b'\0');
        CString::from_vec_with_nul(vec).unwrap()
    };

    let passwd_ptr = unsafe { libc::getpwnam(user_cstring.as_ptr()) };
    if passwd_ptr == 0 as *mut libc::passwd {
        panic!(
            "error getting passwd info for user {}: {}",
            user,
            strerror(unsafe { *libc::__errno_location() })
        );
    }
    unsafe { ((*passwd_ptr).pw_uid, (*passwd_ptr).pw_gid) }
}

fn seteuid(uid: libc::uid_t) {
    let errno = unsafe {
        *libc::__errno_location() = 0;
        libc::seteuid(uid)
    };
    if errno != 0 {
        panic!(
            "error setting euid {}: {}",
            uid,
            strerror(unsafe { *libc::__errno_location() })
        );
    }
}

fn setegid(gid: libc::gid_t) {
    let errno = unsafe {
        *libc::__errno_location() = 0;
        libc::setegid(gid)
    };
    if errno != 0 {
        panic!(
            "error setting egid {}: {}",
            gid,
            strerror(unsafe { *libc::__errno_location() })
        );
    }
}

fn strerror(errno: i32) -> String {
    let ptr = unsafe { libc::strerror(errno) };
    if ptr == 0 as *mut i8 {
        panic!("invalid errno number {}", errno);
    }
    let s = unsafe { CString::from_raw(ptr) };
    s.into_string().unwrap()
}
