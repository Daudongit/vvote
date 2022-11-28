use crate::models::result::Results;
use jelly::actix_web::web::Path;
use jelly::prelude::*;
use jelly::Result;
use excel::*;

pub async fn index(request: HttpRequest, path: Path<(i32, i32)>) -> Result<HttpResponse> {
    let (election, position) = path.into_inner();
    let db = request.db_pool()?;
    let sheet_name = 
        Results::position_result_name((election, position), db).await?;
    let position_result = 
        Results::group_by_voter_election_position((election, position), db).await?;
    let position_result_bytes = 
        Results::position_result_bytes(position_result, sheet_name);
    Ok(HttpResponse::Ok()
    .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
    .body(position_result_bytes?))

}

pub async fn show(request: HttpRequest, path: Path<(i32, i32)>) -> Result<HttpResponse> {
    let (election_id, slot_id) = path.into_inner();
    let db = request.db_pool()?;
    let sheet_name = 
        Results::position_election_title((slot_id, election_id), db).await?;
    let nominees_result_count = 
        Results::nominees_with_result_count((slot_id, election_id), db).await?;
    let nominee_result_bytes = 
        Results::nominees_result_bytes(nominees_result_count, sheet_name);
    Ok(HttpResponse::Ok()
    .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
    .body(nominee_result_bytes?))
}


pub async fn generate()->Result<HttpResponse> {
    // let mut wb = Workbook::create("/tmp/b.xlsx");
    let mut wb = Workbook::create_in_memory();
    let mut sheet = wb.create_sheet("SheetName");

    // set column width
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 80.0 });
    sheet.add_column(Column { width: 60.0 });

    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["Name", "Title","Success","XML Remark"])?;
        sw.append_row(row!["Amy", (), true,"<xml><tag>\"Hello\" & 'World'</tag></xml>"])?;
        sw.append_blank_rows(2);
        sw.append_row(row!["Tony", blank!(2), "retired"])
    }).expect("write excel error!");

    let mut sheet = wb.create_sheet("Sheet2");
    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["Name", "Title","Success","Remark"])?;
        sw.append_row(row!["Amy", "Manager", true])
    }).expect("write excel error!");

    let excel_bytes = wb.close().expect("close excel error!");
    let excel_bytes = excel_bytes.unwrap_or_default();
    Ok(HttpResponse::Ok()
    .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
    .body(excel_bytes))
}
