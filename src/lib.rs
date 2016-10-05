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
/*===============================================================================================*/

// Crate attributes
#![deny (missing_debug_implementations)]
#![deny (missing_docs)]

// Create imports
extern crate ansi_term;
extern crate log;

// Module imports
use ansi_term::Colour::{Blue, Purple, Yellow, Red};
pub use log::{Log, LogRecord, LogMetadata, LogLevelFilter};

use std::boxed::Box;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::Write;

/*===============================================================================================*/
/*------LOG BUILDER STRUCT-----------------------------------------------------------------------*/
/*===============================================================================================*/

/// Allows for easy configuration of the logger.
#[derive (Debug)]
pub struct LogBuilder {

    // Private
    _coloured_output: bool,
    _log_file_path:   String,
    _log_to_file:     bool,
    _log_to_term:     bool,
    _max_log_level:   LogLevelFilter,
}

/*===============================================================================================*/
/*------LOG BUILDER TRAIT IMPLEMENTATIONS--------------------------------------------------------*/
/*===============================================================================================*/

impl Default for LogBuilder {

    fn default () -> Self {

        LogBuilder::new ()
    }
}

/*===============================================================================================*/
/*------LOG BUILDER PUBLIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

impl LogBuilder {

    /// Returns a new `LogBuilder` instance
    ///
    /// # Examples
    /// ```
    /// # use ion_log::LogBuilder;
    /// let builder = LogBuilder::new ();
    /// ```
    pub fn new () -> Self {

        LogBuilder {

            _coloured_output: true,
            _log_file_path:   "log.txt".to_string (),
            _log_to_file:     true,
            _log_to_term:     true,
            _max_log_level:   LogLevelFilter::Debug,
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets whether the terminal output should be colour coded.
    ///
    /// # Examples
    /// ```
    /// # use ion_log::LogBuilder;
    /// let mut builder = LogBuilder::new ();
    /// builder.coloured_output (false);
    /// ```
    pub fn coloured_output (&mut self, coloured_output: bool) -> &mut Self {

        self._coloured_output = coloured_output;
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the log file path.
    ///
    /// # Examples
    /// ```
    /// # use ion_log::LogBuilder;
    /// let mut builder = LogBuilder::new ();
    /// builder.log_file_path ("./Logs/log.txt");
    /// ```
    pub fn log_file_path (&mut self, log_file_path: &str) -> &mut Self {

        self._log_file_path = log_file_path.to_string ();
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets whether to write output to a file.
    ///
    /// # Examples
    /// ```
    /// # use ion_log::LogBuilder;
    /// let mut builder = LogBuilder::new ();
    /// builder.log_to_file (true);
    /// ```
    pub fn log_to_file (&mut self, log_to_file: bool) -> &mut Self {

        self._log_to_file = log_to_file;
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets whether to write output to the terminal.
    ///
    /// # Examples
    /// ```
    /// # use ion_log::LogBuilder;
    /// let mut builder = LogBuilder::new ();
    /// builder.log_to_term (true);
    /// ```
    pub fn log_to_term (&mut self, log_to_term: bool) -> &mut Self {

        self._log_to_term = log_to_term;
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the max log level.
    ///
    /// # Examples
    /// ```
    /// # use ion_log::{LogBuilder, LogLevelFilter};
    /// let mut builder = LogBuilder::new ();
    /// builder.max_log_level (LogLevelFilter::Debug);
    /// ```
    pub fn max_log_level (&mut self, log_level: LogLevelFilter) -> &mut Self {

        self._max_log_level = log_level;
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Finalizes the builder and initializes the logger.
    ///
    /// # Examples
    /// ```
    /// # use ion_log::LogBuilder;
    /// let mut builder = LogBuilder::new ();
    /// builder.log_file_path ("log.txt")
    ///        .coloured_output (false)
    ///        .finalize ()
    ///        .unwrap ();
    /// ```
    pub fn finalize (&self) -> Result<(), log::SetLoggerError> {

        log::set_logger (|max_log_level| {

            max_log_level.set (self._max_log_level);

            Box::new (Logger {

                _coloured_output: self._coloured_output,
                _log_file:        BufWriter::new (File::create (&self._log_file_path).unwrap ()),
                _log_to_file:     self._log_to_file,
                _log_to_term:     self._log_to_term,
            })
        })
    }
}

/*===============================================================================================*/
/*------LOG STRUCT-------------------------------------------------------------------------------*/
/*===============================================================================================*/

struct Logger {

    _coloured_output: bool,
    _log_file:        BufWriter<File>,
    _log_to_file:     bool,
    _log_to_term:     bool,
}

/*===============================================================================================*/
/*------LOG TRAIT IMPLEMENTATIONS----------------------------------------------------------------*/
/*===============================================================================================*/

impl log::Log for Logger {

    fn enabled (&self, metadata: &LogMetadata) -> bool {
        unimplemented! ()
    }

/*-----------------------------------------------------------------------------------------------*/

    fn log (&self, record: &LogRecord) {
        unimplemented! ()
    }
}
