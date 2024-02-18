export async function* filterEmpty(prev: AsyncGenerator<string>) {
  for await (const line of prev) {
    if (line !== "") {
      yield line;
    } else {
      continue;
    }
  }
}
