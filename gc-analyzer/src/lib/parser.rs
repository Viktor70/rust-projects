use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]
pub fn read_file(file_path: &str) -> io::Result<u32> {
    // Открываем файл
    let file = File::open(file_path)?;

    // Создаём BufReader для эффективного чтения
    let reader = BufReader::new(file);
    let mut count: u32 = 0;
    // Читаем файл построчно
    for line in reader.lines() {
        let _line = line?; // Обрабатываем возможную ошибку
        count += 1;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let result = read_file("./logs/gc-2025-03-31_07-54-24.log").expect("какая-то херня");
        println!("Прочитатли строк: {result}");
        assert!(result == 4085322);
    }
}
