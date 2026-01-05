use crate::state::AppState;
use teloxide::prelude::*;
use teloxide::types::{CallbackQueryId, ParseMode};

pub async fn handle_callback_query(bot: Bot, q: CallbackQuery, state: AppState) -> ResponseResult<()> {
    if let Some(message) = &q.message {
        let chat_id = message.chat_id();
        
        // Check if the user is the owner
        if q.from.id.0 != state.config.owner_id {
            bot.answer_callback_query(q.id.clone())
                .text("❌ У вас нет прав для выполнения этой команды.")
                .await?;
            return Ok(());
        }

        match q.data.as_deref() {
            Some("personas_menu") => show_personas_menu(bot, &q.id, chat_id).await?,
            Some("model_settings") => show_model_settings_menu(bot, &q.id, chat_id).await?,
            Some("rag_settings") => show_rag_settings_menu(bot, &q.id, chat_id).await?,
            Some("chat_settings") => show_chat_settings_menu(bot, &q.id, chat_id).await?,
            Some("change_persona") => show_change_persona_menu(bot, &q.id, chat_id).await?,
            Some("create_persona_wizard") => start_create_persona_wizard(bot, &q.id, chat_id, &state).await?,
            Some("activate_persona_wizard") => show_activate_persona_wizard(bot, &q.id, chat_id, &state).await?,
            Some("update_persona_wizard") => start_update_persona_wizard(bot, &q.id, chat_id, &state).await?,
            Some("delete_persona_wizard") => show_delete_persona_wizard(bot, &q.id, chat_id, &state).await?,
            Some("memory_settings") => show_memory_settings_menu(bot, &q.id, chat_id).await?,
            Some("model_params") => show_model_params_menu(bot, &q.id, chat_id).await?,
            Some("settings_menu") => send_settings_menu(bot, &q.id, chat_id).await?,
            Some("main_menu") => send_main_menu(bot, &q.id, chat_id).await?,
            Some("system_status") => {
                // Reuse the existing status command
                if let Some(msg) = q.message.clone() {
                    if let Ok(message) = msg.clone().into_message() {
                        super::commands::handle_status(bot, message, &state).await?;
                    }
                }
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("help_info") => {
                // Reuse the existing help command
                super::commands::send_help_message(bot, chat_id).await?;
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("list_personas") => {
                // Reuse the existing list command
                if let Some(msg) = q.message.clone() {
                    if let Ok(message) = msg.clone().into_message() {
                        super::commands::handle_list_personas(bot, message, &state).await?;
                    }
                }
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("set_model") => {
                bot.send_message(chat_id, "🏷️ <b>Смена модели</b>\n\nИспользуйте команду: <code>/set_model название</code>")
                    .parse_mode(ParseMode::Html)
                    .await?;
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("set_temperature") => {
                bot.send_message(chat_id, "🌡️ <b>Установка температуры</b>\n\nИспользуйте команду: <code>/set_temperature значение</code>")
                    .parse_mode(ParseMode::Html)
                    .await?;
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("set_max_tokens") => {
                bot.send_message(chat_id, "🔢 <b>Установка максимальных токенов</b>\n\nИспользуйте команду: <code>/set_max_tokens значение</code>")
                    .parse_mode(ParseMode::Html)
                    .await?;
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("enable_rag") => {
                if let Some(msg) = q.message.clone() {
                    if let Ok(message) = msg.clone().into_message() {
                        super::commands::handle_enable_rag(bot, message, &state).await?;
                    }
                }
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("disable_rag") => {
                if let Some(msg) = q.message.clone() {
                    if let Ok(message) = msg.clone().into_message() {
                        super::commands::handle_disable_rag(bot, message, &state).await?;
                    }
                }
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("set_memory_depth") => {
                bot.send_message(chat_id, "🧠 <b>Глубина памяти</b>\n\nИспользуйте команду: <code>/set_memory_depth значение</code>")
                    .parse_mode(ParseMode::Html)
                    .await?;
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("enable_auto_reply") => {
                if let Some(msg) = q.message.clone() {
                    if let Ok(message) = msg.clone().into_message() {
                        super::commands::handle_enable_auto_reply(bot, message, &state).await?;
                    }
                }
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("disable_auto_reply") => {
                if let Some(msg) = q.message.clone() {
                    if let Ok(message) = msg.clone().into_message() {
                        super::commands::handle_disable_auto_reply(bot, message, &state).await?;
                    }
                }
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("reply_to_all") => {
                if let Some(msg) = q.message.clone() {
                    if let Ok(message) = msg.clone().into_message() {
                        super::commands::handle_reply_to_all(bot, message, &state).await?;
                    }
                }
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("reply_to_mention") => {
                if let Some(msg) = q.message.clone() {
                    if let Ok(message) = msg.clone().into_message() {
                        super::commands::handle_reply_to_mention(bot, message, &state).await?;
                    }
                }
                bot.answer_callback_query(q.id.clone()).await?;
            }
            Some("set_cooldown") => {
                bot.send_message(chat_id, "⏱️ <b>Задержка между ответами</b>\n\nИспользуйте команду: <code>/set_cooldown значение</code>")
                    .parse_mode(ParseMode::Html)
                    .await?;
                bot.answer_callback_query(q.id.clone()).await?;
            }
            _ => {
                bot.answer_callback_query(q.id.clone())
                    .text("❌ Неизвестная команда меню.")
                    .await?;
            }
        }
    } else {
        // If we can't get the message from the callback query, try to get the chat ID differently
        bot.answer_callback_query(q.id.clone())
            .text("❌ Не удалось получить информацию о чате.")
            .await?;
    }

    Ok(())
}

async fn show_personas_menu(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId) -> ResponseResult<()> {
    use teloxide::types::InlineKeyboardButton;
    use teloxide::types::InlineKeyboardMarkup;

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("📋 Список персон", "list_personas"),
        ],
        vec![
            InlineKeyboardButton::callback("🆕 Создать персону", "create_persona_wizard"),
        ],
        vec![
            InlineKeyboardButton::callback("✏️ Изменить персону", "update_persona_wizard"),
        ],
        vec![
            InlineKeyboardButton::callback("🗑️ Удалить персону", "delete_persona_wizard"),
        ],
        vec![
            InlineKeyboardButton::callback("✅ Активировать персону", "activate_persona_wizard"),
        ],
        vec![
            InlineKeyboardButton::callback("🔙 Назад", "main_menu"),
        ],
    ]);

    bot.send_message(chat_id, "👤 <b>Управление персонами</b>\n\nВыберите действие:")
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    bot.answer_callback_query(callback_id.clone()).await?;

    Ok(())
}

async fn show_model_settings_menu(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId) -> ResponseResult<()> {
    use teloxide::types::InlineKeyboardButton;
    use teloxide::types::InlineKeyboardMarkup;

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("🏷️ Сменить модель", "set_model"),
        ],
        vec![
            InlineKeyboardButton::callback("🌡️ Температура", "set_temperature"),
        ],
        vec![
            InlineKeyboardButton::callback("🔢 Макс. токены", "set_max_tokens"),
        ],
        vec![
            InlineKeyboardButton::callback("🔙 Назад", "main_menu"),
        ],
    ]);

    bot.send_message(chat_id, "⚙️ <b>Настройки модели</b>\n\nВыберите параметр для настройки:")
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    bot.answer_callback_query(callback_id.clone()).await?;

    Ok(())
}

async fn show_rag_settings_menu(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId) -> ResponseResult<()> {
    use teloxide::types::InlineKeyboardButton;
    use teloxide::types::InlineKeyboardMarkup;

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("🟢 Включить RAG", "enable_rag"),
        ],
        vec![
            InlineKeyboardButton::callback("🔴 Отключить RAG", "disable_rag"),
        ],
        vec![
            InlineKeyboardButton::callback("🧠 Глубина памяти", "set_memory_depth"),
        ],
        vec![
            InlineKeyboardButton::callback("🔙 Назад", "main_menu"),
        ],
    ]);

    bot.send_message(chat_id, "🧠 <b>Настройки RAG</b>\n\nВыберите действие:")
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    bot.answer_callback_query(callback_id.clone()).await?;

    Ok(())
}

async fn show_chat_settings_menu(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId) -> ResponseResult<()> {
    use teloxide::types::InlineKeyboardButton;
    use teloxide::types::InlineKeyboardMarkup;

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("🟢 Включить автоответы", "enable_auto_reply"),
        ],
        vec![
            InlineKeyboardButton::callback("🔴 Отключить автоответы", "disable_auto_reply"),
        ],
        vec![
            InlineKeyboardButton::callback("💬 Отвечать всем", "reply_to_all"),
        ],
        vec![
            InlineKeyboardButton::callback("👤 Только по упоминанию", "reply_to_mention"),
        ],
        vec![
            InlineKeyboardButton::callback("⏱️ Задержка между ответами", "set_cooldown"),
        ],
        vec![
            InlineKeyboardButton::callback("🔙 Назад", "main_menu"),
        ],
    ]);

    bot.send_message(chat_id, "💬 <b>Настройки чата</b>\n\nВыберите параметр для настройки:")
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    bot.answer_callback_query(callback_id.clone()).await?;

    Ok(())
}

