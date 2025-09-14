mod window;
mod graphics;
mod state;
mod education;

use window::WindowManager;
use graphics::GraphicsEngine;
use state::AppState;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let window_manager = WindowManager::new("Learn Liberty - Educational RPG");
    let mut graphics_engine = GraphicsEngine::new(&window_manager).await?;
    let mut app_state = AppState::default();
    
    let mut last_time = Instant::now();
    
    window_manager.run(move |_window| {
        let current_time = Instant::now();
        let delta_time = last_time.elapsed().as_secs_f64();
        last_time = current_time;
        
        app_state.update(delta_time);
        
        if let Err(e) = graphics_engine.render() {
            eprintln!("Render error: {}", e);
        }
    });
    
    Ok(())
}
