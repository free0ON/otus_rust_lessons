// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=8bdf716e45a5d842e2439e945a09d8d0
// Определяем типаж стратегии сортировки
trait SortStrategy {
    fn sort(&self, data: &mut [i32]);
}

// Реализация пузырьковой сортировки
struct BubbleSort;

impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut [i32]) {
        let len = data.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }
}

// Реализация быстрой сортировки
struct QuickSort;

impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut [i32]) {
        if data.len() <= 1 {
            return;
        }

        let pivot = data.len() / 2;
        let mut i = 0;
        let mut j = data.len() - 1;

        loop {
            while data[i] < data[pivot] {
                i += 1;
            }
            while data[j] > data[pivot] {
                j -= 1;
            }

            if i >= j {
                break;
            }

            data.swap(i, j);
            i += 1;
            j -= 1;
        }

        self.sort(&mut data[..i]);
        self.sort(&mut data[i..]);
    }
}

// Функция для применения стратегии сортировки
fn sort_data(strategy: &dyn SortStrategy, data: &mut [i32]) {
    strategy.sort(data);
}

fn main() {
    // Тестируем пузырьковую сортировку
    let mut data1 = [3, 1, 2, 5, 4];
    let bubble = Box::new(BubbleSort);
    sort_data(&*bubble, &mut data1);
    println!("Bubble sort: {:?}", data1); // [1, 2, 3, 4, 5]

    // Тестируем быструю сортировку
    let mut data2 = [7, 3, 9, 2, 1];
    let quick = Box::new(QuickSort);
    sort_data(&*quick, &mut data2);
    println!("Quick sort: {:?}", data2); // [1, 2, 3, 7, 9]

    // Динамический выбор стратегии
    let strategies: Vec<Box<dyn SortStrategy>> = vec![Box::new(BubbleSort), Box::new(QuickSort)];

    for strategy in strategies {
        let mut test_data = [5, 2, 4, 1, 3];
        strategy.sort(&mut test_data);
        println!("Sorted: {:?}", test_data);
    }
}
