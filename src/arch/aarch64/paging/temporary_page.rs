//! Temporarily map a page
//! From [Phil Opp's Blog](http://os.phil-opp.com/remap-the-kernel.html)

use memory::Frame;

use super::{ActivePageTable, Page, VirtualAddress};
use super::entry::EntryFlags;
use super::table::{Table, Level1};

pub struct TemporaryPage {
    page: Page,
}

impl TemporaryPage {
    pub fn new(page: Page) -> TemporaryPage {
        TemporaryPage {
            page: page,
        }
    }

    pub fn start_address (&self) -> VirtualAddress {
        self.page.start_address()
    }

    /// Maps the temporary page to the given frame in the active table.
    /// Returns the start address of the temporary page.
    pub fn map(&mut self, frame: Frame, flags: EntryFlags, active_table: &mut ActivePageTable) -> VirtualAddress {
        self.page.start_address()
    }

    /// Maps the temporary page to the given page table frame in the active
    /// table. Returns a reference to the now mapped table.
    pub fn map_table_frame(&mut self, frame: Frame, flags: EntryFlags, active_table: &mut ActivePageTable) -> &mut Table<Level1> {
        unsafe { &mut *(self.map(frame, flags, active_table).get() as *mut Table<Level1>) }
    }

    /// Unmaps the temporary page in the active table.
    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
    }
}
