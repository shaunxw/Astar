// This file is part of Astar.

// Copyright (C) 2019-2023 Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

//! Runtime integration tests.

#![cfg(test)]

#[cfg(any(feature = "shibuya", feature = "shiden", feature = "astar"))]
mod setup;

#[cfg(any(feature = "shibuya"))]
mod proxy_new;

// Remove this once dApp staking v3 is integrated into Shiden & Astar
#[cfg(any(feature = "shiden", feature = "astar"))]
mod proxy_old;

#[cfg(any(feature = "shibuya", feature = "shiden", feature = "astar"))]
mod assets;

#[cfg(feature = "shibuya")]
mod xvm;

#[cfg(any(feature = "shibuya"))]
mod dispatch_precompile_filter_new;

// Remove this once dApp staking v3 is integrated into Shiden & Astar
#[cfg(any(feature = "shiden", feature = "astar"))]
mod dispatch_precompile_filter_old;

#[cfg(feature = "shibuya")]
mod unified_accounts;

#[cfg(any(feature = "shibuya"))]
mod assets_chain_extensions;
