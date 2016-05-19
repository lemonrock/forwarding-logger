// This file is part of forwarding-logger. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/forwarding-logger/master/COPYRIGHT. No part of forwarding-logger, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of forwarding-logger. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/forwarding-logger/master/COPYRIGHT.


extern crate log;
use self::log::LogLevel;
use self::log::LogRecord;
use std::thread::current;


#[derive(Debug, Clone)]
pub struct ForwardedLogMessage
{
	level: LogLevel,
	target: String,
	threadName: Option<String>,
}

impl ForwardedLogMessage
{
	pub fn from_log_record<'a>(log_record: &'a LogRecord) -> ForwardedLogMessage
	{
		let target = log_record.target().to_owned();
		// &str... look into a Cow?
		let threadName = current().name().map(|name| name.to_owned());
		
		ForwardedLogMessage
		{
			level: log_record.level(),
			target: target,
			threadName: threadName,
		}
	}
	
	// fn rfc31644_message(&self) -> String
	// {
	// 	format!("{}:{}:{}", self.level, self.target, self.threadName.unwrap_or_else(|| String::from("<unknown>")))
	// }
}
