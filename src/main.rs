use ctru::applets::swkbd::{Button, Swkbd};
use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();

    // Start a console on the top screen
    let top_screen = Console::new(gfx.top_screen.borrow_mut());
    top_screen.select();

    let mut last_circlepad = (0, 0);

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        let current_circlepad = hid.circlepad_position();
        if current_circlepad != last_circlepad {
            last_circlepad = current_circlepad;
            println!("{}, {}", last_circlepad.0, last_circlepad.1);
        }
    }
}
