mod game;

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    loop {
        game::clear_screen();
        // print_snake_ascii_art();
        let selections = &["Gioca", "Esci"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("HEYMAN")
            .default(0)
            .items(&selections[..])
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            match selection {
                0 => game::play(),
                1 => break,
                _ => unreachable!(),
            }
        }
    }
}
