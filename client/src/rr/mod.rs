/*
 * Copyright (C) 2015 Benjamin Fry <benjaminfry@me.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Resource record related components, e.g. `Name` aka label, `Record`, `RData`, ...

pub mod dnssec;
mod lower_name;
mod rr_key;

use trust_dns_proto::rr;
pub use trust_dns_proto::rr::dns_class;
pub use trust_dns_proto::rr::domain;
pub use trust_dns_proto::rr::record_data;
pub use trust_dns_proto::rr::record_type;
pub use trust_dns_proto::rr::resource;

pub use self::rr::domain::{IntoName, Name, Label};
pub use self::dns_class::DNSClass;
pub use self::lower_name::LowerName;
pub use self::record_data::RData;
pub use self::record_type::RecordType;
pub use self::resource::Record;
pub use self::rr::IntoRecordSet;
pub use self::rr::RecordSet;
pub use self::rr_key::RrKey;

/// All record data structures and related serialization methods
pub mod rdata {
    pub use trust_dns_proto::rr::rdata::*;
    pub use trust_dns_proto::rr::dnssec::rdata::*;
}
