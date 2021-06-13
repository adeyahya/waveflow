#!/usr/bin/env zx
import path from "path";
const server_binary = path.join(__dirname, "../server/target/release/waveflow");
const frontend_files = path.join(__dirname, "../www/dist");
const dist_path = path.join(__dirname, "../dist/waveflow");
const server_path = path.join(__dirname, "../server");

// prepare db
await $`cd ${server_path} && diesel migration run --database-url=./waveflow.db`;
await $`mkdir -p ${dist_path}`;
await $`cp ${server_path}/waveflow.db ${dist_path}/waveflow.db`;
await $`cp ${server_binary} ${dist_path}/waveflow`;
await $`cp ${__dirname}/waveflow.db ${dist_path}/waveflow.db`;
await $`mkdir -p ${dist_path}/frontend`;
await $`cp -r ${frontend_files}/* ${dist_path}/frontend`;
await $`cd ${dist_path}/../ && zip -r waveflow.zip ./*`;
