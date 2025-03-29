use cli::typegen::generate_typescript_bindings;
// Import types and modules to generate types from






fn main() {
    generate_typescript_bindings("frontend/src/bindings/crayon").unwrap();
    generate_typescript_bindings("app/src/ts").unwrap();

}
