use ctru::applets::swkbd::{Button, Swkbd};
use ctru::prelude::*;

fn print_help(){
    println!("Press start to exit");
    println!("Press A to open a keyboard");
    println!("Press B to switch screen");
}

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();

    // Start a console on the top screen
    let top_screen = Console::new(gfx.top_screen.borrow_mut());

    // Start a console on the bottom screen.
    // The most recently initialized console will be active by default
    let bottom_screen = Console::new(gfx.bottom_screen.borrow_mut());

    // Let's print on the top screen first
    top_screen.select();
    println!("Luna's very useful program V69.4.20");
    print_help();
    let mut top_selected = true;
    
    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        if hid.keys_down().contains(KeyPad::A){
            //Open keyboard and print typed thing to top screen
            let mut keyboard = Swkbd::default();

            match keyboard.get_string(2048){
                Ok((text, Button::Right)) => {
                    if text == "help"{
                        print_help();
                    } else {
                        println!("{text}");
                    }
                },
                Ok((_, Button::Left)) => println!("You closed the Keyboard"),
                Ok((_, Button::Middle)) => println!("How in the fuck"),
                Err(_) => println!("Shit went to fuck, L"),
            }
        }

        if hid.keys_down().contains(KeyPad::B){
            if top_selected {
                println!("Bottom Screen Selected");
                bottom_screen.select();
                top_selected = false;
            } else {
                println!("Top screen selected");
                top_screen.select();
                top_selected = true;
            }
            }
    }
}
