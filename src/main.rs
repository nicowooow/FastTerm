slint::include_modules!();

fn main() {
    // hacemos que en las variables de entorno se ponga el balor de slint_backend en software, para que use una CPU
    // unsafe {
    //     std::env::set_var("SLINT_BACKEND", "software");
    // }

    let app = App::new().unwrap();
    // .unwrap() extrae el resultado exitoso de la compilación o detiene el programa si hubo un error.

    app.run().unwrap();

    // ejecutamos la aplicacion
}
