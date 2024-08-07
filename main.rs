use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_file(filename: &str) -> Result<Vec<i32>, io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().trim().parse::<i32>().unwrap())
        .collect();

    Ok(numbers)
}

fn sort(nums: &mut Vec<i32>) {
    // Usando o algoritmo de ordenação por inserção como exemplo
    for i in 1..nums.len() {
        let key = nums[i];
        let mut j = i - 1;
        while j >= 0 && nums[j] > key {
            nums[j + 1] = nums[j];
            j -= 1;
        }
        nums[j + 1] = key;
    }
}

fn main() {
    let filename = "numbers.txt"; // Substitua pelo nome do seu arquivo
    let mut numbers = read_file(filename).expect("Erro ao ler o arquivo");
    println!("Números antes de ordenar: {:?}", numbers);

    sort(&mut numbers);
    println!("Números após ordenar: {:?}", numbers);
}
