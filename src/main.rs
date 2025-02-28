use std::fs;
use phf::phf_map;

//use rustrict::CensorStr;


static FILES: phf::Map<&'static str, &'static str> = phf_map! {
    "sexual" => r"C:\Users\User\Desktop\bad_words_filter\bad_words_filter\sexual.txt",
    "strong" => r"C:\Users\User\Desktop\bad_words_filter\bad_words_filter\stongswords.txt",
    //"political" => r"C:\Users\User\Desktop\bad_words_filter\bad_words_filter\political.txt",
};

fn main() {
    let text = "пиздаблять";
    let chosen_categories: Vec<String> = vec!["sexual".to_string(), "strong".to_string()];
    let res = text_check_worker_mltr(text, chosen_categories);
    if res != None {
        println!("is {}", res.unwrap());
    }
}

use std::sync::{mpsc, Arc};
use std::thread;

fn text_check_worker(text: &str, chosen_categories: &Vec<&str>) {
    let words: &Vec<&str> = &text.split_whitespace().collect();
    for category in chosen_categories {
        if let Some(file_path) = FILES.get(category) {
            if check_text(&words, file_path.to_string()) {
                println!("is {}", category)
            }
        }
    }
}

fn text_check_worker_mltr(text: &str, chosen_categories: Vec<String>) -> Option<String> {
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






fn check_text(words: &Vec<&str>, file_name: String) -> bool {
    for word in words {
        if check_in_file(word.to_string(), &file_name) {
            return true
        }
    }
    return false
}

fn check_in_file(word: String, file_name: &String) -> bool {
    let file_contents = fs::read_to_string(file_name).expect("Unable to read file");

    for line in file_contents.lines() {
        if word.contains(line) {
            return true
        }
    }
    return false
}