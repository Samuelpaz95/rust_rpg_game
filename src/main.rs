use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
struct History {
    tipo: String,
    tag: String,
    texto: String,
    vida: i32,
    opciones: Vec<History>,
}

impl History {
    fn new(row: StringRecord) -> History {
        let data: History = History {
            tipo: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            texto: row.get(2).unwrap().trim().to_string(),
            vida: row.get(3).unwrap().trim().parse().unwrap_or(0),
            opciones: Vec::new(),
        };
        return data;
    }
}

fn print_state(data: &History) {
    let width = term_size::dimensions().unwrap().0;

    println!("\x1b[30m\x1b[43m{:^width$}\x1b[0m", "", width = width);

    println!(
        "\x1b[1m\x1b[30m\x1b[43m{:^width$}\x1b[0m",
        data.texto,
        width = width
    );

    if data.vida > 0 {
        println!(
            "\x1b[30m\x1b[42m{:^width$}\x1b[0m\n",
            format!("Vida +{}", data.vida),
            width = width
        );
    } else if data.vida < 0 {
        println!(
            "\x1b[30m\x1b[41m{:^width$}\x1b[0m\n",
            format!("Daño {}", data.vida),
            width = width
        );
    } else {
        println!("\x1b[30m\x1b[43m{:^width$}\x1b[0m", "", width = width);
    }
}

fn print_options(data: &History) {
    println!("\x1b[32m[options]\x1b[0m");
    for (index, option) in data.opciones.iter().enumerate() {
        // print options with green color
        println!("  \x1b[32m[{}]\x1b[0m {}", index + 1, option.texto);
    }
}

fn ask_option() -> usize {
    let mut selection: String = String::new();
    std::io::stdin().read_line(&mut selection).unwrap();
    let selection: usize = selection.trim().parse().unwrap_or(99);
    return selection;
}

fn main() {
    let mut hp: i32 = 100;
    let mut current_tag: &str = FIRST_TAG;

    let mut last_record: String = "".to_string();

    let mut history_data: HashMap<String, History> = HashMap::new();

    let content = fs::read_to_string(FILENAME).unwrap();
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in reader.records() {
        let record: StringRecord = result.unwrap();
        let history: History = History::new(record);

        if history.tipo == "SITUACION" {
            let record_tag = history.tag.clone().trim().to_string();
            history_data.insert(record_tag.clone(), history);
            last_record = record_tag;
        } else if history.tipo == "OPCION" {
            if let Some(data) = history_data.get_mut(&last_record) {
                (*data).opciones.push(history);
            }
        }
    }

    // Game loop

    loop {
        println!("HP: {}", hp);
        if hp <= 0 {
            println!("Game Over");
            break;
        }

        if let Some(data) = history_data.get(current_tag) {
            print_state(data);
            if data.opciones.len() == 0 {
                break;
            }
            print_options(data);
            let selection: usize = ask_option();

            if let Some(option) = data.opciones.get(selection - 1) {
                current_tag = option.tag.trim();
            } else {
                println!("No existe esa opcion");
            }
            println!("");

            hp += data.vida;
        } else {
            println!("No hay mas historia");
            break;
        }
    }
}
