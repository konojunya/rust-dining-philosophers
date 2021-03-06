use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Philosopher {
  name: String,
  left: usize,
  right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

impl Philosopher {
  fn new(name: &str, left: usize, right: usize) -> Philosopher {
    Philosopher {
      name: name.to_string(),
      left: left,
      right: right,
    }
  }

  fn eat(&self, table: &Table) {
    let _left = table.forks[self.left].lock().unwrap();
    thread::sleep(Duration::from_millis(150));
    let _right = table.forks[self.right].lock().unwrap();

    println!("{}がいまご飯中！", self.name);
    thread::sleep(Duration::from_millis(1000));
    println!("{}はお腹いっぱいになった！", self.name);
  }
}

fn main() {
  let table = Arc::new(Table { forks: vec![
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
  ]});

  let philosophers = vec![
    Philosopher::new("スティーブ・ジョブズ", 0, 1),
    Philosopher::new("ジェフ・ベゾス", 1, 2),
    Philosopher::new("マーク・ザッカーバーグ", 2, 3),
    Philosopher::new("ラリー・ペイジ", 3, 4),
    Philosopher::new("コーノ・ジュンヤ", 0, 4),
  ];

  let handles: Vec<_> = philosophers.into_iter().map(|p| {
    let table = table.clone();
    thread::spawn(move || {
      p.eat(&table);
    })
  }).collect();

  for h in handles {
    h.join().unwrap();
  }
}