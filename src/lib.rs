use gldf_rs::gldf::GldfProduct;
use phper::{
    arrays::{InsertKey, ZArray},
    functions::Argument,
    modules::Module,
    php_get_module,
    values::{ZVal},
};

fn gldf_to_xml(arguments: &mut [ZVal]) -> phper::Result<String> {
    let path = arguments[0].expect_z_str()?.to_str()?;
    let loaded: GldfProduct = GldfProduct::load_gldf(path).unwrap();
    Ok(loaded.to_xml().unwrap())
}

fn gldf_to_json(arguments: &mut [ZVal]) -> phper::Result<String> {
    let path = arguments[0].expect_z_str()?.to_str()?;
    let loaded: GldfProduct = GldfProduct::load_gldf(path).unwrap();
    Ok(loaded.to_json().unwrap())
}

fn json_from_xml_str(arguments: &mut [ZVal]) -> phper::Result<String> {
    let xml_str = arguments[0].expect_z_str()?.to_str()?;
    let loaded: GldfProduct = GldfProduct::from_xml(xml_str).unwrap();
    Ok(loaded.to_json().unwrap())
}

fn xml_from_json(arguments: &mut [ZVal]) -> phper::Result<String> {
    let json_str = arguments[0].expect_z_str()?.to_str()?;
    let loaded: GldfProduct = GldfProduct::from_json(json_str).unwrap();
    Ok(loaded.to_xml().unwrap())
}

/// This is the entry of php extension, the attribute macro `php_get_module`
/// will generate the `extern "C" fn`.
#[php_get_module]
pub fn get_module() -> Module {
    // New `Module` with extension info.
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Register function `say_hello`, with one argument `name`.
    module
        .add_function("gldf_to_xml", gldf_to_xml)
        .argument(Argument::by_val("path"));
    module
        .add_function("gldf_to_json", gldf_to_json)
        .argument(Argument::by_val("path"));
    module
        .add_function("json_from_xml_str", json_from_xml_str)
        .argument(Argument::by_val("xml_str"));
    module
        .add_function("xml_from_json", xml_from_json)
        .argument(Argument::by_val("json_str"));

    module
}