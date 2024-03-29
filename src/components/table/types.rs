use super::error::Result;
use serde::Serialize;
use serde_value::Value;
use std::fmt;
use yew::Html;

pub trait TableData: 'static + Default + Clone + PartialOrd + Serialize {
    /// Returns the Html representation of a field. When None, the field is not rendered.
    fn get_field_as_html(&self, field_name: &str) -> Result<Html>;

    /// Returns a table value given its field name. This value is used as a sorting key for the corresponding column.
    fn get_field_as_value(&self, field_name: &str) -> Result<Value>;

    fn matches_search(&self, needle: Option<String>) -> bool;
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Column {
    pub name: String,
    pub short_name: Option<String>,
    pub data_property: Option<String>,
    pub orderable: bool,
    pub header_classes: Vec<String>,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.short_name.as_ref().unwrap_or(&self.name))
    }
}

#[derive(Default)]
pub struct ColumnBuilder {
    name: String,
    short_name: Option<String>,
    data_property: Option<String>,
    orderable: bool,
    header_classes: Vec<String>,
}

impl ColumnBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            short_name: None,
            data_property: None,
            orderable: false,
            header_classes: vec![],
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> Column {
        Column {
            name: self.name,
            short_name: self.short_name,
            data_property: self.data_property,
            orderable: self.orderable,
            header_classes: self.header_classes,
        }
    }

    pub const fn orderable(mut self, orderable: bool) -> Self {
        self.orderable = orderable;
        self
    }

    pub fn data_property(mut self, data_property: &str) -> Self {
        self.data_property = Some(data_property.to_string());
        self
    }

    pub fn short_name(mut self, short_name: &str) -> Self {
        self.short_name = Some(short_name.to_string());
        self
    }

    pub fn header_class(mut self, class: &str) -> Self {
        self.header_classes.push(class.to_string());
        self
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TableOrder {
    Unordered = 0,
    Ascending,
    Descending,
}

impl Default for TableOrder {
    fn default() -> Self {
        Self::Unordered
    }
}

impl TableOrder {
    pub const fn rotate(self) -> Self {
        use TableOrder::{Ascending, Descending, Unordered};
        match self {
            Unordered => Ascending,
            Ascending => Descending,
            Descending => Unordered,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Default)]
pub struct TableState {
    pub order: Vec<TableOrder>,
}

/// The a table with columns holding data.
#[derive(Clone, Eq, PartialEq, Default)]
pub struct Table<T>
where
    T: TableData + fmt::Debug,
{
    /// The order of the columns determines the order in which they are displayed.
    pub columns: Vec<Column>,
    pub data: Vec<T>,
    pub state: TableState,
    pub orderable: bool,
}
