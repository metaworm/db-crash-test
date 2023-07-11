use std::time::Duration;

fn main() {
    use redb::{Database, ReadableTable, TableDefinition};

    const TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("table");

    let db = Database::builder()
        .set_cache_size(1024 * 1024 * 2)
        .create("redb-test")
        .unwrap();
    let db: &'static Database = Box::leak(db.into());

    for i in 0..100000 {
        let key = format!("key_{i}");
        let tr = db.begin_write().expect("write");
        {
            let mut t = tr.open_table(TABLE).unwrap();
            let mut value = vec![0; 4000 + rand::random::<usize>() % 1000];
            for v in value.iter_mut() {
                *v = b'A' + rand::random::<u8>() % 26;
            }
            t.insert(key.as_bytes(), value.as_slice()).expect("insert");
        }
        let k = key.clone();
        std::thread::spawn(move || {
            let tr = db.begin_read().expect("read");
            if let Ok(t) = tr.open_table(TABLE) {
                if let Some(value) = t.get(key.as_bytes()).expect("get") {
                    println!(
                        "last value[{}]: {}",
                        i,
                        String::from_utf8_lossy(&value.value()[..100])
                    );
                }
            }
            drop(tr);
        });
        tr.commit().expect("commit");
        let tr = db.begin_write().expect("write");
        {
            let mut t = tr.open_table(TABLE).unwrap();
            let mut value = vec![0; 4000 + rand::random::<usize>() % 1000];
            for v in value.iter_mut() {
                *v = b'A' + rand::random::<u8>() % 26;
            }
            t.insert(k.as_bytes(), value.as_slice()).expect("insert");
        }
        // tr.commit().expect("commit");
        std::thread::sleep(Duration::from_millis(1));
    }
}
