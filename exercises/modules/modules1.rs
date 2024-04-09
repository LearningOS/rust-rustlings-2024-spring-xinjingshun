// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        self::get_secret_recipe(); //在本题中，self可有可无。
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
