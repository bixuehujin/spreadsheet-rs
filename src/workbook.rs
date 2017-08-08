use std::ffi::CString;
use std::path::Path;
use std::ptr;

pub enum lxw_workbook {}
pub enum lxw_worksheet {}
pub enum lxw_error{}

#[link(name="xlsxwriter")]
extern {
    fn workbook_new(name: *const i8) -> &lxw_workbook;
    fn workbook_add_worksheet(book: &lxw_workbook, name: *const i8) -> &lxw_worksheet;
    fn workbook_close(book: &lxw_workbook) -> lxw_error;
}

pub struct Workbook<'a> {
    intern: &'a lxw_workbook,
}

impl<'a> Workbook<'a> {

    pub fn new(filename: &str) -> Workbook {
        let cfilename = CString::new(filename).unwrap();

        Workbook {
            intern: unsafe { workbook_new(cfilename.as_ptr()) },
        }
    }

    pub fn add_worksheet(&self, sheetname: &str) -> &lxw_worksheet {
        let name = CString::new(sheetname).unwrap();

        unsafe {
            workbook_add_worksheet(self.intern, name.as_ptr())
        }
    }

    pub fn close(&self) {
        unsafe{ workbook_close(self.intern) };
    }
}

#[test]
fn test_new_workbook() {
    let path = "tmp/test.xlsx";
    let workbook = Workbook::new(path);
    workbook.add_worksheet("my sheet");
    workbook.close();

    assert!(Path::new(path).exists())
}
