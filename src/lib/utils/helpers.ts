import type { Result } from "../types/global";

export const guard = async <T>(
  callback: () => Promise<T>,
  err?: string
): Promise<Result<T>> => {
  const result: Result<T> = {
    data: null,
    error: null,
  };

  try {
    result.data = await callback();
  } catch (error) {
    if (typeof error !== "string") return;
    result.error = err ?? error;
  }

  return result;
};
