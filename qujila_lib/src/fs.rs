use std::{
    fs,
    env,
    format as f,
    borrow::Cow,
    path::{PathBuf, Path},
};


pub fn schema_file_path() -> Result</*absolute*/PathBuf, Cow<'static, str>> {
    read_dir(&qujila_dir_path()?)?
        .find(|f| f.is_file() && &f.file_name().unwrap().to_string_lossy() == "schema.prisma")
        .ok_or_else(|| Cow::Borrowed("`qujila/schema.prisma` was not found"))
}

pub fn migration_dir_path() -> Result</*absolute*/PathBuf, Cow<'static, str>> {
    let path = {let mut p = qujila_dir_path()?; p.push("migrations"); p};
    if !path.exists() {
        fs::create_dir(&path).map_err(|e| Cow::Owned(f!("Failed to create migration directory: {e}")))?;
    }
    Ok(path)
}

pub fn migration_version_file_path() -> Result</*absolute*/PathBuf, Cow<'static, str>> {
    let path = {let mut p = migration_dir_path()?; p.push("version.txt"); p};
    if !path.exists() {
        fs::File::create(&path).map_err(|e| Cow::Owned(f!("Failed to create file: {e}")))?;
    }
    Ok(path)
}

pub fn qujila_dir_path() -> Result</*absolute*/PathBuf, Cow<'static, str>> {
    read_dir(&project_root()?)?
        .find(|f| f.is_dir() && &f.file_name().unwrap().to_string_lossy() == "qujila")
        .ok_or_else(|| Cow::Borrowed("No qujila directory found"))
}

pub fn project_root() -> Result</*absolute*/PathBuf, Cow<'static, str>> {
    let mut cd = env::current_dir().map_err(|e| Cow::Owned(f!("Can't get current directory: {e}")))?;
    loop {
        if read_dir(&cd)?.any(|f| f.is_file() && &f.file_name().unwrap().to_string_lossy() == "Cargo.toml") {
            return Ok(cd)
        } else {
            let parent = cd.parent().ok_or_else(|| Cow::Borrowed("No Cargo.toml found"))?;
            cd = parent.to_path_buf()
        }
    }
}




fn read_dir(path: impl AsRef<Path>) -> Result<impl Iterator<Item = PathBuf>, Cow<'static, str>> {
    Ok(fs::read_dir(path.as_ref())
        .map_err(|e| Cow::Owned(f!("Can't read directory `{}`: {e}", path.as_ref().display())))?
        .collect::<std::io::Result<Vec<_>>>()
        .map_err(|e| Cow::Owned(f!("Can't read a file or directory in `{}`: {e}", path.as_ref().display())))?
        .into_iter().map(|entry| entry.path()))
}
