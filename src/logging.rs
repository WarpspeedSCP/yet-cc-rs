use log::{Level, Log, Metadata, Record};
use std::env;
use std::io::Write;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

enum LogOutput {
	None,
	Stdout,
	Stderr,
	File(Arc<Mutex<std::io::BufWriter<std::fs::File>>>),
}

struct SimpleLogger {
	level: Level,
	output_buffers: [LogOutput; 6],
}

impl log::Log for SimpleLogger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= self.level
	}

	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			let output = &self.output_buffers[record.level() as usize];
			let time = chrono::Local::now();
			match output {
				LogOutput::None => {}
				LogOutput::Stderr => eprintln!(
					"[{} {}] ({}:{}) {}",
					record.level(),
					time.format("%H:%M:%S.%f"),
					record.file().unwrap_or("???"),
					record
						.line()
						.map(|it| it.to_string())
						.unwrap_or("??".to_owned()),
					record.args()
				),
				LogOutput::Stdout => println!(
					"[{} {}] ({}:{}) {}",
					record.level(),
					time.format("%H:%M:%S.%f"),
					record.file().unwrap_or("???"),
					record
						.line()
						.map(|it| it.to_string())
						.unwrap_or("??".to_owned()),
					record.args()
				),
				LogOutput::File(mutex) => {
					let res = mutex
						.lock()
						.map_err(|_err| "Could not lock log file!")
						.and_then(|ref mut w| {
							writeln!(
								w,
								"[{} {}] ({}:{}) {}",
								record.level(),
								time.format("%H:%M:%S.%f"),
								record.file().unwrap_or("???"),
								record
									.line()
									.map(|it| it.to_string())
									.unwrap_or("??".to_owned()),
								record.args()
							)
							.map_err(|_err| "Could not write to log file!")
						});

					if let Err(e) = res {
						eprintln!(
							"[{} {}] ({}:{}) {e}",
							record.level(),
							time.format("%H:%M:%S.%f"),
							record.file().unwrap_or("???"),
							record
								.line()
								.map(|it| it.to_string())
								.unwrap_or("??".to_owned()),
						);
						eprintln!("Original log message: {}", record.args());
					}
				}
			}
		}
	}

	fn flush(&self) {
		for output in self.output_buffers.iter() {
			match output {
				LogOutput::None => {}
				LogOutput::Stdout => {
					std::io::stdout().flush().expect("Could not flush stdout");
				}
				LogOutput::Stderr => {
					std::io::stderr().flush().expect("Could not flush stderr");
				}
				LogOutput::File(output) => {
					output
						.lock()
						.expect("Could not lock log file!")
						.flush()
						.expect("Could not flush output file!");
				}
			}
		}
	}
}

impl SimpleLogger {
	pub fn from_env() -> Box<Self> {
		let matching_level =
			log::Level::from_str(&env::var("RUST_LOG").unwrap_or("info".to_owned()))
				.unwrap_or(Level::Info);

		let log_output_str = env::var("LOG_OUTPUT")
			.map(|it| it.to_lowercase())
			.unwrap_or_default();

		let mut output_buffers = [
			LogOutput::None, // Log levels are one indexed.
			LogOutput::Stderr,
			LogOutput::Stderr,
			LogOutput::Stdout,
			LogOutput::None,
			LogOutput::None,
		];

		if log_output_str.is_empty() {
			for (idx, output) in output_buffers.iter_mut().enumerate() {
				if idx <= matching_level as usize {
					let _ = std::mem::replace(output, LogOutput::Stderr);
				}
			}
		} else {
			for (k, v) in log_output_str.split(';').map(|it| {
				let mut data = it.split('=');
				let key = data.next().unwrap_or_default();
				let value = data.next().unwrap_or_default();
				(key, value)
			}) {
				if k.is_empty() || v.is_empty() {
					// TODO: Log an error here.
					continue;
				}
				let log_level = log::Level::from_str(k).unwrap() as usize;

				let configured_log_level = match v {
					"off" => LogOutput::None,
					"stderr" => LogOutput::Stderr,
					"stdout" => LogOutput::Stdout,
					_ => {
						let path = std::path::PathBuf::from(v);
						let file = std::fs::File::create(path).expect("Failed to open log file!");
						LogOutput::File(Arc::new(Mutex::new(std::io::BufWriter::new(file))))
					}
				};

				let _ = std::mem::replace(
					output_buffers.get_mut(log_level).unwrap(),
					configured_log_level,
				);
			}
		}

		Box::new(SimpleLogger {
			level: matching_level,
			output_buffers,
		})
	}
}

pub fn init() -> Result<(), &'static str> {
	let logger = SimpleLogger::from_env();
	let level_filter = logger.level.to_level_filter();
	log::set_boxed_logger(logger)
		.map(|()| log::set_max_level(level_filter))
		.map_err(|_| "Logger already initialised!")
}
