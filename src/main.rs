#[derive(serde::Deserialize, Debug)]
struct CRTEntry {
  issuer_ca_id: i32,
  issuer_name: String,
  name_value: String,
  id: i128,
  entry_timestamp: String,
  not_before: String,
  not_after: String
}

async fn do_request (domain: &str) -> Result<Vec<CRTEntry>, reqwest::Error> {
  let url = format!("https://crt.sh/?q={}&accept=json", domain);
  let client = reqwest::Client::new();

  let body = client
    .get(&url)
    .header("Accept", "application/json")
    .send()
    .await?
    .text()
    .await?;

  let entries: Vec<CRTEntry> = serde_json::from_str(&body).unwrap();

  Ok(entries)
}

#[tokio::main]
pub async fn main() -> Result<(), ()> {
  let matches = clap::App::new("ctl-rs")
    .version("1.0")
    .author("Adam Brady <adam@boxxen.org>")
    .arg("<domain>")
    .get_matches();

  let domain = match matches.value_of("domain") {
    Some(domain) => domain,
    None => panic!("should not get here")
  };

  let spinner = spinners::Spinner::new(spinners::Spinners::Arc, "".to_owned());

  let result = futures::executor::block_on(do_request(&domain));

  if let Err(e) = result {
    panic!(format!("Error fetching: {}", e));
  }

  spinner.stop();

  let entries = result.unwrap();

  // Surely a better way to de-dupe?
  let mut unique_domains: Vec<String> = Vec::new();

  for entry in entries.iter() {
    for split_domain in entry.name_value.split("\n") {
      if !unique_domains.contains(&String::from(split_domain)) {
        unique_domains.push(String::from(split_domain));
      }
    }
  }

  println!("List of domains found:");

  for domain in unique_domains.iter() {
    println!("- {}", domain);
  }
  
  println!("Found total of {} unqiue certificate-domains for {}", unique_domains.len(), domain);

  Ok(())
}
