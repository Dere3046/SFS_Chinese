use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let profile_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();
    let assets = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");

    let src = assets.join("BepInEx_win_x64_5.4.23.5");
    let dst = profile_dir.join("BepInEx_win_x64_5.4.23.5");
    if src.exists() && !dst.exists() {
        copy_dir_all(&src, &dst).ok();
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for e in fs::read_dir(src)? {
        let e = e?;
        if e.file_type()?.is_dir() {
            copy_dir_all(&e.path(), &dst.join(e.file_name()))?;
        } else {
            fs::copy(e.path(), dst.join(e.file_name()))?;
        }
    }
    Ok(())
}
