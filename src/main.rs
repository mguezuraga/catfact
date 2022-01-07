use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, help = "Display multiple cat facts")]
    multiple: bool,
}

#[derive(Deserialize, Debug)]
struct CatFact {
    fact: String,
}

#[derive(Deserialize, Debug)]
struct CatFacts {
    data: Vec<CatFact>,
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[allow(dead_code)]
fn json_from_string() -> String {
    r#"
    {
        "current_page": 1,
        "data": [
          {
            "fact": "A",
            "length": 1
          },
          {
            "fact": "AB",
            "length": 2
          }
        ]
    }
    "#.to_string()
}

fn fetch_random_facts() -> CatFacts {

    let url = "https://catfact.ninja/facts";
    match ureq::get(url).call() {
        Ok(response) => {
            response.into_json().unwrap()
        },
        Err(_) => {
            println!("Something went wrong while fetching facts");
            std::process::exit(1);
        }
    }
}

fn fetch_one_random_fact() -> CatFact {
    let url = "https://catfact.ninja/fact";
    match ureq::get(url).call() {
        Ok(response) => {
            response.into_json().unwrap()
        },
        Err(_) => {
            println!("Something went wrong while fetching facts");
            std::process::exit(1);
        }
    }

}

fn main()  {
    let args = Cli::from_args();

    if args.multiple {
        let cat_facts: CatFacts = fetch_random_facts();
        for cat_fact in &cat_facts.data {
            println!("* {}\n", cat_fact.fact);
        }
    } else {
        let cat_fact: CatFact = fetch_one_random_fact();
        println!("{:#?}", cat_fact.fact);
    }

}