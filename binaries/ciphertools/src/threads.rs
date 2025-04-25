use crate::CiphertoolsContext;
use crate::scoreboard::Scoreboard;
use crossbeam::channel::{self, Receiver, Sender};
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

pub enum CandidateCollectorMsg {
  CandidatePlaintext { text: String, key: String },
}

pub fn spawn_candidate_collector(
  ciphertools_context: CiphertoolsContext,
) -> (Sender<CandidateCollectorMsg>, JoinHandle<()>) {
  let (tx, rx) = channel::bounded(100);
  let handle = thread::spawn(|| {
    candidate_collector(ciphertools_context, rx);
  });
  (tx, handle)
}

fn candidate_collector(
  ciphertools_context: CiphertoolsContext,
  rx: Receiver<CandidateCollectorMsg>,
) {
  let CiphertoolsContext {
    get_confidence,
    pool,
  } = ciphertools_context;

  let scoreboard = Arc::new(Scoreboard::new(
    NonZeroUsize::new(10).unwrap(),
    get_confidence.clone(),
  ));

  pool.scope(|s| {
    rx.iter().for_each(|msg| {
      let scoreboard = Arc::clone(&scoreboard);
      let get_confidence = get_confidence.clone();

      s.spawn(move |_| match msg {
        CandidateCollectorMsg::CandidatePlaintext { text, key } => {
          let confidence = get_confidence.run(&text);
          scoreboard.insert_with_confidence(text, key, confidence);
        }
      });
    });
  });

  scoreboard.display_scoreboard();
}
