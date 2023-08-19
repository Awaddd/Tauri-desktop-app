export interface Result<T = void> {
  data: T | null;
  error: string | null;
}
