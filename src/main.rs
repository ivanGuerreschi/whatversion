// Copyright (c) 2023 Ivan Guerreschi. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use whatversion::{csv, fetch, local_version};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    const FILE: &str = "/.apps.csv";

    let apps = csv::read_lines().await;
    match apps = {
        Ok(apps_line) => {
            for app in apps_line.skip(1) {
                let apps_name = app;
                match apps_name {
                    Ok() => {},
                    Err(error) => panic!(),
                }
            }
        },
        Err(error) => panic!("Error fetch data from GitHub: {:?}", error),
    }

    if let Ok(apps) = csv::read_lines(FILE).await {
        for app in apps.skip(1) {
            if let Ok(apps_name) = app {
                let app_values: Vec<&str> = apps_name.split(',').collect();
                if let Ok(output) = local_version::command(app_values[0]).await {
                    if let Ok(release) = fetch::release(app_values[1], app_values[2]).await {
                        println!(
                            "Local version of {}last version in GitHub repo is {}\nurl {}\n",
                            String::from_utf8_lossy(&output.stdout),
                            release.tag_name,
                            release.html_url
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
