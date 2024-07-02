//----------------------------------------
//          Project: Web-Scraper
//----------------------------------------

use std::{sync::Arc, thread, time::Instant};

fn main() -> Result<(), ureq::Error> {
    let webpages = vec![
        "https://www.hitbullseye.com/",
        "https://imsindia.com/center/chandigarh/",
        "https://www.careerlauncher.com/",
    ];
    let agent = ureq::AgentBuilder::new().build();
    let now = Instant::now();

    for webpage in &webpages {
        let web_body = agent.get(webpage).call()?.into_string()?;
    }
    println!("Time taken without threads: {:.2?}", now.elapsed());

    let now = Instant::now();
    let agent = Arc::new(agent);
    let mut handles: Vec<thread::JoinHandle<Result<(), ureq::Error>>> = Vec::new();

    for webpage in webpages {
        let agent_thread = agent.clone();
        let t = thread::spawn(move || {
            let web_body = agent_thread.get(webpage).call()?.into_string()?;

            Ok(())
        });

        handles.push(t);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Time taken using Threads: {:.2?}", now.elapsed());

    Ok(())
}
