fn main() {
    glib_build_tools::compile_resources(
        &["data"],
        "data/gresource.xml",
        "bike.gresource",
    );
}
