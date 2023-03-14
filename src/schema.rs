//! Core entities.
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// All the schema information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// List of tables in the database.
    pub tables: Vec<Table>,
    /// List of relations in the database.
    pub relations: Vec<Relation>,
    /// Partial Tables
    pub partial_tables: HashMap<String, Vec<String>>
}

/// Table information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
    pub column: String,
    pub data_type: String,
    pub index: i32,
    pub default: Option<String>,
    pub nullable: String,
    pub max_chars: Option<i32>,
    pub description: Option<String>,
    pub table_description: Option<String>, // Redundant but easiest way to get it.
    pub primary_key: bool
}

/// Table information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    /// Table name.
    pub name: String,
    /// Table Description
    pub description: Option<String>,
    /// List of fields.
    pub fields: Vec<TableColumn>,
}

/// Row description.
//#[derive(Debug)]
//pub struct Field(pub FieldName, pub FieldType);

/// Relation node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    /// Table that the constraint references.
    pub on_table: TableName,
    /// Field that the constraint references.
    pub on_field: FieldName,
    /// Table which the fk references.
    pub to_table: TableName,
    /// Field which the fk references.
    pub to_field: FieldName,
}

pub type TableName = String;
pub type FieldName = String;
//pub type FieldType = String;

/// Index Definition
pub struct Index {
    pub table: TableName,
    pub name: String,
    pub primary: bool,
    pub unique: bool,
    pub fields: Vec<String>,
}