#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod qrcode;
mod update;
mod dirs;

use fltk::{*, prelude::*, enums::*};
use qrcode::*;
use update::*;
use dirs::*;


fn center() -> (i32, i32) 
{
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}


fn prompt_update_gui() -> Result<(), Box<dyn std::error::Error>>
{
    let app = app::App::default();
    let mut wind = window::Window::default()
    .with_size(200, 200)
    .with_pos(center().0 - 100, center().1 - 50);

    let mut frame = frame::Frame::default().size_of(&wind);
    frame.set_label("A newer version is available.\n\nInstall it now?");

    let mut yes = button::Button::new(10, 150, 80, 40, "Yes");
    yes.set_callback(move |_| {app.quit(); download()});

    let mut no = button::Button::new(110, 150, 80, 40, "No");
    no.set_callback(move |_| {app.quit(); main_gui().unwrap()});

    wind.show();
    app.run().unwrap();
    Ok(())
}


fn main_gui() -> Result<(), Box<dyn std::error::Error>> 
{
    // Image Location
    let image_loc = String::from(format!("{}\\image.png", get().unwrap()));
    let image_loc_clone = image_loc.clone();


    // File Dialog
    let mut askfile = dialog::FileDialog::new(fltk::dialog::FileDialogType::BrowseSaveFile);
    askfile.set_preset_file(&image_loc_clone);
    askfile.set_filter("*.png");
    askfile.show();


    // GUI
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300);
    let mut frame = frame::Frame::default().size_of(&wind);


    // Image
    let mut image = image::SharedImage::load(image_loc)?;
    image.scale(64, 64, true, true);
    frame.set_image(Some(image));

    let mut input = input::Input::new(10, 10, 60, 20, "Joe");
    
    let (s, r) = app::channel::<bool>();

    input.set_trigger(CallbackTrigger::Changed);
    input.emit(s, true);


    wind.make_resizable(false);
    wind.show();


    while app.wait()
    {
        match r.recv()
        {
            Some(msg) =>
            {
                if msg && input.value() != ""
                {
                    qr_code(&input.value(), &image_loc_clone);
                    image = image::SharedImage::load(&image_loc_clone)?;
                    frame.set_image(Some(image));
                    println!("{}", input.value());
                    frame.redraw();
                }
            }
            None => (),
        }
    }
    Ok(())
}


fn main()
{
    make_dirs();
    
    let ofd = is_out_of_date();
    let outdated = if ofd.is_ok() {ofd.unwrap()} else {false};

    if outdated {prompt_update_gui().unwrap();}
    else {main_gui().unwrap();}
}