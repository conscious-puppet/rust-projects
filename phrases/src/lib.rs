pub mod english;
pub mod japanese {
    pub mod greetings {
        pub fn hello() -> String {
            "こんにちは".to_string()
        }
    }
    pub mod farewells {
        pub fn goodbye() -> String {
            "さようなら".to_string()
        }
    }
}
