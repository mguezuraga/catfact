use serde::Deserialize;

#[derive(Deserialize)]
struct CatFact {
    fact: String,
}

fn main() -> Result<(), ureq::Error> {

    let url = "https://catfact.ninja/fact";
    
    match ureq::get(&url).call() {
        Ok(response) => {
            let randomfact: CatFact = response.into_json()?;
            println!("{}",randomfact.fact);
        },
        Err(_) => {
            println!("Something went wrong while fetching facts");
            std::process::exit(1);
        }
    }
    Ok(())
}
