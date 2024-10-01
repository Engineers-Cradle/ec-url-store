#!/bin/bash

# Create the db directory if it doesn't exist
if [ ! -d "crates/geo-ip/db" ]; then
    mkdir -p crates/geo-ip/db
fi

# Check if the GeoLite2-City.mmdb file exists
if [ ! -f "crates/geo-ip/db/GeoLite2-City.mmdb" ]; then
    current_date=$(date +"%Y-%m")
    # Decrement the current date by 1 month
    current_date=$(date -d "1 month ago" +"%Y-%m")
    d_url="https://download.db-ip.com/free/dbip-city-lite-${current_date}.mmdb.gz"
    echo $d_url
    tar_gz="crates/geo-ip/db/GeoLite2-City.mmdb.gz"
    final_out_name="crates/geo-ip/db/GeoLite2-City.mmdb"

    curl -o $tar_gz $d_url

    gzip -d $tar_gz
fi