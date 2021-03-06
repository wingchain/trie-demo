// Copyright 2019, 2020 Wingchain
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use hashlink::linked_hash_map::Entry;
use hashlink::LinkedHashMap;

#[test]
fn test_hashlink() {
	let mut a = LinkedHashMap::new();

	a.insert("b", "bb'");
	a.insert("a", "aa");

	match a.entry("b") {
		Entry::Occupied(mut entry) => {
			entry.to_back();
		}
		_ => (),
	}

	for (k, v) in a {
		println!("{:?} = {:?}", k, v);
	}
}
