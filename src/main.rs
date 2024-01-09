use reqwest::Error;

async fn get_request() -> Result<(), Error> {






	let response = reqwest::put("http://httpserver.local:8080/led/1/").await?;
	println!("Status {}", response.status());

	let body = response.text().await?;
	println!("Body: \n{}", body);

	Ok(())
}

async fn get_request() -> Result<(), Error> {
	let response = reqwest::get("http://httpserver.local:8080/led/1/").await?;
	println!("Status {}", response.status());

	let body = response.text().await?;
	println!("Body: \n{}", body);

	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error>
{

	put_request().await?;
	get_request().await?;



	let mut input



	Ok(())
}
