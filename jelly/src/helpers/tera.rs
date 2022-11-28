use std::env;
use std::fmt::Display;
use crate::server::ROUTES_KEY;
use std::collections::HashMap;
use std::collections::VecDeque;
use serde_json::Error as Jerror;
use actix_web::test::TestRequest;
use tera::{Value, to_value, Error};
use serde::{Deserialize, Serialize};

const HASH: &str = "#";
const GAP: &str = "...";
const NEXT: &str = "Next";
const PREVIOUS: &str = "Previous";

/// A `PaginationItem` is a unit item for pagination
/// used in tera function
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PaginationItem {
    pub page: String,
    pub link: String,
    pub active: bool
}

impl From<(&str, i64, i64)> for PaginationItem{
    fn from(item: (&str, i64, i64)) -> Self {
        let (page, current_page, page_count) = item;
        let mut link = page.to_string();
        let mut active = false;
        if page == NEXT {
            link = if current_page > 1 {format!("{}",current_page - 1)} else{HASH.into()};
        }
        if page == NEXT {
            link = if current_page < page_count {format!("{}",current_page + 1)} else{HASH.into()};
        }
        if !matches!(page, GAP | PREVIOUS | NEXT) {
            if Ok(current_page) == page.parse() {
                active = true;
            }
        }
        PaginationItem{page:page.into(), link, active}
    }
}

pub struct PaginationValue {
    current_page: i64, 
    number_of_pages: i64
}

impl PaginationValue {
    pub fn new(current_page:i64, number_of_pages:i64)->Self{
        PaginationValue{current_page, number_of_pages}
    }

    pub fn create_page_item<T:Display>(&self, page: T)->Result<Value, Jerror>{
        to_value::<PaginationItem>(
            (page.to_string().as_str(), self.current_page, self.number_of_pages).into()
        )
    }

}

pub fn url_for(args: &HashMap<String, Value>) -> Result<Value, Error> {
    let name = args["name"].as_str().ok_or(Error::msg("`name` should be a string"))?;
    let empty_elements = Value::Array(vec![]);
    let elements_iter = args.get("elements").unwrap_or(&empty_elements)
        .as_array().ok_or(Error::msg("`elements` should be an array"))?.iter();
    let mut elements = vec![];
    for elem in elements_iter {
        elements.push(elem.as_str().ok_or(
                Error::msg("`elements` array should contain only strings")
            )?
        );
    }
    ROUTES_KEY.with(|routes| {
        let mut route_option = routes.borrow_mut();
        let routes = route_option.as_mut().ok_or(
            Error::msg("`url_for` should only be called in request context")
        )?;
        let fake_req = TestRequest::default().to_http_request();
        let url = routes.url_for(&fake_req, name, elements)
            .or(Err(Error::msg(format!("`{}` resource not found",name))))?;
        Ok(Value::String(url.path().replace("//", "/").to_string()))
    })
}

pub fn url(args: &HashMap<String, Value>) -> Result<Value, Error> {
    let path = args.get("path").ok_or(Error::msg("`path` is required"))?
    .as_str().ok_or(Error::msg("`path` should be a string"))?;
    let fake_req = TestRequest::default().to_http_request();
    let mut host:String = fake_req.uri().host().unwrap_or("").into();
    match fake_req.uri().port(){
        Some(port) if !port.as_str().contains("80") && !host.is_empty()=>{
            host = format!("{}:{}", host, port.as_str());
        },
        _=>()
    }
    Ok(Value::String(format!("{}{}", host, path)))
}

pub fn is_active_url(args: &HashMap<String, Value>) -> Result<Value, Error> {
    let url = args.get("route").ok_or(Error::msg("`route` is required"))?
        .as_str().ok_or(Error::msg("`route` should be a string"))?;
    let current_path = args.get("current_path")
        .ok_or(Error::msg("`current_path` is required"))?
        .as_str().ok_or(Error::msg("`current_path` should be a string"))?;
    let cls = if current_path.contains(url) {"active"} else {""};
    Ok(Value::String(cls.into()))
}

pub fn config(args: &HashMap<String, Value>)-> Result<Value, Error> {
    let key = args.get("key").ok_or(Error::msg("`key` is required"))?
        .as_str().ok_or(Error::msg("`key` should be a string"))?;
    let value = Value::String("".into());
    let default = args.get("default")
        .or(Some(&value)).unwrap().as_str().unwrap();
    Ok(Value::String(env::var(key).unwrap_or(default.into())))
}

pub fn pagination(args: &HashMap<String, Value>)-> Result<Value, Error> {
    let (current_page, per_page, total) = _prepare_pagination_params(args)?;
    let mut number_of_pages = total / per_page;
    if (total % per_page) != 0i64  {
        number_of_pages += 1
    }

    let center = vec![
        current_page - 2, current_page - 1, current_page, current_page + 1, current_page + 2
    ];
    let page_value = PaginationValue::new(current_page, number_of_pages);
    let mut center_deque:VecDeque<Value> = center.iter()
        .filter_map(|&p| {
            if p > 1i64 && p < number_of_pages {
                page_value.create_page_item(p).ok()
            }else{None}
        }).collect();
    let (
        include_three_left, include_three_right, 
        include_left_dots, include_right_dots
    ) = _get_include_check(current_page, number_of_pages)?;

    if include_three_left {center_deque.push_front(page_value.create_page_item("2")?);}
    if include_three_right {
        center_deque.push_back(page_value.create_page_item(number_of_pages - 1i64)?);
    }
    if include_left_dots {center_deque.push_front(page_value.create_page_item(GAP)?);}
    if include_right_dots {center_deque.push_back(page_value.create_page_item(GAP)?);}
    center_deque.push_front(page_value.create_page_item("1")?);
    if number_of_pages > 1i64 {
        center_deque.push_back(page_value.create_page_item(number_of_pages)?);
    }
    center_deque.push_front(page_value.create_page_item(PREVIOUS)?);
    center_deque.push_back(page_value.create_page_item(NEXT)?);
    Ok(Value::Array(center_deque.into()))
    // Ok(Value::Array(vec![]))
}

