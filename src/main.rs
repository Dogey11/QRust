#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod qrcode;
mod update;

use fltk::
{app, button::Button, frame::Frame, image::SharedImage, input::Input, prelude::*, window::Window, enums::CallbackTrigger};
use std::{error::Error, path::Path};
use update::{is_out_of_date, download};
use qrcode::qr_code;

fn center() -> (i32, i32) 
{
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}

fn prompt_update_gui() -> Result<bool, Box<dyn Error>>
{
    let update;

    let app = app::App::default();
    let mut wind = Window::default()
    .with_size(200, 200)
    .with_pos(center().0 - 100, center().1 - 50);

    let mut frame = Frame::default().size_of(&wind);
    frame.set_label("A newer version is available.\n\nInstall it now?");

    let mut yes = Button::new(10, 150, 80, 40, "Yes");
    yes.set_callback(move |_| frame.set_label("Hello World!"));

    let mut no = Button::new(10, 150, 80, 40, "Yes");
    no.set_callback();

    wind.show();
    app.run().unwrap();
    Ok(false)
}


fn main_gui() -> Result<(), Box<dyn Error>> 
{
    qr_code("amogus");

    let mut outdated = false;
    let ofd = is_out_of_date();
    
    if ofd.is_ok() {outdated = ofd.unwrap();}

    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);
    let mut frame = Frame::default().size_of(&wind);

    let mut image = SharedImage::load("file.png")?;
    let mut input = Input::new(10, 10, 60, 20, "Joe");

    image.scale(64, 64, true, true);
    frame.set_image(Some(image));
    
    let (s, r) = app::channel::<bool>();

    input.set_trigger(CallbackTrigger::Changed);
    input.emit(s, true);


    wind.make_resizable(true);
    wind.show();


    while app.wait()
    {
        match r.recv()
        {
            Some(msg) =>
            {
                if msg
                {
                    image = SharedImage::load("file.png")?;
                    frame.set_image(Some(image));
                    println!("{}", input.value());
                    frame.redraw();
                    app.redraw();
                }
            }
            None => (),
        }
    }
    Ok(())
}


fn main()
{
    let ofd = is_out_of_date();
    //let outdated = if ofd.is_ok() {ofd.unwrap()} else {false};
    let outdated = true;

    if outdated
    {
        prompt_update_gui();
    }   
}