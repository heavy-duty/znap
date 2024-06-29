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
  links: { actions: Action[] };
}

export function createClient(baseUrl: string) {
  return {
    async getMetadata(actionName: string) {
      const url = new URL(`${baseUrl}/api/${actionName}`);
      const response = await fetch(url.toString(), {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
        },
      });
      const responseJson = (await response.json()) as Metadata;

      return responseJson;
    },
    async getTransaction<T extends {}>(
      actionName: string,
      account: string,
      params: T
    ) {
      const url = new URL(`${baseUrl}/api/${actionName}`);

      Object.keys(params).forEach((paramName) =>
        url.searchParams.set(paramName, params[paramName])
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
