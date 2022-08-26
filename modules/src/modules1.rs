// modules1.rs

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        let secret_recipe = get_secret_recipe();
        println!("{}!", secret_recipe);
        println!("sausage!");
    }
}

pub fn run_modules1() {
    sausage_factory::make_sausage();
}
