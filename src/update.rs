use isahc::prelude::*;
use webbrowser;

const BUILD: i32 = 1;
const LATEST: &str = "http://dogey11.me/api/ver/code";
const DL_LINK: &str = "http://github.com/Dogey11/QRust/releases/latest";

pub fn is_out_of_date() -> Result<bool, isahc::Error>
{
    let response: Result<String, std::io::Error> = isahc::get(LATEST)?.text();
    
    if response.is_ok()
    {
        let latest_build: String = response.unwrap();
        let to_int: &String = &latest_build.chars().filter(|c| c.is_digit(10)).collect();
        let build_int: i32 = to_int.parse().unwrap();
        //println!("Current build number:\n{}\nLatest build number:\n{}", BUILD, latest_build);
        return Ok(build_int > BUILD);
    }
    else {return Ok(false)}
}

pub fn download()
{
    webbrowser::open(DL_LINK).unwrap();
}