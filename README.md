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
To make json output more readable, use `jq` command. You can install it from [here](https://stedolan.github.io/jq/download/).

Following commands shows how crawler start scraping from `johnlewis.com`.

```bash
cargo run -- -s johnlewis_com  | jq '.'

```

Crawling only 10 products from a shop:

```bash
cargo run -- -s johnlewis_com --limit-pages 10 

```

Filter products by keyword:

```bash
cargo run -- -s johnlewis_com --filter-keyword original
```
Filter products by URL:

```bash
cargo run -- -s johnlewis_com --filter-url /p54
```
Combine filters:

```bash
cargo run -- -s johnlewis_com --filter-url /p54 --filter-keyword original
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

