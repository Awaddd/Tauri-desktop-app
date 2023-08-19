import { invoke } from "@tauri-apps/api";
import { guard } from "../lib/utils/helpers";
import type { Result } from "../lib/types/global";

export const getBooks = async (): Promise<Result<string[]>> => {
  return guard<string[]>(async () => (await invoke("get_books")) as string[]);
};

export const addBook = async (book: string): Promise<Result> => {
  return guard(async () => await invoke("add_book", { book }));
};
