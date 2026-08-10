fn main() {
    // docs.rs and dependency-only checks do not provide GLib's native resource
    // compiler. The runtime examples still compile resources in normal builds.
    if std::env::var_os("DOCS_RS").is_some() {
        return;
    }

    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "compiled.gresource",
    );
}
