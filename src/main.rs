// Main event loop stub — will be fully rewritten with SDL2 in Step 6
// For now this is a placeholder to keep the project compiling

fn main() {
    let config = term_tiler::config::load_config();
    println!("term-tiler v0.2.0 — SDL2 backend");
    println!("Font: {} @ {}pt", config.render.font_family, config.render.font_size);
    println!("Window: {}x{}", config.render.window_width, config.render.window_height);
    println!("Scrollback: {} lines", config.render.scrollback_lines);
}
