use directories::BaseDirs;
use std::fs::create_dir;
use std::path::Path;

pub fn make_dirs()
{
    if let Some(base_dirs) = BaseDirs::new() 
    {
        let parent_dir = format!("{}\\Dogey11", base_dirs.data_local_dir().display());
        let dir = format!("{}\\QRRust", parent_dir);

        if !Path::new(&parent_dir).exists() 
        {
            create_dir(parent_dir).unwrap();
            create_dir(dir).unwrap();
        }
        else if Path::new(&parent_dir).exists() && !Path::new(&dir).exists()
        {
            create_dir(dir).unwrap();   
        }
    }
}

pub fn get() -> Result<String, ()>
{
    if let Some(base_dirs) = BaseDirs::new() 
    {
        let parent_dir = format!("{}\\Dogey11", base_dirs.data_local_dir().display());
        return Ok(format!("{}\\QRRust", parent_dir));
    }
    else {Err(())}
}

pub fn get_home() -> String
{
    if let Some(base_dirs) = BaseDirs::new() {return base_dirs.home_dir().to_str().unwrap().to_string()}
    "".to_string()
}