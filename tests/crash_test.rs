use std::{process::Command, time::Duration};

#[test]
fn crash() {
    let crash_bin = env!("CARGO_BIN_EXE_db-crash");
    for i in 0..500 {
        println!("------------------ try crash {i} ------------------");
        let mut child = Command::new(crash_bin).spawn().unwrap();
        for _ in 0..100 + rand::random::<i32>() % 50 {
            if let Ok(Some(res)) = child.try_wait() {
                if !res.success() {
                    return;
                }
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        child.kill().expect("kill");
        child.wait().ok();
    }
}
