use std::fs;
use std::fs::{File, read_to_string};
use std::io::Write;
use rustube::{Id, VideoFetcher};

#[tokio::main]
async fn main() -> Result<(), ()>  {

    let links : Vec<String> = read_to_string("links.txt").unwrap().lines()
        .filter(|line| line.len() > 0 && line.to_lowercase().contains("http"))
        .map(String::from).collect();

    let mut log_file = File::create("log.txt").unwrap();

    let mut counter = 1;
    for l in &links {

        let id = Id::from_raw("https://www.youtube.com/watch?v=bKldI-XGHIw").unwrap();
        let descrambler = VideoFetcher::from_id(id.into_owned()).unwrap()
            .fetch()
            .await.unwrap();

        let video_info = descrambler.video_info();


        let msg = format!("{}/{} Title: {}\nstart download {}", counter, links.len(),  video_info.player_response.video_details.title, l);

        log_file.write(msg.into_bytes().as_slice()).expect("TODO: panic message");

        let path = rustube::download_worst_quality(l.as_str()).await.unwrap();

        log_file.write(format!("{}", path.to_str().unwrap()).into_bytes().as_slice()).expect("TODO: panic message");
        // println!("{}", path.to_str().unwrap());

        counter += 1;
    }

    Ok(())
}
