use r::Post;
use r34_api as r;
use std::{
    fs,
    io::Write,
    path::{self, Path},
};

pub async fn download_imgs(default_blacklist: bool, mut tags: Vec<String>) {
    if default_blacklist {
        tags.extend_from_slice(&[
            "-ai_generated".to_string(),
            "-scat".to_string(),
            "-inflation".to_string(),
            "-fart".to_string(),
        ]);
    }

    let posts = get_posts(10, &tags).await.unwrap();
    mass_download(posts, "./imgs/").await;
}

async fn get_posts<S: ToString>(amount: usize, tags: &[S]) -> anyhow::Result<Vec<Post>> {
    let tags: Vec<String> = tags.iter().map(|s| s.to_string()).collect();

    let req_url = r::ApiUrl::new()
        .add_tags(tags)
        .set_pid(1)
        .set_limit(amount)
        .to_api_url();

    let res = reqwest::get(req_url).await?;

    let res = res.text().await?;

    let posts = match r::R34JsonParser::new().parse_json(&res) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    Ok(posts)
}

async fn mass_download(posts: Vec<r::Post>, savepth: &str) {
    let mut paths = Vec::new();

    println!("Started Download...");

    for post in posts {
        let filename = post.image;
        let url = post.file_url;
        let filepath = format!("{}{}", savepth, filename);
        if !path::Path::new(&filepath).exists() {
            match reqwest::get(url).await {
                Ok(res) => match res.bytes().await {
                    Ok(b) => {
                        let mut f = fs::File::create_new(&filepath).unwrap();
                        f.write_all(&b).unwrap();
                        paths.push(filepath);
                    }
                    Err(_e) => continue,
                },
                Err(_e) => continue,
            }
        }
    }

    save_paths(paths);
}

fn save_paths(paths: Vec<String>) {
    let path = "./paths";
    if !Path::new(path).exists() {
        let mut f = fs::File::create(path).unwrap();

        for p in paths {
            let pat = format!("{}\n", p);
            f.write_all(pat.as_bytes()).unwrap();
        }
    } else {
        let mut f = fs::File::options().append(true).open(path).unwrap();
        for p in paths {
            let pat = format!("{}\n", p);
            f.write_all(pat.as_bytes()).unwrap();
        }
    }
}
