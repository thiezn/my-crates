use super::fields::PathSegment;
use super::{FieldSelector, OutputFormat, OutputOptions};
use crate::error::{Error, Result};
use serde::Serialize;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(super) fn write_output<T: Serialize>(
    options: &OutputOptions,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
    data: &T,
    markdown_renderer: Option<&dyn Fn(&T) -> String>,
) -> Result {
    match options.format() {
        OutputFormat::Json => write_json(options, stdout, stderr, data),
        OutputFormat::Markdown => write_markdown(options, stdout, stderr, data, markdown_renderer),
    }
}

fn write_json<T: Serialize>(
    options: &OutputOptions,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
    data: &T,
) -> Result {
    let value = serialize_value(data)?;
    let filtered = filter_fields(value, options.field_selectors());

    if let Some(path) = options.output_path() {
        let mut file = create_file(path)?;
        serde_json::to_writer_pretty(&mut file, &filtered)?;
        writeln!(file)?;
        writeln!(stderr, "Wrote JSON output to {}", path.display())?;
        return Ok(());
    }

    serde_json::to_writer_pretty(&mut *stdout, &filtered)?;
    writeln!(stdout)?;
    Ok(())
}

fn write_markdown<T: Serialize>(
    options: &OutputOptions,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
    data: &T,
    markdown_renderer: Option<&dyn Fn(&T) -> String>,
) -> Result {
    if !options.field_selectors().is_empty() {
        return Err(Error::MarkdownFieldsUnsupported);
    }

    let markdown_renderer = markdown_renderer.ok_or(Error::MissingMarkdownRenderer)?;
    let markdown = markdown_renderer(data);

    if let Some(path) = options.output_path() {
        let mut file = create_file(path)?;
        write_text(&mut file, &markdown)?;
        writeln!(stderr, "Wrote Markdown output to {}", path.display())?;
        return Ok(());
    }

    write_text(stdout, &markdown)
}

fn create_file(path: &Path) -> Result<File> {
    File::create(path).map_err(|source| Error::WriteFile {
        path: path.to_path_buf(),
        source,
    })
}

fn write_text(writer: &mut dyn Write, text: &str) -> Result {
    write!(writer, "{text}")?;
    if !text.ends_with('\n') {
        writeln!(writer)?;
    }
    Ok(())
}

fn serialize_value<T: Serialize>(data: &T) -> Result<Value> {
    serde_json::to_value(data).map_err(Error::from)
}

fn filter_fields(value: Value, fields: &[FieldSelector]) -> Value {
    if fields.is_empty() {
        return value;
    }

    match value {
        Value::Array(items) => Value::Array(
            items
                .into_iter()
                .map(|item| extract_fields(&item, fields))
                .collect(),
        ),
        other => extract_fields(&other, fields),
    }
}

fn extract_fields(value: &Value, fields: &[FieldSelector]) -> Value {
    let mut map = serde_json::Map::new();
    for selector in fields {
        for (key, selected_value) in extract_path(value, selector.segments(), "") {
            map.insert(key, selected_value);
        }
    }

    Value::Object(map)
}

fn extract_path(value: &Value, segments: &[PathSegment], prefix: &str) -> Vec<(String, Value)> {
    if segments.is_empty() {
        return vec![(prefix.to_string(), value.clone())];
    }

    match &segments[0] {
        PathSegment::Key(key) => {
            if let Some(child) = value.get(key.as_str()) {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                extract_path(child, &segments[1..], &new_prefix)
            } else {
                Vec::new()
            }
        }
        PathSegment::Index(index) => {
            if let Some(child) = value.get(*index) {
                let new_prefix = if prefix.is_empty() {
                    format!("[{index}]")
                } else {
                    format!("{prefix}[{index}]")
                };
                extract_path(child, &segments[1..], &new_prefix)
            } else {
                Vec::new()
            }
        }
        PathSegment::Wildcard => {
            if let Value::Array(items) = value {
                items
                    .iter()
                    .enumerate()
                    .flat_map(|(index, child)| {
                        let new_prefix = if prefix.is_empty() {
                            format!("[{index}]")
                        } else {
                            format!("{prefix}[{index}]")
                        };
                        extract_path(child, &segments[1..], &new_prefix)
                    })
                    .collect()
            } else {
                Vec::new()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

    use super::*;
    use crate::output::FieldSelector;
    use serde_json::json;

    #[test]
    fn filter_empty_fields_no_change() {
        let value = json!({"a": 1, "b": 2});
        let result = filter_fields(value.clone(), &[]);
        assert_eq!(result, value);
    }

    #[test]
    fn filter_top_level() {
        let value = json!({"name": "x", "age": 1});
        let fields = vec![FieldSelector::parse("name").unwrap()];
        let result = filter_fields(value, &fields);
        assert_eq!(result, json!({"name": "x"}));
    }

    #[test]
    fn filter_nested_fields() {
        let value = json!({"resp": {"status": 200, "body": "ok"}, "id": 1});
        let fields = vec![FieldSelector::parse("resp.status").unwrap()];
        let result = filter_fields(value, &fields);
        assert_eq!(result, json!({"resp.status": 200}));
    }

    #[test]
    fn filter_array_of_objects() {
        let value = json!([
            {"name": "a", "resp": {"status": 200}},
            {"name": "b", "resp": {"status": 404}},
        ]);
        let fields = vec![FieldSelector::parse("resp.status").unwrap()];
        let result = filter_fields(value, &fields);
        assert_eq!(result, json!([{"resp.status": 200}, {"resp.status": 404}]));
    }

    #[test]
    fn extract_wildcard() {
        let value = json!({"items": [{"id": 1}, {"id": 2}]});
        let selector = FieldSelector::parse("items[*].id").unwrap();
        let results = extract_path(&value, selector.segments(), "");
        assert_eq!(
            results,
            vec![
                ("items[0].id".into(), json!(1)),
                ("items[1].id".into(), json!(2)),
            ]
        );
    }
}
