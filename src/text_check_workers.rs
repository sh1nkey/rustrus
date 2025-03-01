extern crate test;

mod files_map;
mod file_servers;
use std::{sync::{mpsc, Arc}, thread};

use file_servers::{check_text, check_text_new};
use files_map::FILES;


pub fn text_check_worker(text: &str, chosen_categories: &Vec<&str>) -> Option<String>{
    let words: &Vec<&str> = &text.split_whitespace().collect();
    for category in chosen_categories {
        if let Some(file_path) = FILES.get(category) {
            if check_text(&words, file_path) {
                return Some(category.to_string())
            }
        }
    }
    None
}

type TextChecker = fn(&Vec<&str>, &str) -> bool;
pub fn text_check_worker_mltr(check_text: TextChecker, text: &str, chosen_categories: Vec<String>) -> Option<String> {
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
                if check_text(&words_clone.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), file_path.clone()) {
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

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_strong() {
        let chosen_categories: Vec<String> = vec!["strong".to_string()];
        let result = text_check_worker_mltr(
            check_text,
            "блять", 
            chosen_categories
        );
        assert_eq!(result.unwrap(), "strong");
    }

    #[test]
    fn test_sexual() {
        let chosen_categories: Vec<String> = vec!["sexual".to_string()];
        let result = text_check_worker_mltr(
            check_text,
            "пизда", 
            chosen_categories
        );
        assert_eq!(result.unwrap(), "sexual");
    }


    #[test]
    fn test_nothing() {
        let chosen_categories: Vec<String> = vec!["sexual".to_string()];
        let result = text_check_worker_mltr(
            check_text,
            "ня ня ня :3", 
            chosen_categories
        );
        assert_eq!(result, None);
    }

}


#[cfg(test)]
mod benches {
    use super::*;
    use std::fs;
    use test::Bencher;

    #[bench]
    fn test_second_version(b: &mut Bencher) {
        let text = fs::read_to_string(r"C:\Users\User\Desktop\bad_words_filter\bad_words_filter\filter_data\test_text.txt")
            .expect("Unable to read file");

        let chosen_categories: Vec<String> = vec!["sexual".to_string(), "strong".to_string()];

        b.iter(|| {
            text_check_worker_mltr(
                check_text_new,
                &text,
                 chosen_categories.clone()
            )
        });
    }



    #[bench]
    fn test_first_version(b: &mut Bencher) {
        let text = fs::read_to_string(r"C:\Users\User\Desktop\bad_words_filter\bad_words_filter\filter_data\test_text.txt")
            .expect("Unable to read file");

        let chosen_categories: Vec<&str> = vec!["sexual", "strong"];

        // Бенчмаркинг
        b.iter(|| {
            text_check_worker(&text, &chosen_categories)
        });
    }
}