async fn show_change_persona_menu(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId) -> ResponseResult<()> {
    use teloxide::types::InlineKeyboardButton;
    use teloxide::types::InlineKeyboardMarkup;

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("🎭 Сменить персону", "change_persona"),
        ],
        vec![
            InlineKeyboardButton::callback("🧠 Настройки памяти", "memory_settings"),
        ],
        vec![
            InlineKeyboardButton::callback("⚙️ Параметры модели", "model_params"),
        ],
        vec![
            InlineKeyboardButton::callback("🔙 Назад", "settings_menu"),
        ],
    ]);

    bot.send_message(chat_id, "🔧 <b>Настройки бота</b>\n\nВыберите параметр для настройки:")
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    bot.answer_callback_query(callback_id.clone()).await?;

    Ok(())
}

async fn start_create_persona_wizard(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId, state: &AppState) -> ResponseResult<()> {
    // Set the wizard state to CreatingPersonaName
    {
        let mut wizard_states = state.wizard_states.lock().await;
        wizard_states.insert(chat_id, crate::state::WizardState::CreatingPersonaName);
    }

    bot.send_message(chat_id, "👤 <b>Создание новой персоны (пошагово)</b>\n\nВведите название персоны:")
        .parse_mode(ParseMode::Html)
        .await?;

    bot.answer_callback_query(callback_id.clone()).await?;

    Ok(())
}

