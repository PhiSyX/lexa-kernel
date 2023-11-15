/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use lexa_kernel::{
	ApplicationAdapterInterface,
	ApplicationStartupExtension,
	AsyncApplicationStartupExtension,
};

pub struct AnyApplicationAdapter;

impl ApplicationAdapterInterface for AnyApplicationAdapter
{
	fn new() -> Self
	{
		Self
	}
}

impl ApplicationStartupExtension for AnyApplicationAdapter
{
	fn run(self)
	{
		println!("Sync AnyApplicationAdapter");
	}
}

impl AsyncApplicationStartupExtension for AnyApplicationAdapter
{
	async fn run(self)
	{
		println!("Async AnyApplicationAdapter");
	}
}
