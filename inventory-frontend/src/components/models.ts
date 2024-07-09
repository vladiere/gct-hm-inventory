export interface Todo {
  id: number;
  content: string;
}

export interface Meta {
  totalCount: number;
}

export interface EssentialLinkPropsChild {
  title: string;
  caption?: string;
  link_name?: string;
  icon?: string;
}

export interface EssentialLinkProps {
  title: string;
  caption?: string;
  link_name?: string;
  icon?: string;
  expand: boolean;
  expand_link?: EssentialLinkPropsChild[];
}



