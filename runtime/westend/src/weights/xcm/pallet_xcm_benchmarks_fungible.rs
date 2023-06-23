// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_xcm_benchmarks::fungible`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_xcm_benchmarks::fungible
// --chain=westend-dev
// --header=./file_header.txt
// --template=./xcm/pallet-xcm-benchmarks/template.hbs
// --output=./runtime/westend/src/weights/xcm/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_xcm_benchmarks::fungible`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	pub fn withdraw_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `3593`
		// Minimum execution time: 22_158_000 picoseconds.
		Weight::from_parts(22_428_000, 3593)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	pub fn transfer_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `6196`
		// Minimum execution time: 46_062_000 picoseconds.
		Weight::from_parts(46_579_000, 6196)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub fn transfer_reserve_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `270`
		//  Estimated: `20911`
		// Minimum execution time: 66_610_000 picoseconds.
		Weight::from_parts(67_366_000, 20911)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub fn initiate_reserve_withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `14210`
		// Minimum execution time: 27_155_000 picoseconds.
		Weight::from_parts(27_545_000, 14210)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Balances InactiveIssuance (r:1 w:1)
	// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	pub fn receive_teleported_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `5094`
		// Minimum execution time: 22_079_000 picoseconds.
		Weight::from_parts(22_664_000, 5094)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	pub fn deposit_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3593`
		// Minimum execution time: 24_546_000 picoseconds.
		Weight::from_parts(24_959_000, 3593)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub fn deposit_reserve_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `17803`
		// Minimum execution time: 47_788_000 picoseconds.
		Weight::from_parts(48_039_000, 17803)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Balances InactiveIssuance (r:1 w:1)
	// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub fn initiate_teleport() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `19304`
		// Minimum execution time: 49_509_000 picoseconds.
		Weight::from_parts(50_031_000, 19304)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}
