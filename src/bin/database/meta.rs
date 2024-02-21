use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use term_rustdoc::util::XString;

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct DocMeta {
    /// the rustc/rustdoc/cargo version compiling the doc, gotten by `cargo +nightly -Vv`
    /// NOTE: only nightly toolchain is supported for now
    cargo_version: String,
    /// the host field from `rustc_version`
    host_triple: XString,
    /// TODO: the target platform. we haven't supported this other than host triple,
    /// so usually this equals to host_triple.
    target_triple: XString,
    // /// For now, each doc is generated on local machine.
    // /// TODO:
    // /// But for the future, we can support save and load docs non-locally generated.
    // /// For example, crates.io or docs.rs or somthing can provide compiled docs, so
    // /// we don't need to compile them locally. Or if you migrate/duplicate docs from
    // /// one machine to another machine.
    // is_local: bool,
    /// the time when the doc starts to compile
    started: SystemTime,
    /// the time when the doc takes to be compiled and generated
    duration: Duration,
}

impl Default for DocMeta {
    fn default() -> Self {
        let started = SystemTime::now();
        let (cargo_version, host_triple, target_triple, duration) = Default::default();
        DocMeta {
            cargo_version,
            host_triple,
            target_triple,
            started,
            duration,
        }
    }
}

impl DocMeta {
    pub fn new() -> Self {
        match std::process::Command::new("cargo")
            .args(["+nightly", "-Vv"])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    let started = SystemTime::now();
                    let cargo_version = String::from_utf8_lossy(&output.stdout).into_owned();
                    let host_triple = cargo_version
                        .lines()
                        .find_map(|line| {
                            if line.starts_with("host: ") {
                                line.get(6..).map(XString::from)
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default();
                    let target_triple = host_triple.clone();
                    return DocMeta {
                        cargo_version,
                        host_triple,
                        target_triple,
                        started,
                        duration: Duration::default(),
                    };
                }
                let err = String::from_utf8_lossy(&output.stderr);
                error!("Failed to run `cargo +nightly -Vv` to get version and host_triple:\n{err}");
            }
            Err(err) => {
                error!("Failed to run `cargo +nightly -Vv` to get version and host_triple:\n{err}")
            }
        }
        DocMeta::default()
    }

    pub fn set_finished_duration(&mut self) {
        self.duration = self.started.elapsed().unwrap_or_default();
    }

    pub fn duration_as_secs(&self) -> f32 {
        self.duration.as_secs_f32()
    }
}
