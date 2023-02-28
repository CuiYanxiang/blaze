// Copyright 2022 The Blaze Authors
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

mod onheap_spill;

use datafusion::common::Result;
use jni::objects::JObject;
use jni::sys::jlong;
use blaze_commons::{jni_call, jni_call_static};
pub use crate::spill::onheap_spill::OnHeapSpill;

pub fn get_spills_disk_usage(spill_ids: &[i32]) -> Result<u64> {
    let hsm = jni_call_static!(JniBridge.getTaskOnHeapSpillManager() -> JObject)?;
    let mut total_usage = 0u64;

    for &id in spill_ids {
        let usage = jni_call!(BlazeOnHeapSpillManager(hsm)
            .getSpillDiskUsage(id) -> jlong)?;
        total_usage += usage as u64;
    }
    Ok(total_usage)
}