async fn show_activate_persona_wizard(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId, state: &AppState) -> ResponseResult<()> {
    // Create a dummy message to reuse the existing list command
    let dummy_msg = teloxide::types::Message {
        id: teloxide::types::MessageId(0),
        date: teloxide::types::Timestamp::now(),
        chat: teloxide::types::Chat::Private(teloxide::types::PrivateChat {
            id: teloxide::types::ChatId(chat_id.0),
            type_: teloxide::types::PrivateChatType::Regular,
            title: None,
            username: None,
            first_name: Some("Test".to_string()),
            last_name: None,
            bio: None,
            has_private_forwards: None,
            has_restricted_voice_and_video_messages: None,
            join_to_send_messages: None,
            join_by_request: None,
            active_usernames: None,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
            available_reactions: None,
            accent_color_id: 0,
            max_reaction_count: 0,
            background_custom_emoji_id: None,
            profile_accent_color_id: None,
            profile_background_custom_emoji_id: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: None,
            has_aggressive_anti_spam_enabled: None,
            chat_boosts: None,
            forum_topic_icon_color: None,
            forum_topic_icon_custom_emoji_id: None,
            is_general_forum_topic: None,
            is_forum: None,
            has_protected_content: None,
            is_member: None,
            can_send_messages: None,
            can_send_audios: None,
            can_send_documents: None,
            can_send_photos: None,
            can_send_videos: None,
            can_send_video_notes: None,
            can_send_voice_notes: None,
            can_send_polls: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
            can_manage_topics: None,
        }),
        from: Some(teloxide::types::User {
            id: teloxide::types::UserId(state.config.owner_id),
            is_bot: false,
            first_name: "Owner".to_string(),
            last_name: None,
            username: None,
            language_code: None,
            is_premium: None,
            added_to_attachment_menu: None,
        }),
        sender_chat: None,
        forward_origin: None,
        is_topic_message: false,
        is_automatic_forward: None,
        reply_to_message: None,
        external_reply: None,
        quote: None,
        reply_to_story: None,
        via_bot: None,
        edit_date: None,
        has_protected_content: None,
        media_group_id: None,
        author_signature: None,
        text: Some("Список доступных персон:".to_string()),
        entities: vec![],
        link_preview_options: None,
        effect_id: None,
        paid_media: None,
    };
    super::commands::handle_list_personas(bot, dummy_msg, state).await?;

    bot.send_message(chat_id, "Введите ID персоны, которую хотите активировать:")
        .await?;

    bot.answer_callback_query(callback_id.clone()).await?;

    Ok(())
}

async fn start_update_persona_wizard(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId, state: &AppState) -> ResponseResult<()> {
    // Set the wizard state to UpdatingPersonaId
    {
        let mut wizard_states = state.wizard_states.lock().await;
        wizard_states.insert(chat_id, crate::state::WizardState::UpdatingPersonaId);
    }

    bot.send_message(chat_id, "✏️ <b>Обновление персоны (пошагово)</b>\n\nВведите ID персоны, которую хотите обновить:")
        .parse_mode(ParseMode::Html)
        .await?;

    bot.answer_callback_query(callback_id.clone()).await?;

    Ok(())
}

