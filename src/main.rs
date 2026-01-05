use persona_forge::config::Config;
use persona_forge::state::AppState;
use persona_forge::bot::handlers::callbacks::handle_callback_query;
use persona_forge::webapp::start_webapp_server;
use sqlx::sqlite::SqlitePoolOptions;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    
    log::info!("╔════════════════════════════════════════╗");
    log::info!("║       🤖 PersonaForge Starting...      ║");
    log::info!("╚════════════════════════════════════════╝");

    let config = match Config::from_env() {
        Ok(cfg) => {
            log::info!("✅ Config loaded");
            log::info!("   ├─ Bot: {}", cfg.bot_name);
            log::info!("   ├─ Owner: {}", cfg.owner_id);
            log::info!("   ├─ LLM: {}", cfg.ollama_chat_model);
            log::info!("   ├─ Vision: {}", if cfg.vision_enabled { "✓" } else { "✗" });
            log::info!("   ├─ Voice: {}", if cfg.voice_enabled { "✓" } else { "✗" });
            log::info!("   └─ Web Search: {}", if cfg.web_search_enabled { "✓" } else { "✗" });
            cfg
        }
        Err(e) => {
            log::error!("❌ Failed to load config: {}", e);
            return;
        }
    };

    let db_pool = match SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            log::info!("✅ Database connected: {}", config.database_url);
            pool
        }
        Err(e) => {
            log::error!("❌ Database connection failed: {}", e);
            return;
        }
    };

    if let Err(e) = sqlx::migrate!("./migrations").run(&db_pool).await {
        log::error!("❌ Migrations failed: {}", e);
        return;
    }
    log::info!("✅ Migrations applied");

    let webapp_port = config.webapp_port;
    let bot = Bot::new(config.teloxide_token.clone());
    let app_state = AppState::new(config, db_pool);

    // Start webapp server in background
    let webapp_state = app_state.clone();
    tokio::spawn(async move {
        start_webapp_server(webapp_state, webapp_port).await;
    });
    log::info!("✅ WebApp started on port {}", webapp_port);

    log::info!("╔════════════════════════════════════════╗");
    log::info!("║         🚀 Bot is now running!         ║");
    log::info!("╚════════════════════════════════════════╝");

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(persona_forge::bot::handlers::messages::handle_message))
        .branch(Update::filter_callback_query().endpoint(handle_callback_query));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![app_state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    log::info!("👋 Bot has shut down.");
}