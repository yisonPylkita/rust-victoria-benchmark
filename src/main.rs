use rand::Rng;
use std::thread;

fn generate_rows(start: i32, end: i32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    (start..end).map(|x| format!("schemaId=00000000-0000-0000-0000-000000000000,objectId={} value={:.8},first_param=0u,second_param=0u,limit=0,flags=0u,should_i_stay=False,or_should_i_go=True,time=0i", uuid::Uuid::from_u128(x as u128), rng.gen_range(0.0f32..300.0f32))).collect()
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

fn worker_main(start: i32, end: i32) {
    let rows = generate_rows(start, end);
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
}

fn main() -> Result<(), reqwest::Error> {
    let handler1 = thread::spawn(|| worker_main(0, 100));
    let handler2 = thread::spawn(|| worker_main(100, 200));
    let handler3 = thread::spawn(|| worker_main(200, 300));
    let handler4 = thread::spawn(|| worker_main(300, 400));
    let handler5 = thread::spawn(|| worker_main(400, 500));
    let handler6 = thread::spawn(|| worker_main(500, 600));
    let handler7 = thread::spawn(|| worker_main(600, 700));
    let handler8 = thread::spawn(|| worker_main(700, 800));
    let handler9 = thread::spawn(|| worker_main(800, 900));
    let handler10 = thread::spawn(|| worker_main(900, 1000));
    let handler11 = thread::spawn(|| worker_main(0, 100));
    let handler12 = thread::spawn(|| worker_main(100, 200));
    let handler13 = thread::spawn(|| worker_main(200, 300));
    let handler14 = thread::spawn(|| worker_main(300, 400));
    let handler15 = thread::spawn(|| worker_main(400, 500));
    let handler16 = thread::spawn(|| worker_main(500, 600));
    let handler17 = thread::spawn(|| worker_main(600, 700));
    let handler18 = thread::spawn(|| worker_main(700, 800));
    let handler19 = thread::spawn(|| worker_main(800, 900));
    let handler20 = thread::spawn(|| worker_main(800, 900));

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
    handler11.join().unwrap();
    handler12.join().unwrap();
    handler13.join().unwrap();
    handler14.join().unwrap();
    handler15.join().unwrap();
    handler16.join().unwrap();
    handler17.join().unwrap();
    handler18.join().unwrap();
    handler19.join().unwrap();
    handler20.join().unwrap();

    Ok(())
}
