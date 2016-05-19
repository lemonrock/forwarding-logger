// This file is part of forwarding-logger. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/forwarding-logger/master/COPYRIGHT. No part of forwarding-logger, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of forwarding-logger. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/forwarding-logger/master/COPYRIGHT.

#![feature(raw)]

pub use forwardedLogMessage::ForwardedLogMessage;
mod forwardedLogMessage;

pub use forwardingLogger::ForwardingLogger;
mod forwardingLogger;

// extern crate log;
// use log::{LogLevelFilter, SetLoggerError};
//
// pub fn withALoggingThread<F>(closureThatTakesASender: F) where F: Fn(&Sender<ForwardedLogMessage>) -> (),
// {
// 	let (loggerJoinHandle, sender) = createLogger(Facility::LOG_LOCAL0);
//
// 	closureThatTakesASender(&sender);
//
// 	// We have to force our sender out-of-scope to cause the sender to disconnect
// 	drop(sender);
// 	loggerJoinHandle.join().unwrap();
// }
//
// fn createLogger(facility: Facility) -> (JoinHandle<()>, Sender<ForwardedLogMessage>)
// {
// 	let writer = syslog::unix(facility).unwrap();
// 	let (sender, receiver): (Sender<ForwardedLogMessage>, Receiver<ForwardedLogMessage>) = mpsc::channel();
//
// 	println!("Change to builder");
// 	let joinHandle = thread::spawn(move || {
// 		// The thread borrows (takes temporary ownership of ourSender) of anything in the environment
//
// 		loop
// 		{
// 			match receiver.recv()
// 			{
// 				Err(_) => break,
// 				Ok(logRecord) =>
// 				{
// 					let message = logRecord.target();
//
// 					println!("{}", message);
//
// 					// if we change this to format!(), we're allocated a string on the heap
// 					let result = writer.send_3164(Severity::LOG_ALERT, &message);
// 					if result.is_err()
// 					{
// 						println!("error sending to syslog {}", result.err().expect("got error"));
// 					}
// 				},
// 			}
// 		}
// 	});
//
// 	return (joinHandle, sender);
// }
//
// // Problem: each thread has a sender...
// pub fn init(log: LogLevelFilter, sender: Sender<String>) -> Result<(), SetLoggerError>
// {
// 	log::set_logger(|max_log_level|
// 	{
// 	    max_log_level.set(LogLevelFilter::Info);
// 	    Box::new(ForwardingLogger::new())
// 	})
// }
