use ctru::applets::swkbd::{Button, Swkbd};
use ctru::prelude::*;

fn main(){
    ctru::use_panic_handler();

    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();

    // Start a console on the top screen
    let top_screen = Console::new(gfx.top_screen.borrow_mut());
    top_screen.select();

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        print!("{}", hid.circlepad_position());
    }
}
