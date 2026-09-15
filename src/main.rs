use brainc::engine::Engine;

fn main() {
    
    match Engine::create() {
        Ok(engine) => {
            println!("Project root: {}", engine.project_root());
        },
        Err(_) => {
            println!("Can't find root.brain")
        }
    }

}
