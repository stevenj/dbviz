//! Loader for postgresql.

use crate::opts;
use crate::schema::{Relation, Schema, Table, TableColumn};

use anyhow::Result;
use itertools::Itertools;
use postgres::tls::NoTls;
use postgres::Client;
use postgres::Row;

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

/// Struct that manages the loading and implements `Loader` trait.
pub struct Conn {
    pg_client: RefCell<Client>,
    schema: String,
    opts: opts::Opts,
}

impl Conn {
    // Make a new postgres connection
    pub fn new(opts: &opts::Opts) -> Result<Conn> {
        let pg_client = postgres::Config::new()
            .user(&opts.pg_opts.username)
            .password(&opts.pg_opts.password)
            .dbname(&opts.pg_opts.database)
            .host(&opts.pg_opts.hostname)
            .connect(NoTls)?;

        let pg_client = RefCell::new(pg_client);
        let schema = opts.pg_opts.schema.clone();
        Ok(Conn {
            pg_client,
            schema,
            opts: opts.clone(),
        })
    }

    // Do we include this table name?
    fn include_table(&self, name: &String) -> bool {
        match &self.opts.include_tables {
            Some(inc) => inc.contains(name),
            None => true,
        }
    }

    // Do we include this table name?
    fn exclude_table(&self, name: &String) -> bool {
        match &self.opts.exclude_tables {
            Some(inc) => inc.contains(name),
            None => false,
        }
    }

    pub fn load(&self) -> Result<Schema> {
        let mut client = self.pg_client.borrow_mut();
        let tables_rows = client.query(tables_query(), &[&self.schema])?;
        let relations_rows = client.query(relations_query(), &[&self.schema])?;

        let tables: Vec<_> = tables_rows
            .into_iter()
            .group_by(|row| row.get(0))
            .into_iter()
            .filter(|(name, _rows)| self.include_table(name) && !self.exclude_table(name))
            .map(|(name, rows)| {
                let fields: Vec<_> = rows
                    .into_iter()
                    .map(|row| {
                        let mut field: TableColumn = row.try_into().unwrap();
                        let desc = match field.description {
                            Some(desc) => match self.opts.column_description_wrap {
                                Some(wrap) => Some(textwrap::fill(&desc, wrap)),
                                None => Some(desc),
                            },
                            None => None,
                        };
                        field.description = desc;

                        field
                    })
                    .collect();

                let desc = match &fields[0].table_description {
                    Some(desc) => match self.opts.table_description_wrap {
                        Some(wrap) => Some(textwrap::fill(desc, wrap)),
                        None => Some(desc).cloned(),
                    },
                    None => None,
                };

                Table {
                    name,
                    description: desc,
                    fields,
                }
            })
            .collect();

        let relations: Vec<_> = relations_rows
            .into_iter()
            .map(|row| {
                let relation: Relation = row.try_into().unwrap();
                relation
            })
            .filter(|relation| {
                self.include_table(&relation.on_table)
                    && self.include_table(&relation.to_table)
                    && !self.exclude_table(&relation.on_table)
                    && !self.exclude_table(&relation.to_table)
            })
            .collect();

        Ok(Schema { tables, relations })
    }
}

impl TryFrom<Row> for Relation {
    type Error = String;

    fn try_from(row: Row) -> std::result::Result<Self, String> {
        let fields: HashMap<String, String> = row
            .columns()
            .iter()
            .enumerate()
            .map(|(i, c)| (c.name().to_string(), row.get(i)))
            .collect();

        Ok(Self {
            on_table: fetch_field(&fields, "on_table")?,
            on_field: fetch_field(&fields, "on_field")?,
            to_table: fetch_field(&fields, "to_table")?,
            to_field: fetch_field(&fields, "to_field")?,
        })
    }
}

impl TryFrom<Row> for TableColumn {
    type Error = String;

    fn try_from(row: Row) -> std::result::Result<Self, String> {
        Ok(Self {
            column: row.get(1),
            data_type: row.get(2),
            index: row.get(3),
            default: row.get(4),
            nullable: row.get(5),
            max_chars: row.get(6),
            description: row.get(7),
            table_description: row.get(8),
        })
    }
}

fn fetch_field(map: &HashMap<String, String>, key: &str) -> std::result::Result<String, String> {
    map.get(key)
        .cloned()
        .ok_or(format!("could not find field {key}"))
}

// Query all tables and columns
fn tables_query() -> &'static str {
    "
    select table_name, column_name, data_type, ordinal_position, column_default, is_nullable, character_maximum_length, col_description(table_name::regclass, ordinal_position), obj_description(table_name::regclass)
      from information_schema.columns
     where table_schema = $1
     order by table_name, ordinal_position
    "
}

// Query all relationships
fn relations_query() -> &'static str {
    "
    select *
      from (
        select ns.nspname AS schemaname,
               cl.relname AS on_table,
               attr.attname AS on_field,
               clf.relname AS to_table,
               attrf.attname AS to_field
          from pg_constraint con
                 join pg_class cl
                     on con.conrelid = cl.oid
                 join pg_namespace ns
                     on cl.relnamespace = ns.oid
                 join pg_class clf
                     on con.confrelid = clf.oid
                 join pg_attribute attr
                     on attr.attnum = ANY(con.conkey) and
                 attr.attrelid = con.conrelid
                 join pg_attribute attrf
                     on attrf.attnum = ANY(con.confkey) and
                 attrf.attrelid = con.confrelid
      ) as fk
     where fk.schemaname = $1
    "
}