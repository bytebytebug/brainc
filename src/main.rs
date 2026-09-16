use brainc::engine::EngineInitError;
use brainc::engine::PartError;
use brainc::engine::Engine;

fn main() {
    
    match Engine::create() {
        Ok(engine) => {
            println!("Project root: {}", engine.project_root());
            println!("Parts: {}", engine.project_parts().join(","));
            engine.compile_parts();
        },
        Err(error) => {
            match error {
                EngineInitError::RootNotFound => {
                    println!("root not found")
                }
                EngineInitError::PartAtRoot => {
                    println!("part colision with root")
                }
                EngineInitError::LoadInvalidPart(part_errors) => {
                    for part_error in part_errors {
                        match part_error {
                            PartError::BadFormat(path) => {
                                println!("bad format error: {}", path.to_string_lossy()) 
                            }
                        }
                    }
                }
            }
        }
    }

}
