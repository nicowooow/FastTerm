slint::include_modules!();

fn main() {
    let app = MainWindow::new().unwrap();
    // .unwrap() extrae el resultado exitoso de la compilación o detiene el programa si hubo un error.

    app.run().unwrap();

    // ejecutamos la aplicacion
}
