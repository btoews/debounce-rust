use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::event::{
    CGEventField, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
    CGEventType,
};
use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, SystemTime},
};

const REPEAT_LIMIT: Duration = Duration::from_millis(100);
const WARN_LIMIT: Duration = Duration::from_millis(200);

const FIELD_AUTO_REPEAT: CGEventField = 8;
const FIELD_KEY_CODE: CGEventField = 9;
const FIELD_KEYBOARD_TYPE: CGEventField = 10;

const SYNTHETIC_KB_ID: i64 = 666;

fn main() {
    let key_times = Mutex::new(HashMap::new());

    let tap = CGEventTap::new(
        CGEventTapLocation::Session,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        vec![CGEventType::KeyDown],
        move |_, _, e| {
            if !matches!(e.get_type(), CGEventType::KeyDown) {
                return None;
            }

            if e.get_integer_value_field(FIELD_KEYBOARD_TYPE) == SYNTHETIC_KB_ID {
                return None;
            }

            if e.get_integer_value_field(FIELD_AUTO_REPEAT) == 1 {
                return None;
            }

            let key_code = e.get_integer_value_field(FIELD_KEY_CODE);
            let now = SystemTime::now();

            if let Some(last) = key_times.lock().unwrap().insert(key_code, now) {
                let dur = now.duration_since(last).unwrap();

                if dur < REPEAT_LIMIT {
                    println!(
                        "Bounce prevented key_code={:?} delay={}ms",
                        key_code,
                        dur.as_millis()
                    );

                    // we should be able to drop the event by returning None,
                    // but the rust core-foundation library interprets None as a
                    // directive to keep the original event...
                    e.set_type(CGEventType::Null);
                } else if dur < WARN_LIMIT {
                    println!(
                        "Bounce detected key_code={:?} delay={}ms",
                        key_code,
                        dur.as_millis()
                    );
                }
            }

            None
        },
    )
    .expect("failed to create tap");

    tap.enable();

    let loop_source = tap
        .mach_port
        .create_runloop_source(0)
        .expect("failed to get runloop source");

    CFRunLoop::get_current().add_source(&loop_source, unsafe { kCFRunLoopCommonModes });

    println!("tap installed");
    CFRunLoop::run_current();
}
