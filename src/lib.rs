#![deny(
    missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
    unused_qualifications
)]

extern crate chrono;
extern crate config;
extern crate ctrlc;
extern crate dbus;
extern crate inotify;
extern crate libnotify;
extern crate uuid;
extern crate x11;

mod async;
mod conf;
mod error;
#[macro_use]
mod feature;
mod features;
mod io;
mod status_bar;

use error::*;
use status_bar::StatusBar;
use std::collections::HashMap;
use std::env;
use std::sync::mpsc;

fn get_config() -> Result<String> {
    let mut args = env::args();

    let path = args
        .nth(1)
        .wrap_error("usage", "first parameter config file")?;

    io::read_file(&path).wrap_error("config file", &format!("{} not readable", path))
}

fn render(
    tx: &mpsc::Sender<async::Message>,
    rx: &mpsc::Receiver<async::Message>,
    order: &[String],
    feature_map: &mut HashMap<String, Box<feature::Feature>>,
) -> Result<()> {
    let tx = tx.clone();
    ctrlc::set_handler(move || {
        tx.send(async::Message::Kill)
            .wrap_error_kill("termination", "notify thread killed");
    }).wrap_error("termination", "failed to set termination handler")?;

    let status_bar = StatusBar::new()?;
    status_bar.render(order, feature_map)?;

    for message in rx {
        match message {
            async::Message::FeatureUpdate(ref id) => {
                match feature_map.get_mut(id) {
                    Some(ref mut feature) => {
                        feature.update()?;
                        println!("update {}: {}", feature.name(), feature.render());
                    },
                    None => {
                        return Err(Error::new_custom(
                            "invalid message",
                            &format!("message id {} does not exist", id),
                        ))
                    },
                };

                status_bar.render(order, feature_map)?;
            },
            async::Message::Kill => break,
        }
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let config = conf::Conf::new()?;

    let mut features = Vec::new();
    for line in get_config()?.lines() {
        let mut feature = features::create_feature(line, &tx)?;
        feature.init_notifier()?;
        feature.update()?;
        features.push(feature);
    }

    if features.is_empty() {
        return Err(Error::new_custom("empty config", "no features enabled"));
    }

    let order: Vec<_> = features.iter().map(|x| String::from(x.id())).collect();

    let mut feature_map: HashMap<_, _> = features
        .into_iter()
        .map(|feature| (String::from(feature.id()), feature))
        .collect();

    render(&tx, &rx, &order, &mut feature_map)
}
