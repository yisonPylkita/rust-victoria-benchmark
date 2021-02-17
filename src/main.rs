use core::time;
use rand::{thread_rng, Rng};
use std::{fmt::format, thread};

fn generate_rows(start: i32, end: i32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    (start..end).map(|x| format!("schemaId=00000000-0000-0000-0000-000000000000,objectId={} value={:.8},quality_value=0u,quality_reason=0u,limit=0,flags=0u,time_valid=False,day_no=0i,time=0i", uuid::Uuid::from_u128(x as u128), rng.gen_range(0.0f32..300.0f32))).collect()
}

fn update_row(mut row: String) -> String {
    let mut rng = rand::thread_rng();
    let value_f32 = rng.gen_range(0.0f32..300.0f32);
    let value_str = format!("{:.8}", value_f32);
    let value_index = 98;
    for i in 0..8 {
        unsafe {
            row.as_bytes_mut()[value_index + i] = value_str.as_bytes()[i];
        }
    }

    return row;
}

fn main() -> Result<(), reqwest::Error> {
    let handler1 = thread::spawn(|| {
        let rows = generate_rows(0, 100);
        let client = reqwest::blocking::Client::new();
        let mut index = 0;
        let mut checked_times = 0;
        loop {
            if checked_times > 10_000 {
                let before_update = std::time::Instant::now();
                let row_to_send = update_row(rows[index].clone());
                println!("Update time: {}ns", before_update.elapsed().as_nanos());

                let before_send = std::time::Instant::now();
                client
                    .post("http://localhost:8428/write")
                    .body(row_to_send)
                    .send()
                    .unwrap();
                println!("Send time: {}ns\n", before_send.elapsed().as_nanos());
                checked_times = 0;
            } else {
                checked_times += 1;
                let row_to_send = update_row(rows[index].clone());
                client
                    .post("http://localhost:8428/write")
                    .body(row_to_send)
                    .send()
                    .unwrap();
            };

            index += 1;
            if index > 99 {
                index = 0;
            }
        }
    });

    let handler2 = thread::spawn(|| {
        let rows = generate_rows(100, 200);
        let client = reqwest::blocking::Client::new();
        let mut index = 0;
        loop {
            let row_to_send = update_row(rows[index].clone());
            client
                .post("http://localhost:8428/write")
                .body(row_to_send)
                .send()
                .unwrap();
            index += 1;
            if index > 99 {
                index = 0;
            }
        }
    });

    let handler3 = thread::spawn(|| {
        let rows = generate_rows(200, 300);
        let client = reqwest::blocking::Client::new();
        let mut index = 0;
        loop {
            let row_to_send = update_row(rows[index].clone());
            client
                .post("http://localhost:8428/write")
                .body(row_to_send)
                .send()
                .unwrap();
            index += 1;
            if index > 99 {
                index = 0;
            }
        }
    });

    let handler4 = thread::spawn(|| {
        let rows = generate_rows(300, 400);
        let client = reqwest::blocking::Client::new();
        let mut index = 0;
        loop {
            let row_to_send = update_row(rows[index].clone());
            client
                .post("http://localhost:8428/write")
                .body(row_to_send)
                .send()
                .unwrap();
            index += 1;
            if index > 99 {
                index = 0;
            }
        }
    });

    let handler5 = thread::spawn(|| {
        let rows = generate_rows(400, 500);
        let client = reqwest::blocking::Client::new();
        let mut index = 0;
        loop {
            let row_to_send = update_row(rows[index].clone());
            client
                .post("http://localhost:8428/write")
                .body(row_to_send)
                .send()
                .unwrap();
            index += 1;
            if index > 99 {
                index = 0;
            }
        }
    });

    let handler6 = thread::spawn(|| {
        let rows = generate_rows(500, 600);
        let client = reqwest::blocking::Client::new();
        let mut index = 0;
        loop {
            let row_to_send = update_row(rows[index].clone());
            client
                .post("http://localhost:8428/write")
                .body(row_to_send)
                .send()
                .unwrap();
            index += 1;
            if index > 99 {
                index = 0;
            }
        }
    });

    let handler7 = thread::spawn(|| {
        let rows = generate_rows(600, 700);
        let client = reqwest::blocking::Client::new();
        let mut index = 0;
        loop {
            let row_to_send = update_row(rows[index].clone());
            client
                .post("http://localhost:8428/write")
                .body(row_to_send)
                .send()
                .unwrap();
            index += 1;
            if index > 99 {
                index = 0;
            }
        }
    });

    let handler8 = thread::spawn(|| {
        let rows = generate_rows(700, 800);
        let client = reqwest::blocking::Client::new();
        let mut index = 0;
        loop {
            let row_to_send = update_row(rows[index].clone());
            client
                .post("http://localhost:8428/write")
                .body(row_to_send)
                .send()
                .unwrap();
            index += 1;
            if index > 99 {
                index = 0;
            }
        }
    });

    let handler9 = thread::spawn(|| {
        let rows = generate_rows(800, 900);
        let client = reqwest::blocking::Client::new();
        let mut index = 0;
        loop {
            let row_to_send = update_row(rows[index].clone());
            client
                .post("http://localhost:8428/write")
                .body(row_to_send)
                .send()
                .unwrap();
            index += 1;
            if index > 99 {
                index = 0;
            }
        }
    });

    let handler10 = thread::spawn(|| {
        let rows = generate_rows(900, 1000);
        let client = reqwest::blocking::Client::new();
        let mut index = 0;
        loop {
            let row_to_send = update_row(rows[index].clone());
            client
                .post("http://localhost:8428/write")
                .body(row_to_send)
                .send()
                .unwrap();
            index += 1;
            if index > 99 {
                index = 0;
            }
        }
    });

    handler1.join().unwrap();
    handler2.join().unwrap();
    handler3.join().unwrap();
    handler4.join().unwrap();
    handler5.join().unwrap();
    handler6.join().unwrap();
    handler7.join().unwrap();
    handler8.join().unwrap();
    handler9.join().unwrap();
    handler10.join().unwrap();
    Ok(())
}
