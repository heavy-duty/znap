export interface Action {
  label: string;
  href: string;
  parameters: {
    label: string;
    name: string;
  }[];
}

export interface Metadata {
  icon: string;
  title: string;
  description: string;
  label: string;
  disabled: boolean;
  error: null;
  links: { actions: Action[] } | null;
}

export function createActionClient(actionUrl: string) {
  return {
    async getMetadata() {
      const url = new URL(actionUrl);
      const response = await fetch(url.toString(), {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
        },
      });
      const responseJson = (await response.json()) as Metadata;

      return responseJson;
    },
    async getTransaction<T extends {}>(account: string, query: T) {
      const url = new URL(actionUrl);

      Object.keys(query).forEach((name) =>
        url.searchParams.set(name, query[name])
      );

      const response = await fetch(url.toString(), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ account }),
      });
      const responseJson = (await response.json()) as {
        transaction: string;
        message: string;
      };

      return responseJson;
    },
  };
}
