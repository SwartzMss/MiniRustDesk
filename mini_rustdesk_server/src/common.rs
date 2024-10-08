use clap::App;

fn arg_name(name: &str) -> String {
    name.to_uppercase().replace('_', "-")
}

pub fn get_arg(name: &str) -> String {
    get_arg_or(name, "".to_owned())
}

pub fn get_arg_or(name: &str, default: String) -> String {
    std::env::var(arg_name(name)).unwrap_or(default)
}

pub fn init_args(args: &str, name: &str, about: &str) {
    let matches = App::new(name)
        .about(about)
        .args_from_usage(args)
        .get_matches();
    for (k, v) in matches.args {
        if let Some(v) = v.vals.first() {
            std::env::set_var(arg_name(k), v.to_string_lossy().to_string());
        }
    }
}