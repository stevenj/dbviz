//! Prints all the data in plain text format.
//!

use crate::drawer::Drawer;
use crate::schema::{Relation, Schema, Table};

use std::io::{Result, Write};

pub struct PlainText;

impl<W> Drawer<W> for PlainText
where
    W: Write,
{
    fn write(
        &self,
        schema: &Schema,
        buffer: &mut W,
        _include: Option<Vec<String>>,
        _exclude: Option<Vec<String>>,
        _title:Option<String>,
        _title_loc:&str,
        _title_size:u32,
        _title_color:&str,
        _direction:&str,
    ) -> Result<()> {
        buffer.write_all(b"=== Tables ===\n")?;

        for table in &schema.tables {
            write_table(table, buffer)?;
            buffer.write_all(b"\n")?;
        }

        buffer.write_all(b"=== Relations ===\n")?;

        for relation in &schema.relations {
            write_relation(relation, buffer)?;
            buffer.write_all(b"\n")?;
        }

        buffer.write_all(b"=== Done ===\n")?;

        Ok(())
    }
}

fn write_table<W>(table: &Table, buffer: &mut W) -> Result<()>
where
    W: Write,
{
    buffer.write_all(b"[")?;
    buffer.write_all(table.name.as_bytes())?;
    buffer.write_all(b"]\n")?;

    for field in &table.fields {
        // let Field(field_name, field_type) = field;
        buffer.write_all(field.column.as_bytes())?;
        buffer.write_all(b": ")?;
        buffer.write_all(field.data_type.as_bytes())?;
        buffer.write_all(b"\n")?;
    }

    buffer.write_all(b"\n")?;

    Ok(())
}

fn write_relation<W>(relation: &Relation, buffer: &mut W) -> Result<()>
where
    W: Write,
{
    buffer.write_all(relation.on_table.as_bytes())?;
    buffer.write_all(b":")?;
    buffer.write_all(relation.on_field.as_bytes())?;
    buffer.write_all(b" -> ")?;
    buffer.write_all(relation.on_table.as_bytes())?;
    buffer.write_all(b":")?;
    buffer.write_all(relation.on_field.as_bytes())?;
    buffer.write_all(b"\n")?;
    Ok(())
}
