use std::thread::yield_now; // как вызвать deadlocks в многопоточности искусственно, разбор решения
use std::thread::JoinHandle; // надо ещё посмотреть здесь https://deadlockempire.github.io. Там примеры на C#, но некоторые паттерны можно и на Rust перенести
use std::sync::Barrier;
use std::sync::Arc;
use std::sync::Mutex;

fn thr(fst: Arc<Mutex<()>>, snd: Arc<Mutex<()>>, b: Arc<Barrier>) {
    b.wait();
    //println!("lock 1");
    let _g1 = fst.lock();
    // yield_now();  
    //println!("lock 2");
    let _g2 = snd.lock();
    //println!("done");
}

fn start_thread(fst: &Arc<Mutex<()>>, snd: &Arc<Mutex<()>>, b: &Arc<Barrier>) -> JoinHandle<()> { // интересно, что функция исключительно для клонирования
    let fst = fst.clone();
    // yield_now();  // эта штука нужна, если потоки эмулируются - как-то так. 
    let snd = snd.clone();
    let b = b.clone();
    std::thread::spawn(move || thr(fst, snd, b))
}

fn main() {
    let fst = Arc::new(Mutex::new(()));
    let snd = Arc::new(Mutex::new(()));
    for i in 0..30 {
        println!("begin {i}");
        let b = Arc::new(Barrier::new(2));  // барьер нужно между взятиями локов юзать, чтобы наверняка (не так, как здесь)
        let t1 = start_thread(&fst, &snd, &b);
        let t2 = start_thread(&snd, &fst, &b);
        t1.join();
        t2.join();
        println!("end {i}");
    }
}
