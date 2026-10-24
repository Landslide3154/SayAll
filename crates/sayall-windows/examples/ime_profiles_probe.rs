//! 列出系统已安装的键盘输入法（设置页「键盘输入法」下拉的数据源探针）。
//!
//! 用法：`cargo run -p sayall-windows --example ime_profiles_probe`
//!
//! 输出每行：`名称<TAB>CLSID<TAB>Profile GUID<TAB>LangID<TAB>模块目录`。
//! 用于真机确认目标输入法（如"清风输入法"）的三元组，并核对
//! `ime::keyboard_target_from_parts` 能否解析。

fn main() {
    match sayall_windows::list_installed_ime_profiles() {
        Ok(profiles) => {
            for profile in profiles {
                println!(
                    "{}\t{}\t{}\t0x{:04X}\t{}",
                    profile.name,
                    profile.clsid,
                    profile.profile_guid,
                    profile.lang_id,
                    profile.module_dir.as_deref().unwrap_or("-")
                );
            }
        }
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}
