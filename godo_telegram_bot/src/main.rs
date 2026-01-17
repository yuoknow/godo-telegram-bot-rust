use teloxide::macros::BotCommands;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};
use teloxide::RequestError;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
enum Command {
    #[command(description = "Показать информацию о проекте")]
    Info,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Запуск бота...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        if text == "/info" {
            show_menu(bot, msg.chat.id).await?;
        }
    }
    Ok(())
}

async fn show_menu(bot: Bot, chat_id: ChatId) -> Result<(), RequestError> {
    let keyboard = make_info_keyboard();
    bot.send_message(chat_id, "Информация о проекте")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

fn make_info_keyboard() -> InlineKeyboardMarkup {
    let mut buttons = Vec::new();

    buttons.push(vec![InlineKeyboardButton::callback(
        "Как работать в github",
        "github_info",
    )]);

    buttons.push(vec![InlineKeyboardButton::callback(
        "Репозиторий",
        "repository",
    )]);

    buttons.push(vec![InlineKeyboardButton::callback(
        "Доска YouGile",
        "board",
    )]);

    buttons.push(vec![InlineKeyboardButton::callback(
        "Информация о проекте",
        "project_info",
    )]);

    InlineKeyboardMarkup::new(buttons)
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    log::info!("Callback query: {:?}", q);
    let data = match q.data {
        Some(d) => d,
        None => return Ok(()),
    };

    let msg = match q.message {
        Some(m) => m,
        None => {
            bot.answer_callback_query(q.id).await?;
            return Ok(());
        }
    };

    if (data == "back_to_menu") {
        bot.answer_callback_query(q.id).await?;
        bot.edit_message_text(
            msg.chat().id,
            msg.id(),
            "📂 *Главное меню проекта*\nВыберите нужный раздел:",
        )
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(make_info_keyboard())
        .await?;
        return Ok(());
    }

    let response_text = match data.as_str() {
        "github_info" => {
            "1\\. 🌿 Создаешь ветку у себя и вносишь изменения 🛠\n\
             2\\. 🚀 Делаешь пулл реквест \\(PR\\) в мой репозиторий 📥\n\
             3\\. ✅ Я принимаю реквест и вливаю изменения \\(Merge\\) 🤝\n\
             4\\. 🔄 Заходишь на GitHub в свой репозиторий и нажимаешь кнопку Sync fork, чтобы подтянуть изменения к себе 🏁"
        }
        "repository" => {
            "🔗 *Ссылка на репозиторий:*\n[https://github\\.com/yuoknow/godo\\-app](https://github.com/yuoknow/godo-app)"
        }
        "board" => {
            "📋 *Доска YouGile:*\nВсе текущие задачи и спринты находятся здесь: [https://ru\\.yougile\\.com/team/74ea09ab0004/GoDo\\-App](https://ru.yougile.com/team/74ea09ab0004/GoDo-App)"
        }
        "project_info" => {
            "ℹ️ *О проекте:*\nПоиск и выбор мастер\\-классов\\. Мы хотим, чтобы пользователь:\n\n\
            • Искал мастер\\-классы в каталоге по категориям, ключевым словам, видел последние добавленные или самые популярные\\. С помощью исследований тебе нужно определить, какие нужны категории и как должен работать поиск\\.\n\
            • Просматривал информацию о каждом мастер\\-классе, включая описание, информацию об инструкторе, длительность\\.\n\
            • Видил разбивку хобби на категории — и чтобы они визуализировали разные направления деятельности, такие как музыка, кино, танцы, фотография и так далее\\.\n\n\
            *Регистрация на мастер\\-класс:* Важно, чтобы пользователь мог найти и забронировать место на конкретный мастер\\-класс, выбрать дату и время из предложенных вариантов\\. Один и тот же мастер\\-класс может проводиться в разные даты, поэтому стоит уточнить, какой нужен слот\\. К этому моменту пользователь уже будет зарегистрирован в приложении, поэтому не нужно заново вводить данные пользователя\\.\n\n\
            *Участие в мастер\\-классе:* У пользователя должна быть возможность поучаствовать в онлайн\\-мастер\\-классе через видеоконференцию в Zoom\\. В карточке мероприятия нужна точка входа в конференцию\\. Доступ должен быть простой — приходит уведомление о том, что скоро начнётся мероприятие\\. Мы хотим протестировать групповые онлайн мастер\\-классы, чтобы пользователи могли задать вопросы эксперту, пообщаться друг с другом по ходу мастер\\-класса\\. Это — альтернатива предзаписанным видео, где человек остаётся один на один с картинкой\\."
        }
        _ => "Неизвестная команда",
    };

    bot.edit_message_text(msg.chat().id, msg.id(), response_text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(make_back_keyboard())
        .await?;

    bot.answer_callback_query(q.id).await?;
    Ok(())
}

fn make_back_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "⬅️ Назад в меню",
        "back_to_menu",
    )]])
}
