import { invoke } from "@tauri-apps/api";
import { guard } from "../lib/utils/helpers";
import type { Result } from "../lib/types/global";

export const getBooks = async (): Promise<Result<Book[]>> => {
  return guard(async () => await invoke("get_books"));
};

export const addBook = async (book: BookPartial): Promise<Result> => {
  return guard(async () => await invoke("add_book", { book }));
};
