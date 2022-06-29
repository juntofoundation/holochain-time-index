use chrono::{DateTime, Utc};
use holochain_deterministic_integrity::prelude::*;

use hc_time_index::{IndexableEntry};

#[derive(Clone, Deserialize, Serialize, Debug, SerializedBytes)]
pub struct TestEntry {
    pub title: String,
    pub created: DateTime<Utc>,
}

app_entry!(TestEntry);

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def(visibility = "public")]
    TestEntry(TestEntry)
}

impl IndexableEntry for TestEntry {
    fn entry_time(&self) -> DateTime<Utc> {
        self.created
    }

    fn hash(&self) -> ExternResult<EntryHash> {
        hash_entry(self)
    }
}