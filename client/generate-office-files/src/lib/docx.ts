import init, { generate_docx } from "docx_generator";

const ready = init();

export async function createDocx(
  title: string,
  body: string,
): Promise<Uint8Array> {
  await ready;
  return generate_docx(title, body);
}
