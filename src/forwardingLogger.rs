// This file is part of forwarding-logger. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/forwarding-logger/master/COPYRIGHT. No part of forwarding-logger, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of forwarding-logger. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/forwarding-logger/master/COPYRIGHT.


extern crate threading_support;
use self::threading_support::spawn_thread;
use self::threading_support::UsefulStackSize;
extern crate try_finally;
use self::try_finally::try_finally;
use std::cell::RefCell;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use std::thread::Result;
use std::raw::TraitObject;
use std::mem::transmute;
extern crate log;
use self::log::LogMetadata;
use self::log::LogRecord;
use self::log::LogLevelFilter;
use self::log::set_logger;
use self::log::shutdown_logger;
use self::log::Log;
use ForwardedLogMessage;


thread_local!(static ForwardingSender: RefCell<Option<Sender<ForwardedLogMessage>>> = RefCell::new(None));









macro_rules! downcast
{
	(x => $e:expr) =>
	(
		{
			println!("mode Y: {}", $e);
		}
	);
}

fn downcast_trait_object2(trait_object: Box<Log>) -> &ForwardingLogger
{
	use std::mem::transmute;
	use std::raw::TraitObject;

    let x: TraitObject = unsafe { transmute(trait_object.as_ref()) };
    let z: &ForwardingLogger = unsafe { transmute(x.data) };
    z
}









pub struct ForwardingLogger
{
	logging_thread_join_handle: JoinHandle<()>
}

pub fn with_forwarding_logger<F, R>(log_level_filter: LogLevelFilter, action: F) -> R
where F: Fn() -> R
{
	ForwardingLogger::init(log_level_filter);
	
	try_finally(action, |outcome|
	{
		ForwardingLogger::shutdown();
	})
}

impl ForwardingLogger
{
	pub fn init(log_level_filter: LogLevelFilter) -> Box<ForwardingLogger>
	{
		let (sender, receiver): (Sender<ForwardedLogMessage>, Receiver<ForwardedLogMessage>) = channel();
		
		let joinHandle = spawn_thread(String::from("Logger"), UsefulStackSize, move |relative_thread_id|
		{
			loop
			{
				match receiver.recv()
				{
					Err(_) => break,
					Ok(forwarded_log_message) =>
					{
						println!("{:?}", forwarded_log_message);
					},
				}
			}
		});

		let result = Box::new(ForwardingLogger
		{
			logging_thread_join_handle: joinHandle,
		});
		
		ForwardingSender.with(|value|
		{
			*value.borrow_mut() = Some(sender);
		});
		
		set_logger(|max_log_level|
		{
			max_log_level.set(log_level_filter);
			result
		});

		result
	}
	
	pub fn spawn_with_logging<F, T>(thread_name: String, thread_stack_size: usize, f: F) -> JoinHandle<T>
	where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
	{
		ForwardingSender.with(|value|
		{
			let borrowed = value.borrow();
			let unwrapped = borrowed.as_ref().unwrap();
			let cloned = unwrapped.clone();
		
			spawn_thread(thread_name, thread_stack_size, move ||
			{
				ForwardingSender.with(|value|
				{
					*value.borrow_mut() = Some(cloned);
				});
			
				f()
			})
		})
	}
	
	pub fn shutdown()
	{
		if let Ok(logger) = shutdown_logger()
		{
			// Should force a drop of the sender; until all senders are dropped, the logging thread will not exit
			ForwardingSender.with(|value|
			{
				*value.borrow_mut() = None;
			});
			
			let forwarding_logger: &ForwardingLogger = downcast_trait_object2(logger);
			
			forwarding_logger.logging_thread_join_handle.join().expect("Logging thread unexpectedly terminated in a panic");
		}
	}
}

impl log::Log for ForwardingLogger
{
	#[inline(always)]
	fn enabled(&self, metadata: &LogMetadata) -> bool
	{
		true
	}

	#[inline(always)]
	fn log<'a>(&self, record: &LogRecord<'a>)
	{
		// order as per __log();
		//let level: LogLevel = record.level();
		//let target: &str = record.target();
		//let loc: &LogLocation = record.location(); // consists of module_path(), file() and line()
		//let args: &Arguments = record.args();

		ForwardingSender.with(|value|
		{
			let message = ForwardedLogMessage::from_log_record(record);
			let borrowed = value.borrow();
			let unwrapped = borrowed.as_ref().unwrap();
			unwrapped.send(message);
		})
	}
}
