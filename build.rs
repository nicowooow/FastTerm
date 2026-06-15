fn main() {
    // Le decimos a Cargo que si cambia styles.slint, vuelva a compilar la app
    println!("cargo:rerun-if-changed=src/styles/styles.slint");

    // Aquí es donde SÍ va el slint_build para decirle a Rust dónde está el diseño
    slint_build::compile("src/app.slint").unwrap();
    //  de Cargo.toml llamamos a la dependencia de slint_build
    // y le decimos que compile el fichero llamado app.slint en el directorio ui
}
