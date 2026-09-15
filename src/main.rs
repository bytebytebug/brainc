use brainc::engine::Engine;

fn main() {
    
    let engine = Engine::create();

    println!("Project root: {}", engine.project_root());
}
