use std::borrow::Cow;

use heck::ToLowerCamelCase;
use indoc::formatdoc;
use specta::{functions::FunctionDataType, ts, ts::ExportError, DataType, TypeMap};

use crate::{EventDataType, ExportLanguage, ItemType};

pub const DO_NOT_EDIT: &str = "// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.";
const CRINGE_ESLINT_DISABLE: &str = "/* eslint-disable */
";

pub type ExportConfig = crate::ExportConfig<specta::ts::ExportConfig>;

pub fn render_all_parts<T: ExportLanguage<Config = specta::ts::ExportConfig>>(
    commands: &[FunctionDataType],
    events: &[EventDataType],
    type_map: &TypeMap,
    cfg: &ExportConfig,
    dependant_types: &str,
    globals: &str,
) -> Result<String, T::Error> {
    let commands = T::render_commands(commands, type_map, cfg)?;
    let events = T::render_events(events, type_map, cfg)?;

    Ok(formatdoc! {
        r#"
            {DO_NOT_EDIT}

            {commands}

			{events}

			/** user-defined types **/

			{dependant_types}

			/** tauri-specta globals **/

            {globals}
        "#
    })
}

pub fn arg_names(args: &[(Cow<'static, str>, DataType)]) -> Vec<String> {
    args.iter()
        .map(|(name, _)| name.to_lower_camel_case())
        .collect::<Vec<_>>()
}

pub fn arg_usages(args: &[String]) -> Option<String> {
    (!args.is_empty()).then(|| format!("{{ {} }}", args.join(", ")))
}

fn return_as_result_tuple(expr: &str, as_any: bool) -> String {
    let as_any = as_any.then_some(" as any").unwrap_or_default();

    formatdoc!(
        r#"
		try {{
		    return {{ status: "ok", data: {expr} }};
		}} catch (e) {{
		    if(e instanceof Error) throw e;
		    else return {{ status: "error", error: e {as_any} }};
		}}"#
    )
}

pub fn maybe_return_as_result_tuple(expr: &str, typ: &DataType, as_any: bool) -> String {
    match typ {
        DataType::Result(_) => return_as_result_tuple(expr, as_any),
        _ => format!("return {expr};"),
    }
}

pub fn function(
    docs: &str,
    name: &str,
    args: &[String],
    return_type: Option<&str>,
    body: &str,
) -> String {
    let args = args.join(", ");
    let return_type = return_type
        .map(|t| format!(": Promise<{}>", t))
        .unwrap_or_default();

    formatdoc! {
        r#"
		{docs}async {name}({args}) {return_type} {{
		{body}
		}}"#
    }
}

fn tauri_invoke(name: &str, arg_usages: Option<String>) -> String {
    let arg_usages = arg_usages.map(|u| format!(", {u}")).unwrap_or_default();

    format!(r#"await TAURI_INVOKE("{name}"{arg_usages})"#)
}

pub fn handle_result(
    function: &FunctionDataType,
    type_map: &TypeMap,
    cfg: &ExportConfig,
) -> Result<String, ExportError> {
    Ok(match &function.result {
        Some(DataType::Result(t)) => {
            let (t, e) = t.as_ref();

            format!(
                "__Result__<{}, {}>",
                ts::datatype(&cfg.inner, t, type_map)?,
                ts::datatype(&cfg.inner, e, type_map)?
            )
        }
        t => ts::datatype(&cfg.inner, &t.clone().unwrap(), type_map)?,
    })
}

pub fn command_body(cfg: &ExportConfig, function: &FunctionDataType, as_any: bool) -> String {
    let name = cfg
        .plugin_name
        .apply_as_prefix(&function.name, ItemType::Command);

    maybe_return_as_result_tuple(
        &tauri_invoke(&name, arg_usages(&arg_names(&function.args))),
        &function.result.as_ref().unwrap(),
        as_any,
    )
}

pub fn events_map(events: &[EventDataType], cfg: &ExportConfig) -> String {
    events
        .iter()
        .map(|event| {
            let name_str = cfg.plugin_name.apply_as_prefix(event.name, ItemType::Event);
            let name_camel = event.name.to_lower_camel_case();

            format!(r#"{name_camel}: "{name_str}""#)
        })
        .collect::<Vec<_>>()
        .join(",\n")
}

pub fn events_types(
    events: &[EventDataType],
    cfg: &ExportConfig,
    type_map: &TypeMap,
) -> Result<Vec<String>, ExportError> {
    events
        .iter()
        .map(|event| {
            let name_camel = event.name.to_lower_camel_case();

            let typ = ts::datatype(&cfg.inner, &event.typ, type_map)?;

            Ok(format!(r#"{name_camel}: {typ}"#))
        })
        .collect()
}

pub fn events_data(
    events: &[EventDataType],
    cfg: &ExportConfig,
    type_map: &TypeMap,
) -> Result<(Vec<String>, String), ExportError> {
    Ok((
        events_types(events, cfg, type_map)?,
        events_map(events, cfg),
    ))
}

impl From<specta::ts::ExportConfig> for ExportConfig {
    fn from(config: specta::ts::ExportConfig) -> Self {
        Self {
            header: CRINGE_ESLINT_DISABLE.into(),
            ..Self::new(config)
        }
    }
}
