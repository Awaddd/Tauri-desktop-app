import { invoke } from "@tauri-apps/api";
import { guard } from "../lib/utils/helpers";
import type { Result } from "../lib/types/global";

export const getBooks = async (): Promise<Result<Book[]>> => {
  return guard<Book[]>(async () => (await invoke("get_books")) as Book[]);
};

export const addBook = async (
  title: string,
  author: string
): Promise<Result> => {
  return guard(async () => await invoke("add_book", { title, author }));
};
