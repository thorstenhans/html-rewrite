# Anti-Scraping Through Edge HTML Rewriting

A WebAssembly application built with Spin and `lol_html` demonstrating high-performance reverse proxying and in-flight HTML rewriting for edge-level anti-scraping and data obfuscation.

## Build and Run

```bash
spin up --build
```

## Verification & Testing

By default, the application proxies [Books to Scrape](https://books.toscrape.com/) (a standard e-commerce sandbox for scraping demos).

## Price Obfuscation

Fetch the homepage through the local Spin proxy:

```bash
curl -s http://localhost:3000/ | grep -C 1 "price_color"
```

**Proxy Output** (sanitized):
```html
<p class="price_color">Contact Sales</p>
```

In contrast, fetching directly from the upstream origin reveals the original prices:

```bash
curl -s https://books.toscrape.com/ | grep -C 1 "price_color"
```

**Origin Output**:
```html
<p class="price_color">£51.77</p>
```

## Availability Stripping

Check for stock availability classes:

```bash
curl -s http://localhost:3000/ | grep -C 1 "availability"
```

**Proxy Output**:
> Returns nothing (the matching `<p class="instock availability">` elements are removed from the markup).

**Origin Output**:
```bash
curl -s https://books.toscrape.com/ | grep -C 1 "availability"
```
```html
<p class="instock availability">
    <i class="icon-ok"></i>
```