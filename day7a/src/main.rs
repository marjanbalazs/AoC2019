// Another Intcode Machine problem

// So this problem screams for a threadpool and a central thread that will string together the inputs and outputs between the stages.
// The threadpool will run intmachines and the central thread will submit jobs with a "phase" and "input" and expect an output.

use intcode::Machine;
use std::env;
use std::fs;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // This is hand tailored, but I want to the function to take three arguments... The input, the phase setting and the memory it will operate on
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<std::sync::Mutex<std::sync::mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move|| {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);
    
                job();
            }
        });
        Worker { id, handle }
    }
}

fn parse_file_to_vec(file_path: String) -> Vec<i32> {
    let content = fs::read_to_string(file_path).unwrap();
    let tokens: Vec<&str> = content.split(',').collect();

    let v: Vec<i32> = tokens
        .into_iter()
        .map(|e| e.trim().parse::<i32>().unwrap())
        .collect();

    v
}

fn main() -> Result<(), ()> {
    let vec_str: Vec<String> = env::args().collect();

    let (file_path, input_val) = match vec_str.len() {
        0..=2 => {
            panic!("Not enough args");
        }
        3 => (
            vec_str.get(1).unwrap(),
            (*vec_str.get(2).unwrap()).trim().parse::<i32>().unwrap(),
        ),
        _ => {
            panic!("Too many args");
        }
    };

    let mem_original = parse_file_to_vec(file_path.to_string());

    /*
    machine.run();
    println!("{}", machine.get_output());
    */

    let threadpool = ThreadPool::new(4);



    threadpool.execute(move|| {
        let mut machine = intcode::Machine {
            memory: &mut mem_original.clone(),
            ip: 0,
            input: input_val,
            output: 0,
        };
        machine.run();
        println!("Result: {}", machine.get_output());
    });

    Ok(())
}
