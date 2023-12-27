use std::path::{Path, PathBuf};

use spreadsheet_ods as ods;
use icu_locid as icu;
use anyhow::{Context, Error};
use chrono;

pub mod game;

pub const EXT_FODS: &'static str = ".fods";
pub const CSV_JOINT: &'static str = ", ";

pub trait SheetTrait {
    fn title(&self) -> &'static str;
    fn orientation(&self) -> crate::spreadsheet::SheetOrientation;
    fn headings(&self) -> Box<dyn Iterator<Item = Box<dyn HeadingTrait<SheetEnum = Self>>>>;
}

pub trait HeadingTrait {
    type SheetEnum: SheetTrait;

    fn sheet(&self) -> Self::SheetEnum;
    fn title(&self) -> &'static str;
    fn rowcol(&self) -> RowCol;


    fn next(&self, index: u32) -> RowCol {
        match self.sheet().orientation() {
            SheetOrientation::Vertical   => RowCol(self.row() + 1 + index, self.column()),
            SheetOrientation::Horizontal => RowCol(self.row(), self.column() + 1 + index)
        }
    }

    fn row(&self) -> u32 {
        self.rowcol().row()
    }

    fn column(&self) -> u32 {
        self.rowcol().column()
    }
}

pub enum SheetOrientation {
    Vertical,
    Horizontal
}

impl SheetOrientation {
    const fn heading_cellstyle_name(&self) -> &'static str {
        match self {
            Self::Vertical => "heading_vertical",
            Self::Horizontal => "heading_horizontal"
        }
    }
}

pub struct RowCol(pub u32, pub u32);

impl RowCol {
    pub const fn row(&self) -> u32 {
        self.0
    }

    pub const fn column(&self) -> u32 {
        self.1
    }
}

pub struct WorkbookBuilder {
    workbook: ods::WorkBook
}

impl WorkbookBuilder {
    pub fn new() -> Self {
        let mut workbook = ods::WorkBook::new(icu::locale!("en_US"));

        let mut header_vertical_style = ods::CellStyle::new_empty();
        header_vertical_style.set_name(SheetOrientation::Vertical.heading_cellstyle_name());
        header_vertical_style.set_font_bold();
        header_vertical_style.set_text_align(ods::style::units::TextAlign::Center);

        let mut header_horizontal_style = ods::CellStyle::new_empty();
        header_horizontal_style.set_name(SheetOrientation::Horizontal.heading_cellstyle_name());
        header_horizontal_style.set_font_bold();
        header_horizontal_style.set_text_align(ods::style::units::TextAlign::Left);
        
        workbook.add_cellstyle(header_vertical_style);
        workbook.add_cellstyle(header_horizontal_style);

        Self {
            workbook
        }
    }

    pub fn open(filename_prefix: &str, dir: &Path) -> anyhow::Result<Self> {
        let filepath = dir.join(filename_prefix.to_string() + EXT_FODS);
        Ok(Self {
            workbook: ods::read_fods(filepath)?
        })
    }

    pub fn append_sheets<T>(&mut self, sheets: impl Iterator<Item = T>) -> &mut Self
    where
        T: SheetTrait
    {
        for sheet_enum in sheets {
            let cellstyle = ods::CellStyleRef::from(sheet_enum.orientation().heading_cellstyle_name());
            let mut sheet = ods::Sheet::new(sheet_enum.title());

            for heading in sheet_enum.headings() {
                let (row, column) = (heading.row(), heading.column());
                sheet.set_styled_value(row, column, heading.title(), &cellstyle);
                let mut annotation = ods::draw::Annotation::new("This is an annotation");
                annotation.set_display(false);
                sheet.set_annotation(row, column, annotation); //TODO: implement annotations in sheetenum
            }

            self.workbook.push_sheet(sheet);
        }
        self
    }

    pub fn append_sheet<T>(&mut self, headings_iter: impl Iterator<Item = T>) -> &mut Self
    where
        T: HeadingTrait
    {
        let mut headings_iter = headings_iter.peekable();
        let sheet_enum = headings_iter.peek().unwrap().sheet();
        let cellstyle = ods::CellStyleRef::from(sheet_enum.orientation().heading_cellstyle_name());

        let mut sheet = ods::Sheet::new(sheet_enum.title());

        for heading in headings_iter {
            sheet.set_styled_value(heading.row(), heading.column(), heading.title(), &cellstyle);
        }

        self.workbook.push_sheet(sheet);
        self
    }

