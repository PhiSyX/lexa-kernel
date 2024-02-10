// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use lexa_logger::LoggerBuilder;

// ---- //
// Type //
// ---- //

#[cfg(not(feature = "tracing"))]
pub type LoggerFilter = log::LevelFilter;
#[cfg(feature = "tracing")]
pub type LoggerFilter = tracing::level_filters::LevelFilter;

#[cfg(not(feature = "tracing"))]
pub type LoggerBuilderError = log::SetLoggerError;
#[cfg(feature = "tracing")]
pub type LoggerBuilderError = &'static str;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct LoggerSettings
{
	/// Pré-réglage d'un logger.
	#[serde(default)]
	pub preset: LoggerSettingsPreset,

	// Paramètres du logger
	/// Inclure des couleurs dans les informations d'un log.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub colorized: Option<bool>,
	/// Niveau de log maximal.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_level: Option<LoggerSettingsLevel>,
	/// Filtre des cibles (modules, crates).
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub target_filters: Vec<String>,
	/// Inclure le temps dans un log.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timestamp: Option<bool>,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
#[derive(serde::Deserialize)]
pub enum LoggerSettingsPreset
{
	#[default]
	#[serde(rename = "default")]
	Default,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum LoggerSettingsLevel
{
	DEBUG,
	ERROR,
	INFO,
	TRACE,
	WARNING,
}

// -------------- //
// Implémentation //
// -------------- //

impl LoggerSettings
{
	/// Nom du fichier de configuration du logger.
	pub const FILENAME: &'static str = "logger";
}

impl LoggerSettings
{
	pub fn make_builder(&self, level: impl Into<LoggerFilter>)
		-> Result<(), LoggerBuilderError>
	{
		let mut builder = match self.preset {
			| LoggerSettingsPreset::Default => {
				lexa_logger::LoggerStdout::builder()
					.with_color(true)
					.with_timestamp(true)
					.with_level(level)
			}
		};

		if let Some(c) = self.colorized {
			builder = builder.with_color(c);
		}

		if let Some(t) = self.timestamp {
			builder = builder.with_timestamp(t);
		}

		if let Some(ml) = self.max_level {
			builder = builder.with_level(ml);
		}

		use lexa_wildcard_matching::WildcardMatching;

		for target_filter in self.target_filters.clone() {
			let dependency = target_filter.clone();
			builder = builder.filter(
				move |metadata| metadata.target().iswm(&target_filter),
				dependency
			);
		}

		lexa_logger::LoggerInitiator::stdout(builder)
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<lexa_logger::Settings> for LoggerSettings
{
	fn from(ll_settings: lexa_logger::Settings) -> Self
	{
		Self {
			colorized: Some(ll_settings.colorized),
			max_level: Some(ll_settings.max_level.into()),
			target_filters: ll_settings.target_filters,
			timestamp: Some(ll_settings.timestamp),
			..Default::default()
		}
	}
}

impl From<lexa_logger::SettingsLevel> for LoggerSettingsLevel
{
	fn from(level: lexa_logger::SettingsLevel) -> Self
	{
		match level {
			| lexa_logger::SettingsLevel::DEBUG => Self::DEBUG,
			| lexa_logger::SettingsLevel::ERROR => Self::ERROR,
			| lexa_logger::SettingsLevel::INFO => Self::INFO,
			| lexa_logger::SettingsLevel::TRACE => Self::TRACE,
			| lexa_logger::SettingsLevel::WARNING => Self::WARNING,
		}
	}
}

impl From<LoggerSettingsLevel> for log::LevelFilter
{
	fn from(level: LoggerSettingsLevel) -> Self
	{
		match level {
			| LoggerSettingsLevel::DEBUG => Self::Debug,
			| LoggerSettingsLevel::ERROR => Self::Error,
			| LoggerSettingsLevel::INFO => Self::Info,
			| LoggerSettingsLevel::TRACE => Self::Trace,
			| LoggerSettingsLevel::WARNING => Self::Warn,
		}
	}
}

#[cfg(feature = "tracing")]
impl From<LoggerSettingsLevel> for tracing::level_filters::LevelFilter
{
	fn from(level: LoggerSettingsLevel) -> Self
	{
		match level {
			| LoggerSettingsLevel::DEBUG => Self::DEBUG,
			| LoggerSettingsLevel::ERROR => Self::ERROR,
			| LoggerSettingsLevel::INFO => Self::INFO,
			| LoggerSettingsLevel::TRACE => Self::TRACE,
			| LoggerSettingsLevel::WARNING => Self::WARN,
		}
	}
}
