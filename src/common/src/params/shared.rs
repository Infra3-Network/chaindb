// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::path::PathBuf;

use clap::Args;

use crate::BasePath;

/// Shared parameters used by all `CoreParams`.
#[derive(Debug, Clone, PartialEq, Args)]
pub struct SharedParams {
	/// Specify the development chain.
	///
	/// This flag sets `--chain=dev`, `--force-authoring`, `--rpc-cors=all`,
	/// `--alice`, and `--tmp` flags, unless explicitly overridden.
	#[arg(long)]
	pub dev: bool,

	/// Specify custom base path.
	#[arg(long, short = 'd', value_name = "PATH")]
	pub base_path: Option<PathBuf>,

	/// Sets a custom logging filter. Syntax is `<target>=<level>`, e.g. -lsync=debug.
	///
	/// Log levels (least to most verbose) are error, warn, info, debug, and trace.
	/// By default, all targets log `info`. The global log level can be set with `-l<level>`.
	#[arg(short = 'l', long, value_name = "LOG_PATTERN", num_args = 1..)]
	pub log: Vec<String>,

	/// Enable detailed log output.
	///
	/// This includes displaying the log target, log level and thread name.
	///
	/// This is automatically enabled when something is logged with any higher level than `info`.
	#[arg(long)]
	pub detailed_log_output: bool,

	/// Disable log color output.
	#[arg(long)]
	pub disable_log_color: bool,
}

impl SharedParams {
	/// Specify custom base path.
	pub fn base_path(&self) -> Result<Option<BasePath>, crate::Error> {
		match &self.base_path {
			Some(r) => Ok(Some(r.clone().into())),
			// If `dev` is enabled, we use the temp base path.
			None if self.is_dev() => Ok(Some(BasePath::new_temp_dir()?)),
			None => Ok(None),
		}
	}

	/// Specify the development chain.
	pub fn is_dev(&self) -> bool {
		self.dev
	}

	/// Get the filters for the logging
	pub fn log_filters(&self) -> &[String] {
		&self.log
	}

	/// Should the detailed log output be enabled.
	pub fn detailed_log_output(&self) -> bool {
		self.detailed_log_output
	}

	/// Should the log color output be disabled?
	pub fn disable_log_color(&self) -> bool {
		self.disable_log_color
	}
}
