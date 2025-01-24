use esix::{error::Error, Esix};

fn main() -> Result<(), Error> {
    let mut client = Esix::new(
        "API_KEY",
        "USERNAME",
        "project_name".to_string(),
        "project_version".to_string(),
    );

    let posts = client.list("rating:safe", 1)?;

    for post in posts {
        println!("{:?}", post);
    }

    Ok(())
}
