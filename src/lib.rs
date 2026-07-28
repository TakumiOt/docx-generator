use docx_rs::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_docx(title: &str, body: &str) -> Result<Vec<u8>, JsValue> {
    let docx = Docx::new()
        // タイトル
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(title).size(48))
                .align(AlignmentType::Center),
        )
        .add_paragraph(Paragraph::new());

    let docx = body.lines().fold(docx, |d, line| {
        d.add_paragraph(Paragraph::new().add_run(Run::new().add_text(line)))
    });

    // メモリ上のバッファにビルド
    let mut buf = Vec::new();
    docx.build()
        .pack(&mut std::io::Cursor::new(&mut buf))
        .map_err(|e| JsValue::from_str(&format!("docx build error: {}", e)))?;

    Ok(buf)
}
