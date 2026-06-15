slint::include_modules!();

use slint::Model;
// usamos la variables de entorno mediante la libreria estandar
fn main() {
    // hacemos que en las variables de entorno se ponga el balor de slint_backend en software, para que use una CPU
    // unsafe {
    //     std::env::set_var("SLINT_BACKEND", "software");
    // }

    let app = App::new().unwrap();
    let current_path = std::env::current_dir().unwrap().display().to_string();
    // .unwrap() extrae el resultado exitoso de la compilación o detiene el programa si hubo un error.

    app.global::<Context>()
        .set_path(slint::SharedString::from(current_path));
    // guardamos en el contexto global que creamos la path de donde se abre el ejecutable

    app.on_saveCommand({
        let app_weak = app.as_weak();
        move |command| {
            if let Some(app_ui_global) = app_weak.upgrade() {
                // Obtenemos el path actual y los comandos actuales desde el Contexto global
                let path_actual = app_ui_global.global::<Context>().get_path();

                // Obtenemos los comandos actuales del Context
                let modelo_actual = app_ui_global.global::<Context>().get_historycommands();

                // Coonvertimos el modelo de Slint a un Vec de Rust de forma correcta usando .row_data()
                let mut lista_comandos: Vec<CommandStructure> = modelo_actual.iter().collect();

                // creamos un nuevo item
                let nuevo_item = CommandStructure {
                    id: (lista_comandos.len() + 1) as i32,
                    path: path_actual,
                    command: command.clone(),
                    result: slint::SharedString::from(""),
                };

                // agregamos el nuevo item
                lista_comandos.push(nuevo_item);

                // Lo volvemos a empaquetar para Slint
                let nuevo_modelo = std::rc::Rc::new(slint::VecModel::from(lista_comandos));
                // Lo devolvemos al Contexto Global
                app_ui_global
                    .global::<Context>()
                    .set_historycommands(slint::ModelRc::from(nuevo_modelo));
            }
        }
    });

    app.run().unwrap();

    // ejecutamos la aplicacion
}