    fn sheet_mut(&mut self, heading: &impl HeadingTrait) -> anyhow::Result<&mut ods::Sheet> {
        let title = heading.sheet().title();
        let mut i = 0;
        let mut found = false;
        for sheet in self.workbook.iter_sheets() {
            if sheet.name() == title {
                found = true;
                break;
            }

            i += 1;
        }

        if found {
            return Ok(self.workbook.sheet_mut(i))
        }

        anyhow::bail!("Sheet not found: {title}");
    }

    pub fn cell<V: Into<ods::Value>>(&mut self, heading: impl HeadingTrait, index: u32, value: V)
        -> anyhow::Result<&mut Self>
    {
        let sheet = self.sheet_mut(&heading)?;
        let rowcol = heading.next(index);
        sheet.set_value(rowcol.row(), rowcol.column(), value);
        Ok(self)
    }

    pub fn write(mut self, filename: &str, dir: &Path) -> anyhow::Result<(ods::WorkBook, PathBuf)> {
        let filepath = dir.join(filename.to_string() + EXT_FODS);
        match ods::write_fods(&mut self.workbook, &filepath) {
            Ok(_) => Ok((self.workbook, filepath)),
            Err(e) => Err(Error::new(e))
        }
    }

    pub fn build(self) -> ods::WorkBook {
        self.workbook
    }
}


pub fn sheet<'workbook>(name: &str, workbook: &'workbook ods::WorkBook) -> anyhow::Result<&'workbook ods::Sheet> {
    workbook.iter_sheets().find(|s| s.name() == name)
        .context(format!("Spreadsheet not found: {name}"))
}

pub fn cell_string(heading: impl HeadingTrait, index: u32, sheet: &ods::Sheet) -> anyhow::Result<String> {
    let rowcol = heading.next(index);
    Ok(sheet.value(rowcol.row(), rowcol.column())
        .as_str_opt()
        .context(format!("Unable to read string cell({},{}): {}.{}",
            rowcol.row(), rowcol.column(), heading.sheet().title(), heading.title()))?
        .to_string())
}

pub fn cell_string_optional(heading: impl HeadingTrait, index: u32, sheet: &ods::Sheet) -> anyhow::Result<Option<String>> {
    let rowcol = heading.next(index);
    let value = sheet.value(rowcol.row(), rowcol.column());
    if value.value_type()  == ods::ValueType::Empty {
        return Ok(None);
    }

    let str = value.as_str_opt()
        .context(format!("Unable to read string cell({},{}): {}.{}",
            rowcol.row(), rowcol.column(), heading.sheet().title(), heading.title()))?;

    Ok(Some(str.to_string()))
}

pub fn cell_date(heading: impl HeadingTrait, index: u32, sheet: &ods::Sheet) -> anyhow::Result<chrono::NaiveDate> {
    let rowcol = heading.next(index);
    Ok(sheet.value(rowcol.row(), rowcol.column())
        .as_date_opt()
        .context(format!("Unable to read date cell({},{}): {}.{}",
            rowcol.row(), rowcol.column(), heading.sheet().title(), heading.title()))?)
}

pub fn cell_csv(heading: impl HeadingTrait, index: u32, sheet: &ods::Sheet) -> anyhow::Result<Vec<String>> {
    let rowcol = heading.next(index);
    Ok(sheet.value(rowcol.row(), rowcol.column())
        .as_str_opt()
        .context(format!("Unable to read comma-separated string cell({},{}): {}.{}",
            rowcol.row(), rowcol.column(), heading.sheet().title(), heading.title()))?
        .split(',')
        .map(|s| s.trim().to_string())
        .collect())
}

pub fn cell_float(heading: impl HeadingTrait, index: u32, sheet: &ods::Sheet) -> anyhow::Result<f64> {
    let rowcol = heading.next(index);
    Ok(sheet.value(rowcol.row(), rowcol.column())
        .as_f64_opt()
        .context(format!("Unable to read float cell({},{}): {}.{}",
            rowcol.row(), rowcol.column(), heading.sheet().title(), heading.title()))?)
}

pub fn cell_u8(heading: impl HeadingTrait, index: u32, sheet: &ods::Sheet) -> anyhow::Result<u8> {
    let rowcol = heading.next(index);
    Ok(sheet.value(rowcol.row(), rowcol.column())
        .as_u8_opt()
        .context(format!("Unable to read u8 cell({},{}): {}.{}",
            rowcol.row(), rowcol.column(), heading.sheet().title(), heading.title()))?)
}

