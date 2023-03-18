use relm4::Worker;
use std::{
  sync::atomic::{AtomicBool, Ordering},
  thread::{self, JoinHandle},
  time::{Duration, Instant},
};

static RUNNING: AtomicBool = AtomicBool::new(false);

/// A worker with a background timer thread that emits
/// time interval outputs.
pub struct GameTimer(Option<JoinHandle<()>>);

#[derive(Debug)]
pub enum GameTimerInput {
  Start,
  Stop,
}

#[derive(Debug)]
pub enum GameTimerOutput {
  Tick(u64),
}

impl Worker for GameTimer {
  type Init = ();
  type Input = GameTimerInput;
  type Output = GameTimerOutput;

  fn init(_init: Self::Init, _sender: relm4::ComponentSender<Self>) -> Self {
    Self(None)
  }

  fn update(
    &mut self,
    message: Self::Input,
    sender: relm4::ComponentSender<Self>,
  ) {
    match message {
      GameTimerInput::Start => {
        // This should not be running.
        self.stop_thread();
        RUNNING.store(true, Ordering::Release);
        self.0 = Some(start_timer(sender));
      }
      GameTimerInput::Stop => {
        self.stop_thread();
      }
    }
  }
}

impl GameTimer {
  fn stop_thread(&mut self) {
    println!("Stopping timer");
    if let Some(handle) = self.0.take() {
      RUNNING.store(false, Ordering::Release);
      handle.join().expect("Failed to wait on thread");
    }
  }
}

fn start_timer(sender: relm4::ComponentSender<GameTimer>) -> JoinHandle<()> {
  println!("Starting timer");
  thread::spawn(move || {
    let start_time = Instant::now();
    loop {
      if RUNNING.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_secs(1));
        sender
          .output(GameTimerOutput::Tick(start_time.elapsed().as_secs()))
          .unwrap();
      } else {
        break;
      }
    }
  })
}
