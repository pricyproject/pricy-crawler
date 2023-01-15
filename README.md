# Cheappy Crawler




#### Usage:

Help command:
```bash
cargo run -- -h
```


List of available shops:
```bash
cargo run -- -l

```

Following commands shows how crawler start scraping from `johnlewis.com`.

```bash
cargo run -- -s johnlewis_com  | jq '.'

```

##### Save sitemap links on storage:

```bash
cargo run -- -s johnlewis_com --save_sitemap | jq '.'
```


Crawl a single product:

```bash
echo https://www.johnlewis.com/john-lewis-partners-jl111-wildflower-print-sewing-machine-blue/p5548442 | cargo run -- -p
```

