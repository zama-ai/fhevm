export type ApiKey = {
  created_at: number;
  id: string;
  key: string;
  tags: string[] | null;
  ttl: number | null;
};
