use reqwest::StatusCode;

static REQ_PAYLOAD: &str = "{\n\t\"payload\": \"world\"\n}\n";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = std::env::args().collect::<Vec<_>>();
	if args.len() != 4 {
		println!("Usage: {} url thread-count request-count", args[0]);
		return Ok(());
	}
	
	let url = args[1].clone();
	let request_count = args[2].parse().unwrap();
	let thread_count = args[3].parse().unwrap();
	
	let client = reqwest::Client::new();
	let start = std::time::Instant::now();
	let handles = (0..thread_count).map(|_| {
		let url = url.clone();
		let client = client.clone();
		tokio::spawn(async move {
			let mut error_count = 0;
			let mut results = Vec::new();
			for _ in 0..request_count {
				let start = std::time::Instant::now();
				let res = client
					.post(&url)
					.header("Content-Type", "application/json")
					.body(REQ_PAYLOAD)
					.send()
					.await
					.unwrap();
				if res.status() == StatusCode::OK {
					res.text().await.unwrap();
					let elapsed = std::time::Instant::now().duration_since(start);
					results.push(elapsed.as_micros());
				} else {
					error_count += 1;
				}
			}
			(results, error_count)
		})
	}).collect::<Vec<_>>();
	
	let mut results = Vec::new();
	let mut error_count = 0;
	for handle in handles {
		let (out, err_count) = handle.await.unwrap();
		results.extend(out.into_iter());
		error_count += err_count;
	}
	let elapsed = std::time::Instant::now().duration_since(start);
	let rps = results.len() as f64 / elapsed.as_secs_f64();
	
	results.sort();
	println!(
		"average={}us, median={}us, errors={}, total={}, rps={}",
		results.iter()
			.copied()
			.reduce(|a, b| a + b).unwrap() / results.len() as u128,
		results[results.len() / 2],
		error_count,
		results.len(),
		rps
	);
	Ok(())
}
