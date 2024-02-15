extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};

fn main(){
    let filename = "params.txt";
    let (a, b, n, hi_sqr_theory) = read_params(filename).expect("Файл не найден!");
    println!("Параметры успешно считаны из файла {}", filename);

    let k = (1.0 + 3.322 * f64::log10(n as f64)).ceil() as usize; 
    let R = get_random(a, b, n as i32);
    let R_clone = R.clone();
    

    let mean_X = mean(R);
    let mean_theory = (a + b) / 2.0;
    let dispersion_X = dispersion(&R_clone);
    let dispersion_theory = (b - a) * (b - a) / 12.0;
    let hi_sqr = hi_squared(&R_clone, a, b,  k);

    // вывод в текстовый файл
    let mut results = String::new();
    results.push_str(&format!("Входные параметры: a = {}, b = {}, N = {}\n", a, b, n));
    results.push_str(&format!("Количество интервалов: {}\n", k));
    results.push_str(&format!("Сгенерированнная выборка: {:?}\n", R_clone));
    results.push_str(&format!("Математическое ожидание (теоретическое): {}\n", mean_theory));
    results.push_str(&format!("Математическое ожидание (расчетное): {}\n", mean_X));
    results.push_str(&format!("Дисперсия (теоретическая): {}\n", dispersion_theory));
    results.push_str(&format!("Дисперсия (рачетная): {}\n", dispersion_X));
    results.push_str(&format!("Уровень значимости: {}\n", 0.05));
    results.push_str(&format!("Критерий Пирсона (табличное): {}\n", hi_sqr_theory[k - 1]));
    results.push_str(&format!("Критерий Пирсона (расчетнное): {}\n", hi_sqr));
    write_to_file("results.txt", results).expect("Не удалось записать в файл!");
    println!("Результаты записаны в файл results.txt");

}

fn get_random (
    a: f64, // нижняя граница генеральной выборки
    b: f64, // верхняя граница генеральной выборки
    n: i32, // число элементов в генеральной выборке
)-> Vec<f64>{
    // Инициализация генератора случайных чисел
    let mut rng = rand::thread_rng();

    // Определение диапазона значений
    let range = Uniform::new(a, b);

    // Генерация n случайных чисел из равномерного распределения
    let R: Vec<f64> = (0..n).map(|_| rng.sample(&range)).collect();

    R
}

// математическое ожидание
fn mean(R: Vec<f64>) -> f64 {
    R.iter().sum::<f64>() / R.len() as f64
}

// дисперсия
fn dispersion(R: &Vec<f64>) -> f64 {
    let m = mean(R.to_vec());
    let mut sum_of_squares: f64 = 0.0;
    for x in R {
        let diff = *x as f64 - m;
        sum_of_squares += diff * diff;
    }
    sum_of_squares / R.len() as f64
}

// расхождение практического и теоретического (критерий Пиросона)
fn hi_squared(R: &Vec<f64>, a: f64, b: f64,  k: usize) -> f64 {
    let mut sum = 0.0;
    
    for i in 0..k {
        let mut n_counts = 0;
        for x in R {
            if *x >= a + ((i)  as f64) * (b - a) / (k as f64) && *x < a + ((i + 1) as f64) * (b - a) / (k as f64) {
                n_counts += 1;
            }
        }
        let n_normal = R.len() as f64;
        let pi = 1.0 / k as f64;
        let dif = n_counts as f64 - n_normal * pi;
        sum += dif * dif / (n_normal * pi);
    }
 
    sum

}

// СЛУЖЕБНЫЕ ФУНКЦИИ
fn read_params(filename: &str) -> Result<(f64, f64, usize, Vec<f64>), io::Error>{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let params = lines.next().expect("Первая строка отсутствует!")?;
    let hi_sqr_theory_line = lines.next().expect("Вторая строка отсутствует!")?;

    
    println!("Содержимое файла:");
    println!("{}", params);
    println!("{}", hi_sqr_theory_line);



    
    let a: f64 = params.split_whitespace().next().unwrap().parse().expect("Некорректное значение для a!");
    let b: f64 = params.split_whitespace().nth(1).unwrap().parse().expect("Некорректное значение для b!");
    let n: usize = params.split_whitespace().nth(2).unwrap().parse().expect("Некорректное значение для N!");

    //  табличные значения для критерия Пирсона
    let hi_sqr_theory: Vec<f64> = hi_sqr_theory_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    Ok((a, b, n, hi_sqr_theory))
}

fn write_to_file(filename: &str, results: String)
-> Result<(), io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(results.as_bytes())?;
    Ok(())
}