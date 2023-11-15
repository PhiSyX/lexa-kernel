/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */
// Cette exemple à besoin de la dépendance `clap`.

mod external_crate;

use external_crate::AnyApplicationAdapter;
use lexa_kernel::{
	ApplicationCLIExtension,
	ApplicationCLIInterface,
	ApplicationStartupExtension,
};

// -------- //
// Constant //
// -------- //

const APPLICATION_NAME: &'static str = "lexa-app";
const APPLICATION_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APPLICATION_ROOT_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

// ---- //
// Type //
// ---- //

type Application =
	lexa_kernel::Kernel<AnyApplicationAdapter, (), ApplicationCLI>;

// --------- //
// Structure //
// --------- //

#[derive(clap::Parser)]
#[derive(Debug)]
#[derive(Clone)]
pub struct ApplicationCLI
{
	channel: String,
}

impl ApplicationCLIInterface for ApplicationCLI
{
	fn arguments() -> Self
	{
		use clap::Parser;
		Self::parse()
	}
}

// ---- //
// Main //
// ---- //

fn main()
{
	let application = Application::new(
		APPLICATION_NAME,
		APPLICATION_VERSION,
		APPLICATION_ROOT_DIR,
	)
		.include_cli_args()
	;

	dbg!(application.cli_args());

	application.run();
}
