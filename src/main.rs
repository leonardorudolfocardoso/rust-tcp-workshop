use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn handle_client(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("handling stream {:?}", stream);
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);
    let mut message = String::new();

    loop {
        message.clear();
        if reader.read_line(&mut message)? == 0 {
            break;
        }
        writer.write_all(format!("I read and processed the message {}", message).as_bytes())?;
    }

    Ok(())
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        for _ in 0..size {
            let receiver = Arc::clone(&receiver);

            thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                job();
            });
        }

        ThreadPool { sender }
    }

    fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.sender.send(Box::new(f)).unwrap();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(2);

    for stream in listener.incoming() {
        let stream = stream?;

        pool.execute(|| {
            if let Err(e) = handle_client(stream) {
                eprintln!("{}", e);
            }
        });
    }

    Ok(())
}