async fn show_delete_persona_wizard(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId, state: &AppState) -> ResponseResult<()> {
    // Create a dummy message to reuse the existing list command
    let dummy_msg = teloxide::types::Message {
        id: teloxide::types::MessageId(0),
        date: teloxide::types::Timestamp::now(),
        chat: teloxide::types::Chat::Private(teloxide::types::PrivateChat {
            id: teloxide::types::ChatId(chat_id.0),
            type_: teloxide::types::PrivateChatType::Regular,
            title: None,
            username: None,
            first_name: Some("Test".to_string()),
            last_name: None,
            bio: None,
            has_private_forwards: None,
            has_restricted_voice_and_video_messages: None,
            join_to_send_messages: None,
            join_by_request: None,
            active_usernames: None,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
            available_reactions: None,
            accent_color_id: 0,
            max_reaction_count: 0,
            background_custom_emoji_id: None,
            profile_accent_color_id: None,
            profile_background_custom_emoji_id: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: None,
            has_aggressive_anti_spam_enabled: None,
            chat_boosts: None,
            forum_topic_icon_color: None,
            forum_topic_icon_custom_emoji_id: None,
            is_general_forum_topic: None,
            is_forum: None,
            has_protected_content: None,
            is_member: None,
            can_send_messages: None,
            can_send_audios: None,
            can_send_documents: None,
            can_send_photos: None,
            can_send_videos: None,
            can_send_video_notes: None,
            can_send_voice_notes: None,
            can_send_polls: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
            can_manage_topics: None,
        }),
        from: Some(teloxide::types::User {
            id: teloxide::types::UserId(state.config.owner_id),
            is_bot: false,
            first_name: "Owner".to_string(),
            last_name: None,
            username: None,
            language_code: None,
            is_premium: None,
            added_to_attachment_menu: None,
        }),
        sender_chat: None,
        forward_origin: None,
        is_topic_message: false,
        is_automatic_forward: None,
        reply_to_message: None,
        external_reply: None,
        quote: None,
        reply_to_story: None,
        via_bot: None,
        edit_date: None,
        has_protected_content: None,
        media_group_id: None,
        author_signature: None,
        text: Some("Список доступных персон:".to_string()),
        entities: vec![],
        link_preview_options: None,
        effect_id: None,
        paid_media: None,
    };
    super::commands::handle_list_personas(bot, dummy_msg, state).await?;

    bot.send_message(chat_id, "Введите ID персоны, которую хотите удалить:")
        .await?;

    bot.answer_callback_query(callback_id.clone()).await?;

    Ok(())
}

async fn show_memory_settings_menu(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId) -> ResponseResult<()> {
    use teloxide::types::InlineKeyboardButton;
    use teloxide::types::InlineKeyboardMarkup;

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("🧠 Глубина памяти", "set_memory_depth"),
        ],
        vec![
            InlineKeyboardButton::callback("📊 Просмотр памяти", "view_memory"),
        ],
        vec![
            InlineKeyboardButton::callback("🧹 Очистить память", "clear_memory"),
        ],
        vec![
            InlineKeyboardButton::callback("🔙 Назад", "settings_menu"),
        ],
    ]);

    bot.send_message(chat_id, "🧠 <b>Настройки памяти</b>\n\nВыберите действие:")
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    bot.answer_callback_query(callback_id.clone()).await?;

    Ok(())
}

async fn show_model_params_menu(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId) -> ResponseResult<()> {
    use teloxide::types::InlineKeyboardButton;
    use teloxide::types::InlineKeyboardMarkup;

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("🏷️ Сменить модель", "set_model"),
        ],
        vec![
            InlineKeyboardButton::callback("🌡️ Температура", "set_temperature"),
        ],
        vec![
            InlineKeyboardButton::callback("🔢 Макс. токены", "set_max_tokens"),
        ],
        vec![
            InlineKeyboardButton::callback("🔙 Назад", "settings_menu"),
        ],
    ]);

    bot.send_message(chat_id, "⚙️ <b>Параметры модели</b>\n\nВыберите параметр для настройки:")
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    bot.answer_callback_query(callback_id.clone()).await?;

    Ok(())
}

pub async fn send_settings_menu(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId) -> ResponseResult<()> {
    super::commands::send_settings_menu(bot.clone(), chat_id).await?;
    bot.answer_callback_query(callback_id.clone()).await?;
    Ok(())
}

pub async fn send_main_menu(bot: Bot, callback_id: &CallbackQueryId, chat_id: ChatId) -> ResponseResult<()> {
    super::commands::send_main_menu(bot.clone(), chat_id).await?;
    bot.answer_callback_query(callback_id.clone()).await?;
    Ok(())
}