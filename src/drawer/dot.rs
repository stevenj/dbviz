//! Prints all the data in dot format.
//!

use crate::drawer::Drawer;
use crate::schema::{Relation, Schema, Table};

use std::io::{Result, Write};

/// Graphviz drawer.
pub struct Dot;

// Add Title
// label     = "The title of the graph"
// labelloc  =  b // t: Place the graph's title on top.
// fontsize  = 30 // Make title stand out by giving a large font size
// fontcolor = blue

// Allow Direction to be changed. LR / TB

fn graph_header(
    title: Option<String>,
    title_loc: &str,
    title_size: u32,
    title_color: &str,
    direction: &str,
) -> String {
    let title_header = match title {
        Some(title) => format!("label = \"{title}\"\n   labelloc = {title_loc}\n    fontsize = {title_size}\n   fontcolor = {title_color}\n"),
        None => String::new(),
    };

    format!(
        "digraph erd {{

    {title_header}

    graph [
    rankdir = \"{direction}\"
    ];
    node [
    fontsize = \"16\"
    shape = \"plaintext\"
    ];
    edge [
    ];
"
    )
}

const GRAPH_FOOTER: &str = "\n}\n";

impl<W> Drawer<W> for Dot
where
    W: Write,
{
    fn write(
        &self,
        schema: &Schema,
        buffer: &mut W,
        include: Option<Vec<String>>,
        exclude: Option<Vec<String>>,
        title:Option<String>,
        title_loc:&str,
        title_size:u32,
        title_color:&str,
        direction:&str,
    ) -> Result<()> {
        buffer
            .write_all(graph_header(title, title_loc, title_size, title_color, direction).as_bytes())?;

        //buffer.write_all(GRAPH_HEADER.as_bytes())?;

        let empty_vec: Vec<String> = vec![];
        let exclude_list = exclude.as_ref().unwrap_or(&empty_vec);

        for table in &schema.tables {
            let name_vec = vec![table.name.clone()];
            let include_list = include.as_ref().unwrap_or(&name_vec);

            if include_list.contains(&table.name) && !exclude_list.contains(&table.name) {
                write_table(table, buffer)?;
                buffer.write_all(b"\n")?;
            }
        }

        for relation in &schema.relations {
            let name_vec = vec![relation.on_table.clone(), relation.to_table.clone()];
            let include_list = include.as_ref().unwrap_or(&name_vec);

            if include_list.contains(&relation.on_table)
                && include_list.contains(&relation.to_table)
                && !exclude_list.contains(&relation.on_table)
                && !exclude_list.contains(&relation.to_table)
            {
                write_relation(relation, buffer)?;
            }
        }

        buffer.write_all(GRAPH_FOOTER.as_bytes())?;

        Ok(())
    }
}

fn write_table<W>(table: &Table, buffer: &mut W) -> Result<()>
where
    W: Write,
{
    buffer.write_all(table_header(&table.name).as_bytes())?;

    for field in &table.fields {
        //let Field(field_name, field_type) = field.column;
        buffer.write_all(table_field(&(field.column), &(field.data_type)).as_bytes())?;
    }

    buffer.write_all(table_footer().as_bytes())?;

    Ok(())
}

fn table_header(name: &str) -> String {
    format!(
        "  \"{name}\" [label=<
        <table
            border='0'
            cellborder='1'
            cellspacing='0'
            style='rounded' >
        <tr>
            <td
                colspan='2'
                bgcolor='#009879'
                port='__title'
            ><font color='white' face='Courier bold italic' point-size='20'><b>{name}</b></font></td>
        </tr>
        <tr>
            <td><font color='black' face='Courier bold' point-size='18'><b>Column</b></font></td>
            <td><font color='black' face='Courier bold' point-size='18'><b>Type</b></font></td>
        </tr>\n")
}

fn table_field(field_name: &str, field_type: &str) -> String {
    let field_type = match field_type {
        "character varying" => "varchar",
        "timestamp without time zone" => "timestamp",
        x => x,
    };
    format!(
        "            <tr><td port=\"{field_name}\" align='text'><font>{field_name}</font><br align='left'/></td><td><font>{field_type}</font></td></tr>\n"
    )
}

fn table_footer() -> String {
    String::from("          </table>>];\n")
}

fn write_relation<W>(relation: &Relation, buffer: &mut W) -> Result<()>
where
    W: Write,
{
    let relation = format!(
        "\"{}\":\"{}\" -> \"{}\":\"{}\"\n",
        relation.on_table, relation.on_field, relation.to_table, relation.to_field
    );

    buffer.write_all(relation.as_bytes())?;
    Ok(())
}
