use std::thread;

pub fn side_lesson() {
    let outer_scope = 412;

    let join_handle = thread::spawn(move || {
        outer_scope * 2
    });

    let result = join_handle.join();

    match result {
        Ok(value) => {
            println!("{}", value);
        }
        Err(_) => {}
    }
}