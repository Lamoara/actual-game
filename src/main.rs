use actual_game::app::{self};

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    app::run().unwrap();

    #[cfg(target_arch = "wasm32")]
    app::run_web().unwrap();
}
