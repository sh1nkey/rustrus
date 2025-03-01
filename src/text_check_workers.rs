mod files_map;
mod file_servers;
use std::{sync::{mpsc, Arc}, thread};

use file_servers::check_text;
use files_map::FILES;

// pub fn text_check_worker(text: &str, chosen_categories: &Vec<&str>) {
//     let words: &Vec<&str> = &text.split_whitespace().collect();
//     for category in chosen_categories {
//         if let Some(file_path) = FILES.get(category) {
//             if check_text(&words, file_path.to_string()) {
//                 println!("is {}", category)
//             }
//         }
//     }
// }

pub fn text_check_worker_mltr(text: &str, chosen_categories: Vec<String>) -> Option<String> {
    let words: Vec<String> = text.split_whitespace().map(String::from).collect(); // Сбор слов в Vec<String>
    let words_arc = Arc::new(words); // Оборачиваем в Arc
    
    let (tx, rx) = mpsc::channel(); // Создаем канал
    let mut handles = vec![];

    for category in chosen_categories {
        let words_clone = Arc::clone(&words_arc); // Клонируем Arc для передачи в поток
        let tx_clone = tx.clone(); // Создаем клон отправителя для каждого потока

        if let Some(file_path) = FILES.get(category.as_str()) {
            let handle = thread::spawn(move || {
                // Проверяем текст в потоке
                if check_text(&words_clone.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), file_path.to_string().clone()) {
                    // Если совпадение найдено, отправляем результат в канал
                    tx_clone.send(category).unwrap();
                }
            });
            handles.push(handle);
        }
    }

    drop(tx); // Закрываем отправителя, чтобы избежать блокировок

    // Обрабатываем результаты по мере их поступления
    for received in rx {
        return Some(received); // Возвращаем первое полученное значение
    }

    // Дожидаемся завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    None // Возвращаем None, если не найдено совпадений
}
