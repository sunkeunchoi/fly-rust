use crate::types::{Detail, Page};
use anyhow::{bail, Result};
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, UPGRADE_INSECURE_REQUESTS, USER_AGENT,
};
use reqwest::Client;
pub async fn get_pages(novel_id: u32) -> Result<Vec<Page>> {
    let url = format!("https://booktoki156.com/novel/{novel_id}");
    let html = get_html(&url).await?;
    let is_blocked = html.contains("귀하는 240시간동안 접근 불가합니다.");
    if is_blocked {
        bail!("Site blocked");
    }
    Ok(parse_pages(&html))
}
pub async fn get_details(path: String) -> Result<Detail> {
    let url = format!("https://booktoki156.com{path}?spage=1");
    let html = get_html(&url).await?;
    let is_blocked = html.contains("귀하는 240시간동안 접근 불가합니다.");
    if is_blocked {
        bail!("Site blocked");
    }
    Ok(parse_detail(&html))
}
pub async fn get_html(path: &str) -> Result<String> {
    let url = url::Url::parse(path)?;
    let client = Client::new();
    let res = client.get(url)
    .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36")
    .header(ACCEPT_LANGUAGE, "en-US,en;q=0.6")
        .header(CACHE_CONTROL, "max-age=0")
        .header(UPGRADE_INSECURE_REQUESTS,"1")
        .header(ACCEPT,"text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
        .header(ACCEPT_ENCODING, "gzip, deflate, br")
        .send().await?.text().await?;
    Ok(res)
}
pub fn parse_detail(html: &str) -> Detail {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("#novel_content").unwrap();
    let p_tag = scraper::Selector::parse("p").unwrap();
    let contents = document
        .select(&selector)
        .next()
        .unwrap()
        .select(&p_tag)
        .map(|e| e.text().collect::<Vec<_>>().join(""))
        .collect::<Vec<_>>();
    let t_tag = scraper::Selector::parse("#at-main > div.view-wrap > section > article > div.comic-navbar > div > div.toon-info > div.toon-title").unwrap();
    let title = document
        .select(&t_tag)
        .next()
        .unwrap()
        .value()
        .attr("title")
        .unwrap()
        .to_owned();
    Detail { title, contents }
}
pub fn parse_pages(html: &str) -> Vec<Page> {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("ul.list-body > li.list-item").unwrap();
    // let data = document
    //     .select(&selector)
    //     .map(|e| e.text().collect::<Vec<&str>>().join(" "))
    //     .collect::<Vec<String>>();
    let a_tag = scraper::Selector::parse("div.wr-subject > a").unwrap();
    let num_tag = scraper::Selector::parse("div.wr-num").unwrap();
    let data = document
        .select(&selector)
        .map(|e| {
            let page_no = e
                .select(&num_tag)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let href = e
                .select(&a_tag)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap();
            Page {
                page_no,
                path: url::Url::parse(href).unwrap().path().to_owned(),
            }
        })
        .collect();
    data
}
