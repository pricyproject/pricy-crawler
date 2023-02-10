# Pricy Crawler




#### Usage:

Help command:
```bash
cargo run -- -h
```

Pricy bot is fully modular. You can easily add a new shop to it.
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
cargo run -- -s johnlewis_com --save_sitemap
```

- If sitemap is in `gzip` format

```bash
cargo run -- -s yourshop_com --save_sitemap --gzip true
```


Crawl a single product:

```bash
echo https://www.johnlewis.com/john-lewis-partners-jl111-wildflower-print-sewing-machine-blue/p5548442 | cargo run -- -p
```

Crawl multiple products from different shops:


```bash
cargo run -- -m https://www.johnlewis.com/john-lewis-partners-jl111-wildflower-print-sewing-machine-blue/p5548442,https://www.johnlewis.com/john-lewis-partners-jl111-wildflower-print-sewing-machine-blue/p552242

```

What is the format of crawled data?
[Sample](./sample-crawled-products.json)

