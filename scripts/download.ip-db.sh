#!/bin/bash

# let mmdb_dir = std::path::PathBuf::from("src/db");

# if !mmdb_dir.exists() {
#     std::fs::create_dir_all(&mmdb_dir).unwrap();
# }

# // Check if the GeoLite2-City.mmdb file exists
# let mmdb_file = mmdb_dir.join("GeoLite2-City.mmdb");

# if !mmdb_file.exists() {
#     let current_date: String = Utc::now().format("%Y-%m").to_string();
#     let d_url = format!("https://download.db-ip.com/free/dbip-city-lite-{}.mmdb.gz", current_date);
#     let tar_gz = mmdb_dir.join("GeoLite2-City.mmdb.gz");
#     let final_out_name = mmdb_dir.join("GeoLite2-City.mmdb");

#     let _ = std::process::Command::new("curl")
#         .arg("-o")
#         .arg(&tar_gz)
#         .arg(d_url)
#         .output()
#         .expect("Failed to download GeoLite2-City.tar.gz");

#     let _ = std::process::Command::new("gzip")
#         .arg("-d")
#         .arg(&tar_gz)
#         .output()
#         .expect("Failed to extract GeoLite2-City.tar.gz");

#     let current_date: String = Utc::now().format("%Y-%m").to_string();

#     let outfile_name = mmdb_dir.join(format!("dbip-city-lite-{}.mmdb", current_date));

#     let _ = std::fs::rename(&outfile_name, &final_out_name);

#     let _ = std::fs::remove_file(&tar_gz);
# }

# Convert the above script to a shell script

# Create the db directory if it doesn't exist
if [ ! -d "crates/geo-ip/src/db" ]; then
    mkdir -p crates/geo-ip/src/db
fi

# Check if the GeoLite2-City.mmdb file exists
if [ ! -f "crates/geo-ip/src/db/GeoLite2-City.mmdb" ]; then
    current_date=$(date +"%Y-%m")
    # Decrement the current date by 1 month
    current_date=$(date -d "1 month ago" +"%Y-%m")
    d_url="https://download.db-ip.com/free/dbip-city-lite-${current_date}.mmdb.gz"
    echo $d_url
    tar_gz="crates/geo-ip/src/db/GeoLite2-City.mmdb.gz"
    final_out_name="crates/geo-ip/src/db/GeoLite2-City.mmdb"

    curl -o $tar_gz $d_url

    gzip -d $tar_gz
fi