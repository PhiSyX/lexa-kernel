// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

#[macro_export]
macro_rules! using {
	($($vis:vis $name:ident,)*) => {
		$($vis mod $name ;)*
		$($vis use self:: $name :: *;)*
	};

	($($vis:vis $directory:ident / { $($module_vis:vis $name:ident,)* };)*) => {
		$(
			$vis mod $directory {
				$(mod $name ;)*
				$($module_vis use self:: $name ::*;)*
			}
		)*
		$($vis use self:: $directory ::*;)*
	};
}

#[macro_export]
macro_rules! public_using {
	($($name:ident,)*) => {
		$crate::using! { $(pub $name,)* }
	};

	($($directory:ident / { $($name:ident,)* };)*) => {
		$crate::using! { $(pub $directory / { $( pub $name,)* };)* }
	};
}
