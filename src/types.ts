export interface Tag {
  id: number;
  name: string;
  color: string;
}

export interface Item {
  id: number;
  code: string;
  name: string;
  start_date: string;
  end_date: string | null;
  detail: string;
  jira: string | null;
  tags: Tag[];
  status: 'pending' | 'done';
}

export interface ItemInput {
  name: string;
  start_date: string;
  end_date: string | null;
  detail: string;
  jira: string | null;
  tags: number[];
}

export interface ItemFilter {
  status: string | null;
  range: string | null;
  start_date: string | null;
  end_date: string | null;
  tag_ids: number[] | null;
  page: number;
  page_size: number;
}

export interface PageResult {
  items: Item[];
  total: number;
}