fn _get_include_check(current_page:i64, number_of_pages:i64)-> Result<(bool, bool, bool, bool), Error>{
    let include_three_left = current_page == 5;
    let include_three_right = current_page == number_of_pages - 4;
    let include_left_dots = current_page > 5;
    let include_right_dots = current_page < number_of_pages - 4;
    Ok((include_three_left, include_three_right, include_left_dots, include_right_dots))
}

fn _prepare_pagination_params(args: &HashMap<String, Value>)-> Result<(i64, i64, i64), Error>{
    let value = Value::String("1".into());
    let current_page = args.get("current_page")
        .or(Some(&value)).unwrap()
        .as_i64().ok_or(Error::msg("`current_page` should be a numeric"))?;
    let value = Value::String("10".into());
    let mut per_page = args.get("per_page")
        .or(Some(&value)).unwrap()
        .as_i64().ok_or(Error::msg("`per_page` should be a numeric"))?;
    let total = args.get("total")
        .ok_or(Error::msg("`total` is required"))?
        .as_i64().ok_or(Error::msg("`total` should be a numeric"))?;
    if per_page.eq(&0i64){
        per_page = 10;
    }
    Ok((current_page, per_page, total))
}

// fn pagination(args: &HashMap<String, Value>)-> Result<Value, Error> {
//     let (current_page, per_page, total) = _prepare_pagination_params(args)?;
//     let mut number_of_pages = total / per_page;
//     if (total % per_page) != 0i64  {
//         number_of_pages += 1
//     }

//     let mut pagination = Vec::new();

//     // if pages exists after loop's lower limit
//     if number_of_pages > 1i64 {
//         pagination.push(
//             to_value(
//                 PaginationItem{
//                     page: "Previous".into(),
//                     link: if current_page > 1 {format!("{}",current_page - 1)} else{"#".into()},
//                     active: false
//                 }
//             )?
//         );

//         if current_page > 3 {
//             if (current_page - 3) > 0i64 {
//                 pagination.push(
//                     to_value(PaginationItem{ page: "1".into(), link: "1".into(), active: false })?
//                 );
//             }
//             if (current_page - 3) > 1i64 {
//                 pagination.push(
//                     to_value(PaginationItem{ page: "...".into(), link: "#".into(), active: false})?
//                 );
//             }
//         }

//         // Loop for provides links for 2 pages before and after current page
//         if current_page > 1 {
//             for i in (current_page - 2)..=(current_page + 2) {
//                 if i < 1i64 {continue}
//                 if i > number_of_pages {break}
//                 if (current_page - 3) > 0i64 {
//                     pagination.push(
//                         to_value(
//                             PaginationItem{ 
//                                 page: format!("{i:}"), link: format!("{i:}"), active: i.eq(&current_page)  
//                             }
//                         )?
//                     );
//                 }
//             }
//         }

//         // if pages exists after loop's upper limit
//         if (number_of_pages - current_page + 2) > 1i64 {
//             pagination.push(
//                 to_value(PaginationItem{ page: "...".into(), link: "#".into(), active: false})?
//             );
//         }

//         if (number_of_pages as i64 - current_page as i64 + 2) > 0i64 {
//             let is_current_page = number_of_pages.eq(&current_page);
//             pagination.push(
//                 to_value(
//                     PaginationItem{ 
//                         page: format!("{number_of_pages:}"), 
//                         link: if is_current_page {"#".into()}else{format!("{number_of_pages:}")}, 
//                         active: is_current_page
//                     }
//                 )?
//             );
//         }

//         pagination.push(
//             to_value(
//                 PaginationItem{ 
//                     page: "Next".into(),
//                     link: if current_page < number_of_pages {format!("{}",current_page + 1)} else{"#".into()},
//                     active: false
//                 }
//             )?
//         );
//     }
//     Ok(Value::Array(pagination))
// }

// use std::collections::VecDeque;
// fn _pagination(current_page:i64, total_number_of_pages:i64)->VecDeque<String>{
//     let mut center = vec![
//         current_page - 2, current_page - 1, current_page, current_page + 1, current_page + 2
//     ];
//     let mut center_deque:VecDeque<String> = center.iter()
//         .filter(|&p| *p > 1i64 && *p < total_number_of_pages).map(i64::to_string).collect();
//     let include_three_left = current_page == 5;
//     let include_three_right = current_page == total_number_of_pages - 4;
//     let include_left_dots = current_page > 5;
//     let include_right_dots = current_page < total_number_of_pages - 4;

//     if include_three_left {center_deque.push_front("2".into());}
//     if include_three_right {center_deque.push_back((total_number_of_pages - 1i64).to_string());}
//     if include_left_dots {center_deque.push_front("...".into());}
//     if include_right_dots {center_deque.push_back("...".into());}
//     center_deque.push_front("1".into());
//     center_deque.push_back(total_number_of_pages.to_string());
//     center_deque
// }

