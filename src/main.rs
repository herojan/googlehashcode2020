use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Lines, Write};
use std::ops::{Index, IndexMut};

fn main() {
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        let mut problem = make_problem(letter);
        let libs = problem.solve();
        let output_file = File::create(format!("data/{}_out.txt", letter)).unwrap();
        let mut writer = BufWriter::new(output_file);

        writeln!(writer, "{}", libs.len());
        for lib in libs {
            let book_ids: Vec<String> = lib.books.into_iter().map(|i| format!("{}", i)).collect();
            writeln!(writer, "{} {}", lib.id, book_ids.len());
            writeln!(writer, "{}", book_ids.join(" "));
        }
        writer.flush().unwrap();
    }
}

impl Problem {
    fn solve(&mut self) -> Vec<Library> {
        let mut remaining_days = self.days;
        let mut libraries_to_choose: Vec<Library> = vec![];
        let empty_vec: Vec<usize> = vec![];
        loop {
            let books_to_remove: &Vec<usize> = libraries_to_choose
                .last()
                .map(|lib| &lib.books)
                .unwrap_or(&empty_vec);
            let best_score = self.best_library_score(remaining_days, &books_to_remove);

            if best_score != LibraryScore::default() {
                let mut popped = self.libraries.remove(best_score.index);
                let available_books = popped.books.len();
                let num_books_to_ship = best_score.num_books_to_ship;
                if available_books > num_books_to_ship {
                    popped.books.drain(num_books_to_ship..available_books);
                }

                remaining_days -= popped.sign_up_days;
                println!("{}", remaining_days);
                libraries_to_choose.push(popped);
            } else {
                break;
            }
        }
        return libraries_to_choose;
    }

    fn best_library_score(
        &mut self,
        remaining_days: usize,
        books_to_remove: &Vec<usize>,
    ) -> LibraryScore {
        let mut best_score = LibraryScore::default();
        for i in 0..self.libraries.len() {
            let library: &mut Library = self.libraries.index_mut(i);
            library.remove_books(books_to_remove);
            let library_score = library.calculate_score(remaining_days, &self.books, i);
            if library_score.score > best_score.score {
                best_score = library_score;
            }
        }

        return best_score;
    }
}

impl Library {
    fn remove_books(&mut self, books_to_remove: &Vec<usize>) {
        self.books.retain(|book| !books_to_remove.contains(book))
    }

    fn calculate_score(
        &self,
        remaining_days: usize,
        problem_books: &Vec<usize>,
        index: usize,
    ) -> LibraryScore {
        if remaining_days < self.sign_up_days {
            return LibraryScore::default();
        }

        let num_books_to_ship = (remaining_days - self.sign_up_days) * self.books_per_day;
        let num_books = cmp::min(num_books_to_ship, self.books.len());
        let possible_score: usize = self
            .books
            .iter()
            .take(num_books)
            .map(|index| problem_books[*index])
            .sum();

        let score = (possible_score as f64) / ((self.sign_up_days * 6) + remaining_days) as f64;

        return LibraryScore {
            score,
            num_books_to_ship,
            index,
        };
    }
}

fn make_problem(letter: &str) -> Problem {
    let filename = format!("data/{}.txt", letter);
    let mut lines: Lines<BufReader<File>> = BufReader::new(File::open(filename).unwrap()).lines();

    let problem_desc = split_next_vec(&mut lines);
    let num_libraries = *problem_desc.index(1);
    let days = *problem_desc.index(2);
    let books: Vec<usize> = split_next_vec(&mut lines);

    let libraries = (0..num_libraries)
        .map(|i| make_library(&mut lines, i))
        .collect();

    return Problem {
        libraries,
        books,
        days,
    };
}

fn make_library(lines: &mut Lines<BufReader<File>>, id: usize) -> Library {
    let library_desc = split_next_vec(lines);
    let sign_up_days = *library_desc.index(1);
    let books_per_day = *library_desc.index(2);
    let mut books = split_next_vec(lines);
    books.sort_unstable_by(|a, b| b.cmp(a));

    return Library {
        id,
        books,
        sign_up_days,
        books_per_day,
    };
}

fn split_next_vec(lines: &mut Lines<BufReader<File>>) -> Vec<usize> {
    let next = lines.next().unwrap().unwrap();
    let ints: Vec<usize> = next.split(' ').map(|int| int.parse().unwrap()).collect();

    return ints;
}

#[derive(Debug)]
struct Problem {
    libraries: Vec<Library>,
    books: Vec<usize>,
    days: usize,
}

#[derive(Debug)]
struct Library {
    id: usize,
    books: Vec<usize>,
    sign_up_days: usize,
    books_per_day: usize,
}

#[derive(Debug, Default, PartialOrd, PartialEq)]
struct LibraryScore {
    index: usize,
    num_books_to_ship: usize,
    score: f64,
}
