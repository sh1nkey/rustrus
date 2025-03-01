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
    
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for category in chosen_categories {
        let words_clone = Arc::clone(&words_arc);
        let tx_clone = tx.clone();

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

    drop(tx);

    // Обрабатываем результаты по мере их поступления
    for received in rx {
        return Some(received); // Возвращаем первое полученное значение
    }

    // Дожидаемся завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    None 
}





#[cfg(test)]
mod tests {
    use std::fs;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_strong() {
        let chosen_categories: Vec<String> = vec!["sexual".to_string(), "strong".to_string()];
        let result = text_check_worker_mltr(
            "блять", 
            chosen_categories
        );
        assert_eq!(result.unwrap(), "strong");
    }

    #[test]
    fn test_sexual() {
        let chosen_categories: Vec<String> = vec!["sexual".to_string(), "strong".to_string()];
        let result = text_check_worker_mltr(
            "пизда", 
            chosen_categories
        );
        assert_eq!(result.unwrap(), "sexual");
    }


    #[test]
    fn test_nothing() {
        let chosen_categories: Vec<String> = vec!["sexual".to_string(), "strong".to_string()];
        let result = text_check_worker_mltr(
            "ня ня ня :3", 
            chosen_categories
        );
        assert_eq!(result, None);
    }


}