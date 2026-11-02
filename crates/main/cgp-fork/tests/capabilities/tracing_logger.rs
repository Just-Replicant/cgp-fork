//! `TracingLogger` and `TracingDebugLogger` implement `CanLog`.

use std::fmt::{self, Display};
use std::sync::{Arc, Mutex};

use cgp_fork::extra::log::{TracingDebugLogger, TracingLogger};
use cgp_fork::prelude::*;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

pub struct Started {
    pub name: &'static str,
}

impl Display for Started {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "started {}", self.name)
    }
}

pub struct App;

delegate_components! {
    #[unchecked]
    App {
        LoggerComponent: TracingLogger,
    }
}

check_components! {
    App {
        LoggerComponent: Started,
    }
}

#[derive(Debug)]
pub struct Tick {
    pub n: u32,
}

pub struct DebugApp;

delegate_components! {
    #[unchecked]
    DebugApp {
        LoggerComponent: TracingDebugLogger,
    }
}

check_components! {
    DebugApp {
        LoggerComponent: Tick,
    }
}

struct Capture(Arc<Mutex<String>>);

struct WriteMessage<'a>(&'a mut String);

impl Visit for WriteMessage<'_> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.0.push_str(&format!("{value:?}"));
        }
    }
}

impl Subscriber for Capture {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _span: &Attributes<'_>) -> Id {
        Id::from_u64(1)
    }

    fn record(&self, _span: &Id, _values: &Record<'_>) {}

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, event: &Event<'_>) {
        let mut message = self.0.lock().expect("message lock");
        event.record(&mut WriteMessage(&mut message));
    }

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}
}

fn captured(log: impl FnOnce()) -> String {
    let message = Arc::new(Mutex::new(String::new()));
    tracing::subscriber::with_default(Capture(Arc::clone(&message)), log);
    message.lock().expect("message lock").clone()
}

#[test]
fn tracing_logger_records_the_display_detail() {
    let message = captured(|| App.log(Started { name: "Ferris" }));
    assert!(message.contains("started Ferris"), "{message}");
}

#[test]
fn tracing_debug_logger_records_the_debug_detail() {
    let message = captured(|| DebugApp.log(Tick { n: 7 }));
    assert!(message.contains("Tick"), "{message}");
    assert!(message.contains('7'), "{message}");
}
