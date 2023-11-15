/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

mod external_crate;

use external_crate::AnyApplicationAdapter;
use lexa_kernel::{ApplicationStartupExtension, LoaderExtension};

// -------- //
// Constant //
// -------- //

const APPLICATION_NAME: &'static str = "lexa-app";
const APPLICATION_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APPLICATION_ROOT_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct MyConfig
{
	pub my_field: String,
}

// ---- //
// Main //
// ---- //

fn main()
{
	type Application = lexa_kernel::Kernel<AnyApplicationAdapter>;

	let application = Application::new(
		APPLICATION_NAME,
		APPLICATION_VERSION,
		APPLICATION_ROOT_DIR,
	)
		// NOTE: Par défaut, le répertoire de configuration est configuré sur
		//       APPLICATION_ROOT_DIR + "/config"
		.define_config_directory("examples/config")
		// NOTE: Par défaut, l'extension d'un fichier de configuration est
		//       configurée sur l'extension YAML (LoaderExtension::YAML).
		.define_loader_extension(LoaderExtension::YAML)
	;

	let my_config: MyConfig = application
		.fetch_config("my-config")
		.expect("Impossible de récupérer la configuration de MyConfig");
	dbg!(&my_config);

	application.run();
}
