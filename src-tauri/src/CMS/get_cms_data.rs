use reqwest::header;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    filter: String,
    page: i64,
    sort: String,
}

#[derive(Serialize)]
pub struct SearchResult {
    success: bool,
    data: Option<String>,
    error: Option<String>,
}

#[tauri::command]
pub async fn perform_search(
    filter: String,
    page: i64,
    sort: String,
) -> Result<SearchResult, String> {
    async move {
        let client = reqwest::Client::new();

        let form = reqwest::multipart::Form::new()
            .text("csrfmiddlewaretoken", "jslSdvIXi6Xr7oTmLqLXa0ldpQRoYuGZCUk7dh4fws8EYwJC9EGiELBW4KsAaMWE")
            .text("search_type", "t")
            .text("q", filter)
            .text("consume", "")
            .text("product", "")
            .text("loader_type", "any")
            .text("mc_type", "any")
            .text("create_type", "any")
            .text("z_size", "")
            .text("x_size", "")
            .text("y_size", "")
            .text("function", "")
            .text("grid_col", "1")
            .text("sort", "time")
            .text("order", sort)
            .text("page", page.to_string());

        let response = client
            .post("https://www.creativemechanicserver.com/search/")
            .multipart(form)
            .header(header::COOKIE, "csrftoken=tC9paWwsowln1i0qyo5vEVqTP4LmmsqP", )
            .header(header::REFERER, "https://www.creativemechanicserver.com/", )
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(SearchResult {
                success: false,
                data: None,
                error: Some(format!("HTTP Error: {}, data:{}", response.status(), response.text().await.unwrap())),
            });
        }

        let data = response.text().await?;

        Ok(SearchResult {
            success: true,
            data: Some(data),
            error: None,
        })
    }
        .await
        .map_err(|e: anyhow::Error| e.to_string())
}