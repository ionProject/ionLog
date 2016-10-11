/*===============================================================================================*/
// Copyright 2016 Kyle Finlay
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
/*===============================================================================================*/

/*===============================================================================================*/
//! ionLog is a simple to use logging library.
//!
//! # Example
//! ```
//! #[macro_use]
//! extern crate log;
//! extern crate ion_log;
//! use ion_log::LogConfig;
//!
//! fn main () {
//!
//!     let mut config = LogConfig::new ();
//!     config.log_to_file = true;
//!     config.log_output_path = "Log.txt".to_string ();
//!
//!     ion_log::init (&config).unwrap ();
//!
//!     trace! ("This is a trace log");
//!     debug! ("This is a debug log");
//!     info!  ("This is an info log");
//!     warn!  ("This is a warning log");
//!     error! ("This is an error log");
//!
//!     ion_log::release ();
//! }
//! ```
/*===============================================================================================*/

// Crate attributes
#![deny (missing_debug_implementations)]
#![deny (missing_docs)]

// Create imports
extern crate ansi_term;
extern crate log;

// Module imports
use ansi_term::Colour::{Green, Blue, Purple, Yellow, Red};
pub use log::LogLevelFilter as LogLevel;

use std::boxed::Box;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::Write;

/*===============================================================================================*/
/*------LOG CONFIG STRUCT------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores the logger configuration.
#[derive (Clone, Debug)]
pub struct LogConfig {

    // Public
    /// Whether to log to the terminal.
    pub log_to_io:       bool,
    /// Whether to log to a file.
    pub log_to_file:     bool,
    /// The log output file path.
    pub log_output_path: String,
    /// Whether to use colour coded output.
    pub coloured_output: bool,
    /// The maximum log level.
    pub max_log_level:   LogLevel,
}

/*===============================================================================================*/
/*------LOG CONFIG TRAIT IMPLEMENTATIONS---------------------------------------------------------*/
/*===============================================================================================*/

impl Default for LogConfig {

    fn default () -> Self {

        LogConfig::new ()
    }
}

/*===============================================================================================*/
/*------LOG CONFIG PUBLIC METHODS----------------------------------------------------------------*/
/*===============================================================================================*/

impl LogConfig {

    /// Returns a new instance of `LogConfig`.
    ///
    /// # Examples
    /// ```
    /// # use ion_log::LogConfig;
    /// let config = LogConfig::new ();
    /// ```
    pub fn new () -> Self {

        LogConfig {

            log_to_io:       true,
            log_to_file:     false,
            log_output_path: String::new (),
            coloured_output: true,
            max_log_level:   LogLevel::Trace,
        }
    }
}

/*===============================================================================================*/
/*------LOGGER STRUCT----------------------------------------------------------------------------*/
/*===============================================================================================*/

struct Logger {

    // Private
    config: LogConfig,
    log_output_buffer: BufWriter<File>,
}

/*===============================================================================================*/
/*------LOGGER TRAIT IMPLEMENTATIONS-------------------------------------------------------------*/
/*===============================================================================================*/

impl log::Log for Logger {

    fn enabled (&self, metadata: &log::LogMetadata) -> bool {
        metadata.level () <= self.config.max_log_level
    }

/*-----------------------------------------------------------------------------------------------*/

    fn log (&self, record: &log::LogRecord) {

        let log_string = self.format_log_string (record);

        if self.config.log_to_io {

            if self.config.coloured_output {
                println! ("{}", self.format_log_colour (record, &log_string));
            }

            else {
                println! ("{}", log_string);
            }
        }

        if self.config.log_to_file {
            self.log_output_buffer.get_ref ().write (log_string.as_bytes ()).unwrap ();
        }
    }
}

/*===============================================================================================*/
/*------LOGGER PRIVATE METHODS-------------------------------------------------------------------*/
/*===============================================================================================*/

impl Logger {

    fn format_log_string (&self, record: &log::LogRecord) -> String {

        let log_string = format! ("[{} - {}] {}: {}\n",
                                  record.location ().module_path (),
                                  record.location ().line (),
                                  record.level (),
                                  record.args ());
        log_string
    }

/*-----------------------------------------------------------------------------------------------*/

    fn format_log_colour (&self, record: &log::LogRecord, log_string: &str) -> String {

        match record.level () {

            log::LogLevel::Trace => Green.paint  (log_string),
            log::LogLevel::Debug => Blue.paint   (log_string),
            log::LogLevel::Info  => Purple.paint (log_string),
            log::LogLevel::Warn  => Yellow.paint (log_string),
            log::LogLevel::Error => Red.paint    (log_string)

        }.to_string ()
    }
}

/*===============================================================================================*/
/*------PUBLIC FUNCTIONS-------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Initializes the logger.
pub fn init (config: &LogConfig) -> Result<(), log::SetLoggerError> {

    log::set_logger (|max_log_level| {

        max_log_level.set (config.max_log_level);

        Box::new (Logger {

            config: config.clone (),
            log_output_buffer: BufWriter::new (File::create (&config.log_output_path).unwrap ()),
        })
    })
}

/*-----------------------------------------------------------------------------------------------*/

/// Releases the logger.
pub fn release () {
    drop (log::shutdown_logger ().unwrap ());
}
