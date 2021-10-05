// Copyright (C) 2021 Quickwit, Inc.
//
// Quickwit is offered under the AGPL v3.0 and as commercial software.
// For commercial licensing, contact us at hello@quickwit.io.
//
// AGPL:
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

mod indexing_split_store;
mod local_split_store;

pub use indexing_split_store::{IndexingSplitStore, IndexingSplitStoreParams};
use local_split_store::LocalSplitStore;

/// An intermediate folder created at `cache_dir/INTERNAL_CACHE_DIR_NAME`
/// to hold the local files.
pub const INTERNAL_CACHE_DIR_NAME: &str = "split-cache";