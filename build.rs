
fn main() {
    // Aquí es donde SÍ va el slint_build para decirle a Rust dónde está el diseño
    slint_build::compile("src/ui/app.slint").unwrap();
//  de Cargo.toml llamamos a la dependencia de slint_build
// y le decimos que compile el fichero llamado app.slint en el directorio ui
}