// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

#![forbid(unsafe_code)]

#[macro_use]
extern crate thiserror;

pub mod commands;
pub mod errors;
pub mod helpers;

pub(crate) type Network = snarkvm::prelude::Testnet3;
pub(crate) type Aleo = snarkvm::circuit::AleoV0;

#[cfg(feature = "account")]
pub use aleo_account as account;
#[cfg(feature = "wasm")]
pub use aleo_wasm as wasm;
